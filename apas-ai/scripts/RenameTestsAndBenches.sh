#!/usr/bin/env bash

cd /home/milnes/APASVERUS/APAS-AI/apas-ai

# tests: LinkedList Per/Eph → StPer/StEph
git mv tests/03_TestLinkedListPer.rs tests/03_TestLinkedListStPer.rs
git mv tests/04_TestLinkedListPerChap18.rs tests/04_TestLinkedListStPerChap18.rs
git mv tests/05_TestLinkedListPerChap19.rs tests/05_TestLinkedListStPerChap19.rs
git mv tests/06_TestLinkedListEph.rs tests/06_TestLinkedListStEph.rs
git mv tests/07_TestLinkedListEphChap18.rs tests/07_TestLinkedListStEphChap18.rs
git mv tests/08_TestLinkedListEphChap19.rs tests/08_TestLinkedListStEphChap19.rs

# tests: ArraySeq Eph → StEph
git mv tests/12_TestArraySeqEph.rs tests/12_TestArraySeqStEph.rs
git mv tests/13_TestArraySeqEphChap18.rs tests/13_TestArraySeqStEphChap18.rs
git mv tests/14_TestArraySeqEphChap19.rs tests/14_TestArraySeqStEphChap19.rs

# tests: AVLTree Per/Eph → StPer/StEph
git mv tests/15_TestAVLTreeSeqPer.rs tests/15_TestAVLTreeSeqStPer.rs
git mv tests/16_TestAVLTreeSeqPerChap18.rs tests/16_TestAVLTreeSeqStPerChap18.rs
git mv tests/17_TestAVLTreeSeqPerChap19.rs tests/17_TestAVLTreeSeqStPerChap19.rs
git mv tests/18_TestAVLTreeSeqEph.rs tests/18_TestAVLTreeSeqStEph.rs
git mv tests/19_TestAVLTreeSeqEphChap18.rs tests/19_TestAVLTreeSeqStEphChap18.rs
git mv tests/20_TestAVLTreeSeqEphChap19.rs tests/20_TestAVLTreeSeqStEphChap19.rs

# tests: Graph Eph → StEph
git mv tests/24_TestDirGraphEphChap6_1.rs tests/24_TestDirGraphStEphChap6_1.rs
git mv tests/25_TestUnDirGraphEphChap6_1.rs tests/25_TestUnDirGraphStEphChap6_1.rs

# tests: Set/Relation Eph → StEph
git mv tests/21_TestSetEphChap5_1.rs tests/21_TestSetStEphChap5_1.rs
git mv tests/22_TestRelationEphChap5_2.rs tests/22_TestRelationStEphChap5_2.rs
