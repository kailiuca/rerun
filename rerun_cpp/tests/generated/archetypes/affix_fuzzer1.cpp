// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#include "affix_fuzzer1.hpp"

#include <rerun/collection_adapter_builtins.hpp>

namespace rerun::archetypes {
    AffixFuzzer1 AffixFuzzer1::clear_fields() {
        auto archetype = AffixFuzzer1();
        archetype.fuzz1001 =
            ComponentBatch::empty<rerun::components::AffixFuzzer1>(Descriptor_fuzz1001)
                .value_or_throw();
        archetype.fuzz1002 =
            ComponentBatch::empty<rerun::components::AffixFuzzer2>(Descriptor_fuzz1002)
                .value_or_throw();
        archetype.fuzz1003 =
            ComponentBatch::empty<rerun::components::AffixFuzzer3>(Descriptor_fuzz1003)
                .value_or_throw();
        archetype.fuzz1004 =
            ComponentBatch::empty<rerun::components::AffixFuzzer4>(Descriptor_fuzz1004)
                .value_or_throw();
        archetype.fuzz1005 =
            ComponentBatch::empty<rerun::components::AffixFuzzer5>(Descriptor_fuzz1005)
                .value_or_throw();
        archetype.fuzz1006 =
            ComponentBatch::empty<rerun::components::AffixFuzzer6>(Descriptor_fuzz1006)
                .value_or_throw();
        archetype.fuzz1007 =
            ComponentBatch::empty<rerun::components::AffixFuzzer7>(Descriptor_fuzz1007)
                .value_or_throw();
        archetype.fuzz1008 =
            ComponentBatch::empty<rerun::components::AffixFuzzer8>(Descriptor_fuzz1008)
                .value_or_throw();
        archetype.fuzz1009 =
            ComponentBatch::empty<rerun::components::AffixFuzzer9>(Descriptor_fuzz1009)
                .value_or_throw();
        archetype.fuzz1010 =
            ComponentBatch::empty<rerun::components::AffixFuzzer10>(Descriptor_fuzz1010)
                .value_or_throw();
        archetype.fuzz1011 =
            ComponentBatch::empty<rerun::components::AffixFuzzer11>(Descriptor_fuzz1011)
                .value_or_throw();
        archetype.fuzz1012 =
            ComponentBatch::empty<rerun::components::AffixFuzzer12>(Descriptor_fuzz1012)
                .value_or_throw();
        archetype.fuzz1013 =
            ComponentBatch::empty<rerun::components::AffixFuzzer13>(Descriptor_fuzz1013)
                .value_or_throw();
        archetype.fuzz1014 =
            ComponentBatch::empty<rerun::components::AffixFuzzer14>(Descriptor_fuzz1014)
                .value_or_throw();
        archetype.fuzz1015 =
            ComponentBatch::empty<rerun::components::AffixFuzzer15>(Descriptor_fuzz1015)
                .value_or_throw();
        archetype.fuzz1016 =
            ComponentBatch::empty<rerun::components::AffixFuzzer16>(Descriptor_fuzz1016)
                .value_or_throw();
        archetype.fuzz1017 =
            ComponentBatch::empty<rerun::components::AffixFuzzer17>(Descriptor_fuzz1017)
                .value_or_throw();
        archetype.fuzz1018 =
            ComponentBatch::empty<rerun::components::AffixFuzzer18>(Descriptor_fuzz1018)
                .value_or_throw();
        archetype.fuzz1019 =
            ComponentBatch::empty<rerun::components::AffixFuzzer19>(Descriptor_fuzz1019)
                .value_or_throw();
        archetype.fuzz1020 =
            ComponentBatch::empty<rerun::components::AffixFuzzer20>(Descriptor_fuzz1020)
                .value_or_throw();
        archetype.fuzz1021 =
            ComponentBatch::empty<rerun::components::AffixFuzzer21>(Descriptor_fuzz1021)
                .value_or_throw();
        archetype.fuzz1022 =
            ComponentBatch::empty<rerun::components::AffixFuzzer22>(Descriptor_fuzz1022)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> AffixFuzzer1::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(23);
        if (fuzz1001.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1001.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1002.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1002.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1003.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1003.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1004.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1004.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1005.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1005.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1006.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1006.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1007.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1007.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1008.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1008.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1009.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1009.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1010.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1010.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1011.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1011.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1012.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1012.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1013.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1013.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1014.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1014.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1015.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1015.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1016.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1016.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1017.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1017.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1018.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1018.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1019.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1019.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1020.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1020.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1021.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1021.value(), lengths_)
                                  .value_or_throw());
        }
        if (fuzz1022.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fuzz1022.value(), lengths_)
                                  .value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<AffixFuzzer1>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> AffixFuzzer1::columns() {
        if (fuzz1001.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1001.value().length(), 1));
        }
        if (fuzz1002.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1002.value().length(), 1));
        }
        if (fuzz1003.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1003.value().length(), 1));
        }
        if (fuzz1004.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1004.value().length(), 1));
        }
        if (fuzz1005.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1005.value().length(), 1));
        }
        if (fuzz1006.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1006.value().length(), 1));
        }
        if (fuzz1007.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1007.value().length(), 1));
        }
        if (fuzz1008.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1008.value().length(), 1));
        }
        if (fuzz1009.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1009.value().length(), 1));
        }
        if (fuzz1010.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1010.value().length(), 1));
        }
        if (fuzz1011.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1011.value().length(), 1));
        }
        if (fuzz1012.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1012.value().length(), 1));
        }
        if (fuzz1013.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1013.value().length(), 1));
        }
        if (fuzz1014.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1014.value().length(), 1));
        }
        if (fuzz1015.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1015.value().length(), 1));
        }
        if (fuzz1016.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1016.value().length(), 1));
        }
        if (fuzz1017.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1017.value().length(), 1));
        }
        if (fuzz1018.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1018.value().length(), 1));
        }
        if (fuzz1019.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1019.value().length(), 1));
        }
        if (fuzz1020.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1020.value().length(), 1));
        }
        if (fuzz1021.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1021.value().length(), 1));
        }
        if (fuzz1022.has_value()) {
            return columns(std::vector<uint32_t>(fuzz1022.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::AffixFuzzer1>::serialize(
        const archetypes::AffixFuzzer1& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(23);

        if (archetype.fuzz1001.has_value()) {
            cells.push_back(archetype.fuzz1001.value());
        }
        if (archetype.fuzz1002.has_value()) {
            cells.push_back(archetype.fuzz1002.value());
        }
        if (archetype.fuzz1003.has_value()) {
            cells.push_back(archetype.fuzz1003.value());
        }
        if (archetype.fuzz1004.has_value()) {
            cells.push_back(archetype.fuzz1004.value());
        }
        if (archetype.fuzz1005.has_value()) {
            cells.push_back(archetype.fuzz1005.value());
        }
        if (archetype.fuzz1006.has_value()) {
            cells.push_back(archetype.fuzz1006.value());
        }
        if (archetype.fuzz1007.has_value()) {
            cells.push_back(archetype.fuzz1007.value());
        }
        if (archetype.fuzz1008.has_value()) {
            cells.push_back(archetype.fuzz1008.value());
        }
        if (archetype.fuzz1009.has_value()) {
            cells.push_back(archetype.fuzz1009.value());
        }
        if (archetype.fuzz1010.has_value()) {
            cells.push_back(archetype.fuzz1010.value());
        }
        if (archetype.fuzz1011.has_value()) {
            cells.push_back(archetype.fuzz1011.value());
        }
        if (archetype.fuzz1012.has_value()) {
            cells.push_back(archetype.fuzz1012.value());
        }
        if (archetype.fuzz1013.has_value()) {
            cells.push_back(archetype.fuzz1013.value());
        }
        if (archetype.fuzz1014.has_value()) {
            cells.push_back(archetype.fuzz1014.value());
        }
        if (archetype.fuzz1015.has_value()) {
            cells.push_back(archetype.fuzz1015.value());
        }
        if (archetype.fuzz1016.has_value()) {
            cells.push_back(archetype.fuzz1016.value());
        }
        if (archetype.fuzz1017.has_value()) {
            cells.push_back(archetype.fuzz1017.value());
        }
        if (archetype.fuzz1018.has_value()) {
            cells.push_back(archetype.fuzz1018.value());
        }
        if (archetype.fuzz1019.has_value()) {
            cells.push_back(archetype.fuzz1019.value());
        }
        if (archetype.fuzz1020.has_value()) {
            cells.push_back(archetype.fuzz1020.value());
        }
        if (archetype.fuzz1021.has_value()) {
            cells.push_back(archetype.fuzz1021.value());
        }
        if (archetype.fuzz1022.has_value()) {
            cells.push_back(archetype.fuzz1022.value());
        }
        {
            auto result = ComponentBatch::from_indicator<AffixFuzzer1>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
