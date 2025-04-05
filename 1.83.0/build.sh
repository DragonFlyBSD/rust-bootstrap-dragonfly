#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.82.0
CARGO_BOOTSTRAP_VERSION=1.82.0
RUST_VERSION=1.83.0

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
	# add here your vendor patches, example:
	#fixup-vendor-patch openssl build.rs || exit 1
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
