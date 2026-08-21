#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.87.0
CARGO_BOOTSTRAP_VERSION=1.87.0
RUST_VERSION=1.88.0

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
	fixup-vendor-patch openssl-src-111.17.0+1.1.1m src/lib.rs || exit 1
	fixup-vendor-patch openssl-src-300.4.2+3.4.1 src/lib.rs || exit 1
	fixup-vendor-patch notify-8.0.0 Cargo.toml || exit 1
}

xbuild() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py build --verbose --config ./bootstrap.toml --jobs $(/sbin/sysctl -n hw.ncpu))
}

xdist() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py dist --verbose --config ./bootstrap.toml)
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
