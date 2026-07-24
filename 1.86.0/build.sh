#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.85.1
CARGO_BOOTSTRAP_VERSION=1.85.1
RUST_VERSION=1.86.0

CONFIGURE_CARGO_STATIC_FLAGS="--enable-cargo-native-static"

# Since rust 1.38, OPENSSL_DIR has to be specified.
export OPENSSL_DIR="/usr/local"

# Show backtraces on failures
export RUST_BACKTRACE=1
# Continue linking rustc driver dynamically
export RUSTC_LINK_STD_INTO_RUSTC_DRIVER=0

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

fixup-vendor() {
	fixup-vendor-patch openssl-src-111.28.2+1.1.1w src/lib.rs || exit 1
	fixup-vendor-patch notify-8.0.0 Cargo.toml || exit 1
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
