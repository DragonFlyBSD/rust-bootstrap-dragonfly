#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.27.0
CARGO_BOOTSTRAP_VERSION=0.28.0
RUST_VERSION=1.28.0

BASE=`pwd`
DEST=$1
LLVM_ROOT=""
ADDITIONAL_CONFIGURE_FLAGS="--disable-jemalloc"

. ../checksums.sh
. ../common.sh

RUN info clean extract prepatch config xbuild xdist inst 2>&1
