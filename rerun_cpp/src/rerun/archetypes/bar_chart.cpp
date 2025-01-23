// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/bar_chart.fbs".

#include "bar_chart.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    BarChart BarChart::clear_fields() {
        auto archetype = BarChart();
        archetype.values = ComponentBatch::empty<rerun::components::TensorData>(Descriptor_values)
                               .value_or_throw();
        archetype.color =
            ComponentBatch::empty<rerun::components::Color>(Descriptor_color).value_or_throw();
        return archetype;
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::BarChart>::serialize(
        const archetypes::BarChart& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.values.has_value()) {
            cells.push_back(archetype.values.value());
        }
        if (archetype.color.has_value()) {
            cells.push_back(archetype.color.value());
        }
        {
            auto indicator = BarChart::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
