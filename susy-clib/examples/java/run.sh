#!/usr/bin/env bash

FLAGS="-Xlint:deprecation"
SUSY_JAVA="../../Susy.java"
# susy-clib must be built with feature `jni` in debug-mode to work
SUSY_LIB=".:../../../target/debug/"

# build
cd ..
cargo build --features jni
cd -
javac $FLAGS -d $PWD $SUSY_JAVA
javac $FLAGS *.java
# Setup the path `libsusy.so` and run
java -Djava.library.path=$SUSY_LIB Main
