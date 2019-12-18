
# Expected variables:
#
# * RUSTC_BOOTSTRAP_VERSION
# * CARGO_BOOTSTRAP_VERSION
# * RUST_VERSION
#
# * BASE
# * DEST
# * RELEASE_CHANNEL (defaults to "stable")
# * LLVM_ROOT (defaults to "")
#
# Optional:
#
#   RUST_DIST_SERVER
#   ADDITIONAL_CONFIGURE_FLAGS

if [ "${RUST_DIST_SERVER}" = "" ]; then
	RUST_DIST_SERVER=https://static.rust-lang.org
else
	echo "Using non-default RUST_DIST_SERVER=${RUST_DIST_SERVER}"
fi

if [ "$RUSTC_BOOTSTRAP_VERSION" = "" ]; then
	echo "No RUSTC_BOOTSTRAP_VERSION"
	exit 1
fi

if [ "$CARGO_BOOTSTRAP_VERSION" = "" ]; then
	echo "No CARGO_BOOTSTRAP_VERSION"
	exit 1
fi

if [ "$RUST_VERSION" = "" ]; then
	echo "No RUST_VERSION"
	exit 1
fi

if [ "$BASE" = "" ]; then
	echo "No BASE"
	exit 1
fi

if [ "$DEST" = "" ]; then
	echo "Usage: $0 target-dir"
	exit 1
fi

if [ "$RELEASE_CHANNEL" = "" ]; then
	RELEASE_CHANNEL=stable
fi

if [ "$RELEASE_CHANNEL" = "stable" ]; then
elif [ "$RELEASE_CHANNEL" = "dev" ]; then
else
	echo "Invalid RELEASE_CHANNEL: either stable or dev"
	exit 2
fi

if [ "$LLVM_ROOT" = "" ]; then
	echo "Use in-tree LLVM"
	LLVM_ROOT_OPT=""
else
	echo "Use LLVM tree: $LLVM_ROOT"
	LLVM_ROOT_OPT="--llvm-root=${LLVM_ROOT}"
fi

BASH=bash
TARGET=x86_64-unknown-dragonfly
DEST_INSTALL=$DEST/install
COMPONENTS="cargo-${CARGO_BOOTSTRAP_VERSION} rust-std-${RUSTC_BOOTSTRAP_VERSION} rustc-${RUSTC_BOOTSTRAP_VERSION}"
BOOTSTRAP_URL="https://leaf.dragonflybsd.org/~tuxillo/archive/rust/${RUSTC_BOOTSTRAP_VERSION}"

if [ "${CONFIGURE_CARGO_STATIC_FLAGS}" = "" ]; then
	CONFIGURE_CARGO_STATIC_FLAGS=--enable-cargo-openssl-static
fi

# set path
export PATH=/bin:/usr/bin:/usr/local/bin

# we need these to avoid patching the sources for sha256sum
export PATH=$PATH:$BASE/../bin

mkdir -p $DEST/bootstrap

BOOTSTRAP_COMPILER_BASE=`realpath $DEST/bootstrap`

if [ "x${BOOTSTRAP_COMPILER_BASE}" = "x" ]; then
	echo "No bootstrap compiler found"
	exit 1
fi

export LD_LIBRARY_PATH="${BOOTSTRAP_COMPILER_BASE}/lib:$LD_LIBRARY_PATH"

# for static cargo
export LIBSSH2_NO_PKG_CONFIG=1
export LIBCURL_NO_PKG_CONFIG=1
export LIBZ_NO_PKG_CONFIG=1 
export PROFILE=release
export LIBZ_SYS_STATIC=1
export OPENSSL_NO_PKG_CONFIG=1

clean() {
	rm -rf $DEST
}

# filename cachepath downloadpath url
download() {
	file=`echo $1 | tr  ".-" "_"`
	eval "expected_cksum=\$SHA256_${file}"
	if [ "${expected_cksum}" = "" ]; then
		echo "no sha256 checksum for $2/$1. aborting"
		exit 1
	fi

	if [ -f $2/$1 ]; then
		echo "$1 exists in $2"
		cp $2/$1 $3/$1
	else
		echo "download: $1 from $4 and store into $3"
		fetch -o $3/$1 $4/$1
	fi

	cksum=`sha256 -q $3/$1`

	if [ "${cksum}" = "${expected_cksum}" ]; then
		echo "checksum ok for $1"
	else
		echo "checksum mismatch. expected: ${expected_cksum}. given: ${cksum}"
		exit 1
	fi
}

