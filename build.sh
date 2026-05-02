#!/bin/bash
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
echo $ROOT_DIR
C_LIB=$ROOT_DIR/c_lib
echo $C_LIB
RS_FFI=$ROOT_DIR/rs_ffi
echo $RS_FFI
LIB_NAME=$1
echo "building" $LIB_NAME
# check if it's there
if [ -f $C_LIB/$LIB_NAME/Makefile ]; then
	echo "running makefile"
	cd $C_LIB/$LIB_NAME
	make rebuild
	cd $ROOT_DIR
fi
if [ -f $RS_FFI/$LIB_NAME/Cargo.toml ]; then
	echo "building cargo"
	cd $RS_FFI/$LIB_NAME
	cargo build
	cd $ROOT_DIR
fi