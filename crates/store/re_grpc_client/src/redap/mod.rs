use std::{collections::HashMap, sync::Arc};

use arrow::{
    array::{
        Array as ArrowArray, ArrayRef as ArrowArrayRef, RecordBatch as ArrowRecordBatch,
        StringArray as ArrowStringArray,
    },
    datatypes::{DataType as ArrowDataType, Field as ArrowField},
};
use url::Url;

use re_arrow_util::ArrowArrayDowncastRef as _;
use re_chunk::{Chunk, ChunkBuilder, ChunkId, EntityPath, RowId, Timeline, TransportChunk};
use re_log_encoding::codec::wire::decoder::Decode;
use re_log_types::{
    external::re_types_core::ComponentDescriptor, ApplicationId, BlueprintActivationCommand,
    EntityPathFilter, LogMsg, SetStoreInfo, StoreId, StoreInfo, StoreKind, StoreSource, Time,
};
use re_protos::{
    common::v0::RecordingId,
    remote_store::v0::{
        storage_node_client::StorageNodeClient, CatalogFilter, FetchRecordingRequest,
        QueryCatalogRequest, CATALOG_APP_ID_FIELD_NAME, CATALOG_ID_FIELD_NAME,
        CATALOG_START_TIME_FIELD_NAME,
    },
};
use re_types::{
    arrow_helpers::as_array_ref,
    blueprint::{
        archetypes::{ContainerBlueprint, ViewBlueprint, ViewContents, ViewportBlueprint},
        components::{ContainerKind, RootContainer},
    },
    components::RecordingUri,
    external::uuid,
    Archetype, Component,
};

// ----------------------------------------------------------------------------

mod address;

pub use address::{InvalidRedapAddress, RedapAddress};

use crate::spawn_future;
use crate::StreamError;
use crate::TonicStatusError;

// ----------------------------------------------------------------------------

const CATALOG_BP_STORE_ID: &str = "catalog_blueprint";
const CATALOG_REC_STORE_ID: &str = "catalog";
const CATALOG_APPLICATION_ID: &str = "redap_catalog";

/// Stream an rrd file or metadata catalog over gRPC from a Rerun Data Platform server.
///
/// `on_msg` can be used to wake up the UI thread on Wasm.
pub fn stream_from_redap(
    url: String,
    on_msg: Option<Box<dyn Fn() + Send + Sync>>,
) -> Result<re_smart_channel::Receiver<LogMsg>, InvalidRedapAddress> {
    re_log::debug!("Loading {url}…");

    let address = url.as_str().try_into()?;

    let (tx, rx) = re_smart_channel::smart_channel(
        re_smart_channel::SmartMessageSource::RerunGrpcStream { url: url.clone() },
        re_smart_channel::SmartChannelSource::RerunGrpcStream { url: url.clone() },
    );

    spawn_future(async move {
        match address {
            RedapAddress::Recording {
                redap_endpoint,
                recording_id,
            } => {
                if let Err(err) =
                    stream_recording_async(tx, redap_endpoint, recording_id, on_msg).await
                {
                    re_log::warn!(
                        "Error while streaming {url}: {}",
                        re_error::format_ref(&err)
                    );
                }
            }
            RedapAddress::Catalog { redap_endpoint } => {
                if let Err(err) = stream_catalog_async(tx, redap_endpoint, on_msg).await {
                    re_log::warn!(
                        "Error while streaming {url}: {}",
                        re_error::format_ref(&err)
                    );
                }
            }
        }
    });

    Ok(rx)
}

