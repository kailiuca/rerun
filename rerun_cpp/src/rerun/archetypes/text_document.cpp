// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/text_document.fbs".

#include "text_document.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    TextDocument TextDocument::clear_fields() {
        auto archetype = TextDocument();
        archetype.text =
            ComponentBatch::empty<rerun::components::Text>(Descriptor_text).value_or_throw();
        archetype.media_type =
            ComponentBatch::empty<rerun::components::MediaType>(Descriptor_media_type)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> TextDocument::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(3);
        if (text.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(text.value(), lengths_).value_or_throw()
            );
        }
        if (media_type.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(media_type.value(), lengths_)
                                  .value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<TextDocument>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> TextDocument::columns() {
        if (text.has_value()) {
            return columns(std::vector<uint32_t>(text.value().length(), 1));
        }
        if (media_type.has_value()) {
            return columns(std::vector<uint32_t>(media_type.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::TextDocument>::serialize(
        const archetypes::TextDocument& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.text.has_value()) {
            cells.push_back(archetype.text.value());
        }
        if (archetype.media_type.has_value()) {
            cells.push_back(archetype.media_type.value());
        }
        {
            auto result = ComponentBatch::from_indicator<TextDocument>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
