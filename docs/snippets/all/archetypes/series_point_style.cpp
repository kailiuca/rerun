// Log a scalar over time.

#include <rerun.hpp>

#include <cmath>

constexpr float TAU = 6.28318530717958647692528676655900577f;

int main() {
    const auto rec = rerun::RecordingStream("rerun_example_series_point_style");
    rec.spawn().exit_on_failure();

    // Set up plot styling:
    // They are logged static as they don't change over time and apply to all timelines.
    // Log two point series under a shared root so that they show in the same plot by default.
    rec.log_static(
        "trig/sin",
        rerun::SeriesPoint()
            .with_color({255, 0, 0})
            .with_name("sin(0.01t)")
            .with_marker(rerun::components::MarkerShape::Circle)
            .with_marker_size(4)
    );
    rec.log_static(
        "trig/cos",
        rerun::SeriesPoint()
            .with_color({0, 255, 0})
            .with_name("cos(0.01t)")
            .with_marker(rerun::components::MarkerShape::Cross)
            .with_marker_size(2)
    );

    // Log the data on a timeline called "step".
    for (int t = 0; t < static_cast<int>(TAU * 2.0 * 10.0); ++t) {
        rec.set_index_sequence("step", t);

        rec.log("trig/sin", rerun::Scalar(sin(static_cast<double>(t) / 10.0)));
        rec.log("trig/cos", rerun::Scalar(cos(static_cast<double>(t) / 10.0)));
    }
}
