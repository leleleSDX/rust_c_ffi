#!/bin/bash
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
#echo $ROOT_DIR
C_LIB=$ROOT_DIR/c_lib
#echo $C_LIB
RS_FFI=$ROOT_DIR/rs_ffi
#echo $RS_FFI
EXEC_NAME=$1
if [ -f $RS_FFI/$EXEC_NAME/Cargo.toml ]; then
	echo "running" $EXEC_NAME
	cd $RS_FFI/$LIB_NAME
	cargo run
	cd $ROOT_DIR
fi