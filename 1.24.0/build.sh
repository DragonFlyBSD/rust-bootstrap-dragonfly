#!/bin/sh

# Must be set to the directory where a recent rust snapshot can be found.
# BOOTSTRAP_DIR=

RUSTC_BOOTSTRAP_VERSION=1.23.0
CARGO_BOOTSTRAP_VERSION=0.24.0
RUST_VERSION=1.24.0

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

fixup-vendor() {
	fixup-vendor-patch curl-sys build.rs || exit 1
}

RUN info clean extract prepatch fixup-vendor config build dist inst 2>&1
