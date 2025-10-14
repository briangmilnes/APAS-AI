#!/usr/bin/env bash

cd /home/milnes/APASVERUS/APAS-AI/apas-ai

# benches: LinkedList Per/Eph → StPer/StEph
git mv benches/BenchLinkedListPer.rs benches/BenchLinkedListStPer.rs
git mv benches/BenchLinkedListPerChap18.rs benches/BenchLinkedListStPerChap18.rs
git mv benches/BenchLinkedListPerChap19.rs benches/BenchLinkedListStPerChap19.rs
git mv benches/BenchLinkedListEph.rs benches/BenchLinkedListStEph.rs
git mv benches/BenchLinkedListEphChap18.rs benches/BenchLinkedListStEphChap18.rs
git mv benches/BenchLinkedListEphChap19.rs benches/BenchLinkedListStEphChap19.rs

# benches: ArraySeq Eph/Per → StEph/StPer
git mv benches/BenchArraySeqEph.rs benches/BenchArraySeqStEph.rs
git mv benches/BenchArraySeqEphChap18.rs benches/BenchArraySeqStEphChap18.rs
git mv benches/BenchArraySeqEphChap19.rs benches/BenchArraySeqStEphChap19.rs
git mv benches/BenchArraySeqPer.rs benches/BenchArraySeqStPer.rs
git mv benches/BenchArraySeqPerChap18.rs benches/BenchArraySeqStPerChap18.rs
git mv benches/BenchArraySeqPerChap19.rs benches/BenchArraySeqStPerChap19.rs

# benches: AVLTree Per/Eph → StPer/StEph
git mv benches/BenchAVLTreeSeqPer.rs benches/BenchAVLTreeSeqStPer.rs
git mv benches/BenchAVLTreeSeqPerChap18.rs benches/BenchAVLTreeSeqStPerChap18.rs
git mv benches/BenchAVLTreeSeqPerChap19.rs benches/BenchAVLTreeSeqStPerChap19.rs
git mv benches/BenchAVLTreeSeqEph.rs benches/BenchAVLTreeSeqStEph.rs
git mv benches/BenchAVLTreeSeqEphChap18.rs benches/BenchAVLTreeSeqStEphChap18.rs
git mv benches/BenchAVLTreeSeqEphChap19.rs benches/BenchAVLTreeSeqStEphChap19.rs

# benches: Graph Eph → StEph
git mv benches/BenchDirGraphEphChap6_1.rs benches/BenchDirGraphStEphChap6_1.rs
git mv benches/BenchUnDirGraphEphChap6_1.rs benches/BenchUnDirGraphStEphChap6_1.rs

# benches: Mapping/Relation Eph → StEph
git mv benches/BenchMappingEphChap5_5.rs benches/BenchMappingStEphChap5_5.rs
git mv benches/BenchRelationEphChap5_2.rs benches/BenchRelationStEphChap5_2.rs
