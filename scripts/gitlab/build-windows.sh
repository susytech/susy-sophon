#!/bin/bash
set -e # fail on any error
set -u # treat unset variables as error

set INCLUDE="C:\Program Files (x86)\Microsoft SDKs\Windows\v7.1A\Include;C:\vs2015\VC\include;C:\Program Files (x86)\Windows Kits\10\Include\10.0.10240.0\ucrt"
set LIB="C:\vs2015\VC\lib;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10240.0\ucrt\x64"
sccache -s

echo "__________Show ENVIROMENT__________"
echo "CI_SERVER_NAME:   " $CI_SERVER_NAME
echo "CARGO_HOME:       " $CARGO_HOME
echo "CARGO_TARGET:     " $CARGO_TARGET
echo "RUSTC_WRAPPER:    " $RUSTC_WRAPPER
echo "SCCACHE_DIR:      " $SCCACHE_DIR

echo "_____ Building target: "$CARGO_TARGET" _____"
time cargo build --target $CARGO_TARGET --verbose --release --features final
time cargo build --target $CARGO_TARGET --verbose --release -p svmbin
time cargo build --target $CARGO_TARGET --verbose --release -p sofstore-cli
time cargo build --target $CARGO_TARGET --verbose --release -p sofkey-cli
time cargo build --target $CARGO_TARGET --verbose --release -p whisper-cli

echo "__________Sign binaries__________"
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/susy.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/susy-svm.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/sofstore.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/sofkey.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/whisper.exe

echo "_____ Post-processing binaries _____"
rm -rf artifacts
mkdir -p artifacts
cd artifacts
mkdir -p $CARGO_TARGET
cd $CARGO_TARGET
cp --verbose ../../target/$CARGO_TARGET/release/susy.exe ./susy.exe
cp --verbose ../../target/$CARGO_TARGET/release/susy-svm.exe ./susy-svm.exe
cp --verbose ../../target/$CARGO_TARGET/release/sofstore.exe ./sofstore.exe
cp --verbose ../../target/$CARGO_TARGET/release/sofkey.exe ./sofkey.exe
cp --verbose ../../target/$CARGO_TARGET/release/whisper.exe ./whisper.exe

echo "_____ Calculating checksums _____"
for binary in $(ls)
do
  rhash --sha256 $binary -o $binary.sha256
  ./susy.exe tools hash $binary > $binary.sha3
done
cp susy.exe.sha256 susy.sha256
cp susy.exe.sha3 susy.sha3

sccache -s
