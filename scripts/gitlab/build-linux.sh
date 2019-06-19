#!/bin/bash

set -e # fail on any error
set -u # treat unset variables as error

echo "__________Show ENVIROMENT__________"
echo "CI_SERVER_NAME:   " $CI_SERVER_NAME
echo "CARGO_HOME:       " $CARGO_HOME
echo "CARGO_TARGET:     " $CARGO_TARGET
echo "CC:               " $CC
echo "CXX:              " $CXX
#strip ON
export RUSTFLAGS=" -C link-arg=-s"
# Linker for crosscomile
echo "_____ Linker _____"
cat .cargo/config

echo "_____ Building target: "$CARGO_TARGET" _____"
if [ "${CARGO_TARGET}" = "armv7-linux-androideabi" ]
then
  time cargo build --target $CARGO_TARGET --verbose --color=always --release -p susy-clib --features final
else
  time cargo build --target $CARGO_TARGET --verbose --color=always --release --features final
  time cargo build --target $CARGO_TARGET --verbose --color=always --release -p svmbin
  time cargo build --target $CARGO_TARGET --verbose --color=always --release -p sofstore-cli
  time cargo build --target $CARGO_TARGET --verbose --color=always --release -p sofkey-cli
  time cargo build --target $CARGO_TARGET --verbose --color=always --release -p whisper-cli
fi

echo "_____ Post-processing binaries _____"
rm -rf artifacts/*
mkdir -p artifacts/$CARGO_TARGET
cd artifacts/$CARGO_TARGET

if [ "${CARGO_TARGET}" = "armv7-linux-androideabi" ]
then
 cp -v ../../target/$CARGO_TARGET/release/libsusy.so ./libsusy.so
else
 cp -v ../../target/$CARGO_TARGET/release/susy ./susy
 cp -v ../../target/$CARGO_TARGET/release/susy-svm ./susy-svm
 cp -v ../../target/$CARGO_TARGET/release/sofstore ./sofstore
 cp -v ../../target/$CARGO_TARGET/release/sofkey ./sofkey
 cp -v ../../target/$CARGO_TARGET/release/whisper ./whisper
fi

echo "_____ Calculating checksums _____"
for binary in $(ls)
do
  rhash --sha256 $binary -o $binary.sha256 #do we still need this hash (SHA2)?
  if [[ $CARGO_TARGET == *"x86_64"* ]];
  then
      ./susy tools hash $binary > $binary.sha3
  else
      echo "> ${binary} cannot be hashed with cross-compiled binary (keccak256)"
  fi
done
