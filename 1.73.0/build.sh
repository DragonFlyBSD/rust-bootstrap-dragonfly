#!/bin/sh

set -e

RUSTC_BOOTSTRAP_VERSION=1.72.1
CARGO_BOOTSTRAP_VERSION=1.72.1
RUST_VERSION=1.73.0

CONFIGURE_CARGO_STATIC_FLAGS="--enable-cargo-native-static"

# XXX: miri does not build correctly that's why we need the flag below.
ADDITIONAL_CONFIGURE_FLAGS="--enable-missing-tools"

# Since rust 1.38, OPENSSL_DIR has to be specified.
export OPENSSL_DIR="/usr/local"

# Show backtraces on failures
export RUST_BACKTRACE=1

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

FIXUP_VENDOR_1=openssl:build.rs
FIXUP_VENDOR_2=openssl-sys:build/cfgs.rs
FIXUP_VENDOR_3=libc:src/unix/bsd/freebsdlike/dragonfly/mod.rs

. ../checksums.sh
. ../common.sh

RUN-ALL 2>&1
