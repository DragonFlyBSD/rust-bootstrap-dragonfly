#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.30.1
CARGO_BOOTSTRAP_VERSION=0.31.0
RUST_VERSION=1.31.0

CONFIGURE_CARGO_STATIC_FLAGS=--enable-cargo-native-static

# XXX: miri does not build correctly that's why we need the flag below.
ADDITIONAL_CONFIGURE_FLAGS=--enable-missing-tools

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

RUN info clean extract prepatch config xbuild xdist inst 2>&1