async fn stream_recording_async(
    tx: re_smart_channel::Sender<LogMsg>,
    redap_endpoint: Url,
    recording_id: String,
    on_msg: Option<Box<dyn Fn() + Send + Sync>>,
) -> Result<(), StreamError> {
    use tokio_stream::StreamExt as _;

    re_log::debug!("Connecting to {redap_endpoint}…");
    let mut client = {
        #[cfg(target_arch = "wasm32")]
        let tonic_client = tonic_web_wasm_client::Client::new_with_options(
            redap_endpoint.to_string(),
            tonic_web_wasm_client::options::FetchOptions::new(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        let tonic_client = tonic::transport::Endpoint::new(redap_endpoint.to_string())?
            .connect()
            .await?;

        // TODO(#8411): figure out the right size for this
        StorageNodeClient::new(tonic_client).max_decoding_message_size(usize::MAX)
    };

    re_log::debug!("Fetching catalog data for {recording_id}…");

    let resp = client
        .query_catalog(QueryCatalogRequest {
            column_projection: None, // fetch all columns
            filter: Some(CatalogFilter {
                recording_ids: vec![RecordingId {
                    id: recording_id.clone(),
                }],
            }),
        })
        .await
        .map_err(TonicStatusError)?
        .into_inner()
        .map(|resp| {
            resp.and_then(|r| {
                r.decode()
                    .map_err(|err| tonic::Status::internal(err.to_string()))
            })
        })
        .collect::<Result<Vec<_>, tonic::Status>>()
        .await
        .map_err(TonicStatusError)?;

    if resp.len() != 1 || resp[0].num_rows() != 1 {
        return Err(StreamError::ChunkError(re_chunk::ChunkError::Malformed {
            reason: format!(
                "expected exactly one recording with id {recording_id}, got {}",
                resp.len()
            ),
        }));
    }

    let store_info =
        store_info_from_catalog_chunk(&TransportChunk::from(resp[0].clone()), &recording_id)?;
    let store_id = store_info.store_id.clone();

    re_log::debug!("Fetching {recording_id}…");

    let mut resp = client
        .fetch_recording(FetchRecordingRequest {
            recording_id: Some(RecordingId {
                id: recording_id.clone(),
            }),
        })
        .await
        .map_err(TonicStatusError)?
        .into_inner()
        .map(|resp| {
            resp.and_then(|r| {
                r.decode()
                    .map_err(|err| tonic::Status::internal(err.to_string()))
            })
        });

    drop(client);

    // We need a whole StoreInfo here.
    if tx
        .send(LogMsg::SetStoreInfo(SetStoreInfo {
            row_id: *re_chunk::RowId::new(),
            info: store_info,
        }))
        .is_err()
    {
        re_log::debug!("Receiver disconnected");
        return Ok(());
    }

    re_log::info!("Starting to read...");
    while let Some(result) = resp.next().await {
        let batch = result.map_err(TonicStatusError)?;
        let chunk = Chunk::from_record_batch(batch)?;

        if tx
            .send(LogMsg::ArrowMsg(store_id.clone(), chunk.to_arrow_msg()?))
            .is_err()
        {
            re_log::debug!("Receiver disconnected");
            return Ok(());
        }

        if let Some(on_msg) = &on_msg {
            on_msg();
        }
    }

    Ok(())
}

pub fn store_info_from_catalog_chunk(
    tc: &TransportChunk,
    recording_id: &str,
) -> Result<StoreInfo, StreamError> {
    let store_id = StoreId::from_string(StoreKind::Recording, recording_id.to_owned());

    let (_field, data) = tc
        .components()
        .find(|(f, _)| f.name() == CATALOG_APP_ID_FIELD_NAME)
        .ok_or(StreamError::ChunkError(re_chunk::ChunkError::Malformed {
            reason: "no {CATALOG_APP_ID_FIELD_NAME} field found".to_owned(),
        }))?;
    let app_id = data
        .downcast_array_ref::<arrow::array::StringArray>()
        .ok_or(StreamError::ChunkError(re_chunk::ChunkError::Malformed {
            reason: format!(
                "{CATALOG_APP_ID_FIELD_NAME} must be a utf8 array: {:?}",
                tc.schema_ref()
            ),
        }))?
        .value(0);

    let (_field, data) = tc
        .components()
        .find(|(f, _)| f.name() == CATALOG_START_TIME_FIELD_NAME)
        .ok_or(StreamError::ChunkError(re_chunk::ChunkError::Malformed {
            reason: "no {CATALOG_START_TIME_FIELD_NAME}} field found".to_owned(),
        }))?;
    let start_time = data
        .downcast_array_ref::<arrow::array::TimestampNanosecondArray>()
        .ok_or(StreamError::ChunkError(re_chunk::ChunkError::Malformed {
            reason: format!(
                "{CATALOG_START_TIME_FIELD_NAME} must be a Timestamp array: {:?}",
                tc.schema_ref()
            ),
        }))?
        .value(0);

    Ok(StoreInfo {
        application_id: ApplicationId::from(app_id),
        store_id: store_id.clone(),
        cloned_from: None,
        is_official_example: false,
        started: Time::from_ns_since_epoch(start_time),
        store_source: StoreSource::Unknown,
        store_version: None,
    })
}

async fn stream_catalog_async(
    tx: re_smart_channel::Sender<LogMsg>,
    redap_endpoint: Url,
    on_msg: Option<Box<dyn Fn() + Send + Sync>>,
) -> Result<(), StreamError> {
    use tokio_stream::StreamExt as _;

    re_log::debug!("Connecting to {redap_endpoint}…");
    let mut client = {
        #[cfg(target_arch = "wasm32")]
        let tonic_client = tonic_web_wasm_client::Client::new_with_options(
            redap_endpoint.to_string(),
            tonic_web_wasm_client::options::FetchOptions::new(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        let tonic_client = tonic::transport::Endpoint::new(redap_endpoint.to_string())?
            .connect()
            .await?;

        StorageNodeClient::new(tonic_client)
    };

    re_log::debug!("Fetching catalog…");

    let mut resp = client
        .query_catalog(QueryCatalogRequest {
            column_projection: None, // fetch all columns
            filter: None,            // fetch all rows
        })
        .await
        .map_err(TonicStatusError)?
        .into_inner()
        .map(|resp| {
            resp.and_then(|r| {
                r.decode()
                    .map_err(|err| tonic::Status::internal(err.to_string()))
            })
        });

    drop(client);

    if activate_catalog_blueprint(&tx).is_err() {
        re_log::debug!("Failed to activate catalog blueprint");
        return Ok(());
    }

    // Craft the StoreInfo for the actual catalog data
    let store_id = StoreId::from_string(StoreKind::Recording, CATALOG_REC_STORE_ID.to_owned());

    let store_info = StoreInfo {
        application_id: ApplicationId::from(CATALOG_APPLICATION_ID),
        store_id: store_id.clone(),
        cloned_from: None,
        is_official_example: false,
        started: Time::now(),
        store_source: StoreSource::Unknown,
        store_version: None,
    };

    if tx
        .send(LogMsg::SetStoreInfo(SetStoreInfo {
            row_id: *re_chunk::RowId::new(),
            info: store_info,
        }))
        .is_err()
    {
        re_log::debug!("Receiver disconnected");
        return Ok(());
    }

    re_log::info!("Starting to read...");
    while let Some(result) = resp.next().await {
        let input = TransportChunk::from(result.map_err(TonicStatusError)?);

        // Catalog received from the ReDap server isn't suitable for direct conversion to a Rerun Chunk:
        // - conversion expects "data" columns to be ListArrays, hence we need to convert any individual row column data to ListArray
        // - conversion expects the input TransportChunk to have a ChunkId so we need to add that piece of metadata

        let mut fields: Vec<ArrowField> = Vec::new();
        let mut columns: Vec<ArrowArrayRef> = Vec::new();
        // add the (row id) control field
        let (row_id_field, row_id_data) = input.controls().next().ok_or(
            StreamError::ChunkError(re_chunk::ChunkError::Malformed {
                reason: "no control field found".to_owned(),
            }),
        )?;

        fields.push(
            ArrowField::new(
                RowId::name().to_string(), // need to rename to Rerun Chunk expected control field
                row_id_field.data_type().clone(),
                false, /* not nullable */
            )
            .with_metadata(TransportChunk::field_metadata_control_column()),
        );
        columns.push(row_id_data.clone());

        // next add any timeline field
        for (field, data) in input.timelines() {
            fields.push(field.clone());
            columns.push(data.clone());
        }

        // now add all the 'data' fields - we slice each column array into individual arrays and then convert the whole lot into a ListArray
        for (field, data) in input.components() {
            let data_field_inner =
                ArrowField::new("item", field.data_type().clone(), true /* nullable */);

            let data_field = ArrowField::new(
                field.name().clone(),
                ArrowDataType::List(Arc::new(data_field_inner.clone())),
                false, /* not nullable */
            )
            .with_metadata(TransportChunk::field_metadata_data_column());

            let mut sliced: Vec<ArrowArrayRef> = Vec::new();
            for idx in 0..data.len() {
                sliced.push(data.clone().slice(idx, 1));
            }

            let data_arrays = sliced.iter().map(|e| Some(e.as_ref())).collect::<Vec<_>>();
            #[allow(clippy::unwrap_used)] // we know we've given the right field type
            let data_field_array: arrow::array::ListArray =
                re_arrow_util::arrow_util::arrays_to_list_array(
                    data_field_inner.data_type().clone(),
                    &data_arrays,
                )
                .unwrap();

            fields.push(data_field);
            columns.push(as_array_ref(data_field_array));
        }

        let schema = {
            let metadata = HashMap::from_iter([
                (
                    TransportChunk::CHUNK_METADATA_KEY_ENTITY_PATH.to_owned(),
                    "catalog".to_owned(),
                ),
                (
                    TransportChunk::CHUNK_METADATA_KEY_ID.to_owned(),
                    ChunkId::new().to_string(),
                ),
            ]);
            arrow::datatypes::Schema::new_with_metadata(fields, metadata)
        };

        let record_batch = ArrowRecordBatch::try_new(schema.into(), columns)
            .map_err(re_chunk::ChunkError::from)?;
        let mut chunk = Chunk::from_record_batch(record_batch)?;

        // finally, enrich catalog data with RecordingUri that's based on the ReDap endpoint (that we know)
        // and the recording id (that we have in the catalog data)
        let host = redap_endpoint
            .host()
            .ok_or(StreamError::InvalidUri(format!(
                "couldn't get host from {redap_endpoint}"
            )))?;
        let port = redap_endpoint
            .port()
            .ok_or(StreamError::InvalidUri(format!(
                "couldn't get port from {redap_endpoint}"
            )))?;

        let recording_uri_arrays: Vec<ArrowArrayRef> = chunk
            .iter_slices::<String>(CATALOG_ID_FIELD_NAME.into())
            .map(|id| {
                let rec_id = &id[0]; // each component batch is of length 1 i.e. single 'id' value

                let recording_uri = format!("rerun://{host}:{port}/recording/{rec_id}");

                as_array_ref(ArrowStringArray::from(vec![recording_uri]))
            })
            .collect();

        let recording_id_arrays = recording_uri_arrays
            .iter()
            .map(|e| Some(e.as_ref()))
            .collect::<Vec<_>>();

        let rec_id_field = ArrowField::new("item", ArrowDataType::Utf8, true);
        #[allow(clippy::unwrap_used)] // we know we've given the right field type
        let uris = re_arrow_util::arrow_util::arrays_to_list_array(
            rec_id_field.data_type().clone(),
            &recording_id_arrays,
        )
        .unwrap();

        chunk.add_component(ComponentDescriptor::new(RecordingUri::name()), uris)?;

        if tx
            .send(LogMsg::ArrowMsg(store_id.clone(), chunk.to_arrow_msg()?))
            .is_err()
        {
            re_log::debug!("Receiver disconnected");
            return Ok(());
        }

        if let Some(on_msg) = &on_msg {
            on_msg();
        }
    }

    Ok(())
}

// Craft a blueprint from relevant chunks and activate it
// TODO(zehiko) - manual crafting of the blueprint as we have below will go away and be replaced
// by either a blueprint crafted using rust Blueprint API or a blueprint fetched from ReDap (#8470)
fn activate_catalog_blueprint(
    tx: &re_smart_channel::Sender<LogMsg>,
) -> Result<(), Box<dyn std::error::Error>> {
    let blueprint_store_id =
        StoreId::from_string(StoreKind::Blueprint, CATALOG_BP_STORE_ID.to_owned());
    let blueprint_store_info = StoreInfo {
        application_id: ApplicationId::from(CATALOG_APPLICATION_ID),
        store_id: blueprint_store_id.clone(),
        cloned_from: None,
        is_official_example: false,
        started: Time::now(),
        store_source: StoreSource::Unknown,
        store_version: None,
    };

    if tx
        .send(LogMsg::SetStoreInfo(SetStoreInfo {
            row_id: *re_chunk::RowId::new(),
            info: blueprint_store_info,
        }))
        .is_err()
    {
        re_log::debug!("Receiver disconnected");
        return Ok(());
    }

    let timepoint = [(Timeline::new_sequence("blueprint"), 1)];

    let vb = ViewBlueprint::new("Dataframe")
        .with_visible(true)
        .with_space_origin("/");

    // TODO(zehiko) we shouldn't really be creating all these ids and entity paths manually... (#8470)
    let view_uuid = uuid::Uuid::new_v4();
    let view_entity_path = format!("/view/{view_uuid}");
    let view_chunk = ChunkBuilder::new(ChunkId::new(), view_entity_path.clone().into())
        .with_archetype(RowId::new(), timepoint, &vb)
        .build()?;

    let epf = EntityPathFilter::parse_forgiving("/**");
    let vc = ViewContents::new(epf.iter_expressions());
    let view_contents_chunk = ChunkBuilder::new(
        ChunkId::new(),
        format!(
            "{}/{}",
            view_entity_path.clone(),
            ViewContents::name().short_name()
        )
        .into(),
    )
    .with_archetype(RowId::new(), timepoint, &vc)
    .build()?;

    let rc = ContainerBlueprint::new(ContainerKind::Grid)
        .with_contents(&[EntityPath::from(view_entity_path)])
        .with_visible(true);

    let container_uuid = uuid::Uuid::new_v4();
    let container_chunk = ChunkBuilder::new(
        ChunkId::new(),
        format!("/container/{container_uuid}").into(),
    )
    .with_archetype(RowId::new(), timepoint, &rc)
    .build()?;

    let vp = ViewportBlueprint::new().with_root_container(RootContainer(container_uuid.into()));
    let viewport_chunk = ChunkBuilder::new(ChunkId::new(), "/viewport".into())
        .with_archetype(RowId::new(), timepoint, &vp)
        .build()?;

    for chunk in &[
        view_chunk,
        view_contents_chunk,
        container_chunk,
        viewport_chunk,
    ] {
        if tx
            .send(LogMsg::ArrowMsg(
                blueprint_store_id.clone(),
                chunk.to_arrow_msg()?,
            ))
            .is_err()
        {
            re_log::debug!("Receiver disconnected");
            return Ok(());
        }
    }

    let blueprint_activation = BlueprintActivationCommand {
        blueprint_id: blueprint_store_id.clone(),
        make_active: true,
        make_default: true,
    };

    if tx
        .send(LogMsg::BlueprintActivationCommand(blueprint_activation))
        .is_err()
    {
        re_log::debug!("Receiver disconnected");
        return Ok(());
    }

    Ok(())
}
