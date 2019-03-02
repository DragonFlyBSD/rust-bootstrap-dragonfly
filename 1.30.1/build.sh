#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.29.0
CARGO_BOOTSTRAP_VERSION=0.30.0
RUST_VERSION=1.30.1

BASE=`pwd`
DEST=$1
LLVM_ROOT="/usr/local/llvm60"
ADDITIONAL_CONFIGURE_FLAGS="--disable-jemalloc"

. ../checksums.sh
. ../common.sh

RUN info clean extract prepatch config RUN xbuild xdist inst 2>&1
