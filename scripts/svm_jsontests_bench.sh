#!/usr/bin/env bash

cargo build --release -p svmbin

./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmArithmeticTest
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmBitwiseLogicOperation
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmBlockInfoTest
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmEnvironmentalInfo
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmIOandFlowOperations
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmLogTest
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmPerformance
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmPushDupSwapTest
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmRandomTest
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmSha3Test
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmSystemOperations
./target/release/susy-svm stats-jsontests-vm ./sofcore/res/sophon/tests/VMTests/vmTests
