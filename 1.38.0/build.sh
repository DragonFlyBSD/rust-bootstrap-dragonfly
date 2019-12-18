#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.37.0
CARGO_BOOTSTRAP_VERSION=0.38.0
RUST_VERSION=1.38.0

CONFIGURE_CARGO_STATIC_FLAGS="--enable-cargo-native-static"

# XXX: miri does not build correctly that's why we need the flag below.
ADDITIONAL_CONFIGURE_FLAGS="--enable-missing-tools"

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

fixup-vendor() {
	fixup-vendor-patch openssl src/version.rs || exit 1
	fixup-vendor-patch openssl src/ssl/mod.rs || exit 1
        fixup-vendor-patch openssl build.rs || exit 1
        fixup-vendor-patch openssl-sys build/main.rs || exit 1
        fixup-vendor-patch openssl-sys build/cfgs.rs || exit 1
        fixup-vendor-patch openssl-sys src/ssl.rs || exit 1
        fixup-vendor-patch openssl-sys src/crypto.rs || exit 1
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
