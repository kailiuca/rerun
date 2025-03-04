//! Set different types of indices.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_different_indices").spawn()?;

    rec.set_time_sequence("frame_nr", 42);
    rec.set_time_seconds("elapsed", 12.0);
    rec.set_time_seconds("time", 1_741_017_564); // Seconds since unix epoch
    rec.set_time_nanos("precise_time", 1_741_017_564_987_654_321_i64); // Nanos since unix epoch

    // All following logged data will be timestamped with the above times:
    rec.log("points", &rerun::Points2D::new([(0.0, 0.0), (1.0, 1.0)]))?;

    Ok(())
}
