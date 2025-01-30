// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/clear.fbs".

#include "clear.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Clear Clear::clear_fields() {
        auto archetype = Clear();
        archetype.is_recursive =
            ComponentBatch::empty<rerun::components::ClearIsRecursive>(Descriptor_is_recursive)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> Clear::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(2);
        if (is_recursive.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(is_recursive.value(), lengths_)
                    .value_or_throw()
            );
        }
        columns.push_back(
            ComponentColumn::from_indicators<Clear>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> Clear::columns() {
        if (is_recursive.has_value()) {
            return columns(std::vector<uint32_t>(is_recursive.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Clear>::serialize(
        const archetypes::Clear& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(2);

        if (archetype.is_recursive.has_value()) {
            cells.push_back(archetype.is_recursive.value());
        }
        {
            auto result = ComponentBatch::from_indicator<Clear>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
