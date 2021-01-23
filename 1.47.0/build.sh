#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.46.0
CARGO_BOOTSTRAP_VERSION=0.47.0
RUST_VERSION=1.47.0

CONFIGURE_CARGO_STATIC_FLAGS="--enable-cargo-native-static"

# XXX: miri does not build correctly that's why we need the flag below.
ADDITIONAL_CONFIGURE_FLAGS="--enable-missing-tools"

# Since rust 1.38, OPENSSL_DIR has to be specified.
export OPENSSL_DIR="/usr/local"

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
	fixup-vendor-patch libc src/unix/bsd/freebsdlike/dragonfly/mod.rs || exit 1
	fixup-vendor-patch libc src/lib.rs || exit 1
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
