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

    Collection<ComponentColumn> BarChart::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(3);
        if (values.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(values.value(), lengths_).value_or_throw()
            );
        }
        if (color.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(color.value(), lengths_).value_or_throw()
            );
        }
        columns.push_back(
            ComponentColumn::from_indicators<BarChart>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> BarChart::columns() {
        if (values.has_value()) {
            return columns(std::vector<uint32_t>(values.value().length(), 1));
        }
        if (color.has_value()) {
            return columns(std::vector<uint32_t>(color.value().length(), 1));
        }
        return Collection<ComponentColumn>();
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
