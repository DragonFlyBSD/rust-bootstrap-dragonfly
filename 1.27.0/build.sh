#!/bin/sh

# Must be set to the directory where a recent rust snapshot can be found.
#BOOTSTRAP_DIR=

RUSTC_BOOTSTRAP_VERSION=1.26.0
CARGO_BOOTSTRAP_VERSION=0.27.0
RUST_VERSION=1.27.0

BASE=`pwd`
DEST=$1
LLVM_ROOT=""

. ../checksums.sh
. ../common.sh

RUN info clean extract prepatch config xbuild xdist inst 2>&1
