#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.34.2
CARGO_BOOTSTRAP_VERSION=0.35.0
RUST_VERSION=1.35.0

CONFIGURE_CARGO_STATIC_FLAGS="--enable-cargo-native-static"

# XXX: miri does not build correctly that's why we need the flag below.
ADDITIONAL_CONFIGURE_FLAGS="--enable-missing-tools"

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

fixup-vendor() {
        fixup-vendor-patch openssl-src src/lib.rs || exit 1
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
