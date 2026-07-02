#!/bin/sh

RUSTC_BOOTSTRAP_VERSION=1.91.0
CARGO_BOOTSTRAP_VERSION=1.91.0
RUST_VERSION=1.92.0

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


prepatch() {
	if [ -d $BASE/patches ]; then
		for patch in $BASE/patches/patch-*; do
			echo $patch
			(cd $DEST/rustc-$RUST_VERSION-src && patch -V none < $patch) || exit 1
		done
	fi
}


fixup-vendor-patch() {
	local dir=$DEST/rustc-$RUST_VERSION-src/vendor/$1
	local file=$2

	echo "Regenerating checksum for ${dir}/${file}"

	test -d ${dir} || {
		echo "`${dir}` is no directory"
		exit 1
	}

	local new_checksum=$(${SHA256} -q "${dir}/${file}")
	local regex="-e s|\"${file}\":\"[0-9a-f]{64}\"|\"${file}\":\"${new_checksum}\"|"
	${REINPLACE_CMD} -E ${regex} ${dir}/.cargo-checksum.json || exit 1
}

fixup-vendor() {
	fixup-vendor-patch openssl-src-111.28.2+1.1.1w src/lib.rs || exit 1
	fixup-vendor-patch openssl-src-300.5.0+3.5.0 src/lib.rs || exit 1
	fixup-vendor-patch openssl-src-300.5.3+3.5.4 src/lib.rs || exit 1
	fixup-vendor-patch notify-8.2.0 Cargo.toml || exit 1
}

xbuild() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py build --verbose --config ./bootstrap.toml --jobs $(/sbin/sysctl -n hw.ncpu))
}

xdist() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py dist --verbose --config ./bootstrap.toml)
}

RUN info clean extract prepatch fixup-vendor config xbuild xdist inst 2>&1
