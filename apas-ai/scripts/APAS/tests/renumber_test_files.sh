#!/bin/bash

# Script to rename test files from N_Test* to TestN* format
# and renumber unnumbered files starting from max(n)+1

cd /home/milnes/APASVERUS/APAS-AI/apas-ai/tests

# First, rename the numbered files from N_Test* to TestN* format
git mv 01_TestTypes.rs Test01Types.rs
git mv 02_TestMathSeq.rs Test02MathSeq.rs
git mv 03_TestLinkedListStPer.rs Test03LinkedListStPer.rs
git mv 04_TestLinkedListStPerChap18.rs Test04LinkedListStPerChap18.rs
git mv 05_TestLinkedListStPerChap19.rs Test05LinkedListStPerChap19.rs
git mv 06_TestLinkedListStEph.rs Test06LinkedListStEph.rs
git mv 07_TestLinkedListStEphChap18.rs Test07LinkedListStEphChap18.rs
git mv 08_TestLinkedListStEphChap19.rs Test08LinkedListStEphChap19.rs
git mv 09_TestArraySeqStPer.rs Test09ArraySeqStPer.rs
git mv 10_TestArraySeqStPerChap18.rs Test10ArraySeqStPerChap18.rs
git mv 11_TestArraySeqStPerChap19.rs Test11ArraySeqStPerChap19.rs
git mv 12_TestArraySeqStEph.rs Test12ArraySeqStEph.rs
git mv 13_TestArraySeqStEphChap18.rs Test13ArraySeqStEphChap18.rs
git mv 14_TestArraySeqStEphChap19.rs Test14ArraySeqStEphChap19.rs
git mv 15_TestAVLTreeSeqStPer.rs Test15AVLTreeSeqStPer.rs
git mv 16_TestAVLTreeSeqStPerChap18.rs Test16AVLTreeSeqStPerChap18.rs
git mv 17_TestAVLTreeSeqStPerChap19.rs Test17AVLTreeSeqStPerChap19.rs
git mv 18_TestAVLTreeSeqStEph.rs Test18AVLTreeSeqStEph.rs
git mv 19_TestAVLTreeSeqStEphChap18.rs Test19AVLTreeSeqStEphChap18.rs
git mv 20_TestAVLTreeSeqStEphChap19.rs Test20AVLTreeSeqStEphChap19.rs
git mv 21_TestSetStEphChap5_1.rs Test21SetStEphChap5_1.rs
git mv 22_TestRelationStEphChap5_2.rs Test22RelationStEphChap5_2.rs
git mv 23_TestMappingStEphChap5_5.rs Test23MappingStEphChap5_5.rs
git mv 24_TestDirGraphStEphChap6_1.rs Test24DirGraphStEphChap6_1.rs
git mv 25_TestUnDirGraphStEphChap6_1.rs Test25UnDirGraphStEphChap6_1.rs
git mv 26_TestArraySeqMtPer.rs Test26ArraySeqMtPer.rs
git mv 27_TestArraySeqMtPerChap18.rs Test27ArraySeqMtPerChap18.rs
git mv 28_TestArraySeqMtPerChap19.rs Test28ArraySeqMtPerChap19.rs

# Now rename the unnumbered files starting from 29
git mv Algorithm_21_1.rs Test29Algorithm_21_1.rs
git mv Algorithm_21_2.rs Test30Algorithm_21_2.rs
git mv Algorithm_21_6.rs Test31Algorithm_21_6.rs
git mv Exercise_21_5_and_21_6.rs Test32Exercise_21_5_and_21_6.rs
git mv Exercise_21_7.rs Test33Exercise_21_7.rs
git mv Exercise_21_8_and_Algorithm_21_5.rs Test34Exercise_21_8_and_Algorithm_21_5.rs
git mv Exercsise_21_9.rs Test35Exercsise_21_9.rs
git mv Problem_21_3.rs Test36Problem_21_3.rs
git mv Problem_21_4.rs Test37Problem_21_4.rs
git mv Problem21_1.rs Test38Problem21_1.rs

echo "Test files have been renumbered!"
echo "Files 01-28 renamed from N_Test* to TestN* format"
echo "Unnumbered files renamed to Test29-Test38"
echo "Note: Exercise_21_2.txt was left unchanged as it's a text file"