SHA256=sha256
REINPLACE_CMD="sed -i.bak"

fixup-vendor-patch() {
	cd $DEST/rustc-$RUST_VERSION-src/vendor/$1
	file=$2
	old_checksum=`${SHA256} -q "${file}.orig"`
	new_checksum=`${SHA256} -q "${file}"`
	regex="-e s|\"${file}\":\"${old_checksum}\"|\"${file}\":\"${new_checksum}\"|"; \
	${REINPLACE_CMD} -E ${regex} .cargo-checksum.json || exit 1
}

extract() {
	mkdir -p $DEST/tmp $DEST/bootstrap ${DEST_INSTALL}

	download rustc-$RUST_VERSION-src.tar.gz /usr/distfiles $DEST ${RUST_DIST_SERVER}/dist || exit 1
	tar xvzf $DEST/rustc-$RUST_VERSION-src.tar.gz -C ${DEST} 2>&1 | wc -l

	for component in ${COMPONENTS}; do
		echo "DOWNLOAD COMPONENT: ${component}"
		download $component-${TARGET}.tar.xz /usr/distfiles $DEST ${BOOTSTRAP_URL}

		echo "INSTALL COMPONENT: ${component}"
		tar xvzf $DEST/$component-${TARGET}.tar.xz -C $DEST/tmp  || exit 1
		# install.sh needs bash, but used !/bin/bash which does not exist on DragonFly
		${BASH} $DEST/tmp/$component-${TARGET}/install.sh --prefix=$DEST/bootstrap
	done
}

prepatch() {
	cd $DEST/rustc-$RUST_VERSION-src
	if [ -d $BASE/patches ]; then
		for patch in $BASE/patches/patch-*; do
			echo $patch
			patch < $patch || exit 1
		done
	fi
}

create-config() {
	cat $BASE/config.toml.template | \
		sed -e "s:%%CARGO%%:${BOOTSTRAP_COMPILER_BASE}/bin/cargo:g" | \
		sed -e "s:%%RUSTC%%:${BOOTSTRAP_COMPILER_BASE}/bin/rustc:g" | \
		sed -e "s:%%PREFIX%%:${DEST_INSTALL}:g" | \
		sed -e "s:%%CC%%:${CONF_CC}:g" | \
		sed -e "s:%%CXX%%:${CONF_CXX}:g" | \
		sed -e "s:%%LINKER%%:${CONF_LINKER}:g" | \
		sed -e "s:%%LLVM_CONFIG%%:${LLVM_ROOT}/bin/llvm-config:g" > \
		$DEST/rustc-$RUST_VERSION-src/config.toml
}

config() {
	cd $DEST/rustc-$RUST_VERSION-src
	./configure \
		--release-channel=${RELEASE_CHANNEL} \
		${CONFIGURE_CARGO_STATIC_FLAGS} --enable-extended \
		--enable-vendor \
		${ADDITIONAL_CONFIGURE_FLAGS} \
		--enable-locked-deps \
		--local-rust-root=${BOOTSTRAP_COMPILER_BASE} \
		--sysconfdir=${DEST_INSTALL}/etc \
		--prefix=${DEST_INSTALL} \
		--python=/usr/local/bin/python3.6 \
		${LLVM_ROOT_OPT}
}

mk() {
	cd $DEST/rustc-$RUST_VERSION-src && gmake VERBOSE=YES $1
}

build() {
	mk all
}

dist() {
	mk dist
}

inst() {
	mk install
}

xbuild() {
	cd $DEST/rustc-$RUST_VERSION-src && python3.6 x.py build --verbose --config ./config.toml --jobs $(/sbin/sysctl -n hw.ncpu)
}

xdist() {
	cd $DEST/rustc-$RUST_VERSION-src && python3.6 x.py dist --verbose --config ./config.toml
}

xinst() {
	cd $DEST/rustc-$RUST_VERSION-src && python3.6 x.py install --verbose --config ./config.toml
}

info() {
	echo Base: $BASE
	echo Release-Channel: $RELEASE_CHANNEL
	echo Dest: $DEST
	echo BOOTSTRAP_COMPILER_BASE: ${BOOTSTRAP_COMPILER_BASE}
	echo Path: $PATH
}

RUN() {
	for action in $*; do
		echo "-------------------------------------------"
		echo $action
		echo "-------------------------------------------"
		eval "$action || exit 1"
	done
}
