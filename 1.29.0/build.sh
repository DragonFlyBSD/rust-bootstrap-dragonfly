#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.28.0
CARGO_BOOTSTRAP_VERSION=0.29.0
RUST_VERSION=1.29.0

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

RUN info clean extract prepatch config RUN xbuild xdist inst 2>&1
