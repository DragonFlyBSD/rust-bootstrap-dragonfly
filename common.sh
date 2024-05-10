
# Expected variables:
#
# * RUSTC_BOOTSTRAP_VERSION
# * CARGO_BOOTSTRAP_VERSION
# * RUST_VERSION
# * FIXUP_VENDOR_{1,2...,n}
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
#   MAKE_CHECKSUM
#   PYTHON_BIN

set -e

if [ "${RUST_DIST_SERVER}" = "" ]; then
	RUST_DIST_SERVER=https://static.rust-lang.org
else
	echo "Using non-default RUST_DIST_SERVER=${RUST_DIST_SERVER}"
fi

if [ "${PYTHON_BIN}" = "" ]; then
	if [ -x /usr/local/bin/python ]; then
		PYTHON_BIN=/usr/local/bin/python
	elif [ -x /usr/local/bin/python3.11 ]; then
		PYTHON_BIN=/usr/local/bin/python3.11
	elif [ -x /usr/local/bin/python3.10 ]; then
		PYTHON_BIN=/usr/local/bin/python3.10
	elif [ -x /usr/local/bin/python3.9 ]; then
		PYTHON_BIN=/usr/local/bin/python3.9
	elif [ -x /usr/local/bin/python3.8 ]; then
		PYTHON_BIN=/usr/local/bin/python3.8
	else
		echo "No proper python interpreter found. Please set PYTHON_BIN!"
		exit 1
	fi
fi
echo "Using PYTHON_BIN=${PYTHON_BIN}"

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
export LIBGIT2_NO_PKG_CONFIG=1
export LIBCURL_NO_PKG_CONFIG=1
export LIBZ_NO_PKG_CONFIG=1 
export LIBLZMA_NO_PKG_CONFIG=1
export PROFILE=release
export LIBZ_SYS_STATIC=1
export OPENSSL_NO_PKG_CONFIG=1

clean() {
	rm -rf $DEST
}

# filename cachepath downloadpath url
download() {
	file=`echo $1 | tr  ".-" "_"`
	eval "local expected_cksum=\$SHA256_${file}"
	if [ "${expected_cksum}" = "" ]; then
		echo -n "no sha256 checksum found for $2/$1."
		if [ "${MAKE_CHECKSUM}" = "" ]; then
			echo " aborting"
			exit 1
		else
			echo " continue"
		fi
	fi

	if [ -f $2/$1 ]; then
		echo "$1 exists in $2"
		cp $2/$1 $3/$1
	else
		echo "download: $1 from $4 and store into $3"
		fetch -o $3/$1 $4/$1
	fi

	cksum=`sha256 -q $3/$1`

	if [ "${expected_cksum}" = "" ]; then
		if [ "${MAKE_CHECKSUM}" != "" ]; then
			# Append checksum to checksums.sh
			echo "SHA256_${file}=${cksum}" >> ../checksums.sh
		else
			echo FATAL
			exit 1
		fi
	elif [ "${cksum}" = "${expected_cksum}" ]; then
		echo "checksum ok for $1"
	else
		echo "checksum mismatch. expected: ${expected_cksum}. given: ${cksum}"
		exit 1
	fi
}

SHA256=sha256
REINPLACE_CMD="sed -i.bak"

fixup-vendor-patch() {
	local dir=$DEST/rustc-$RUST_VERSION-src/vendor/$1
	local file=$2

	echo "Regenerating checksum for ${dir}/${file}"

	test -d ${dir} || {
		echo "`${dir}` is no directory"
		exit 1
	}

	local old_checksum=$(${SHA256} -q "${dir}/${file}.orig")
	local new_checksum=$(${SHA256} -q "${dir}/${file}")
	local regex="-e s|\"${file}\":\"${old_checksum}\"|\"${file}\":\"${new_checksum}\"|"; \
	${REINPLACE_CMD} -E ${regex} ${dir}/.cargo-checksum.json || exit 1
}

fixup-vendor() {
	fixup-vendor-loop 1
}

fixup-vendor-loop() {
	local _i="$1"
	eval "local v=\$FIXUP_VENDOR_${_i}"
	if [ "${v}" = "" ]; then
		return
	fi
	local dir=$(echo "${v}" | cut -d : -f 1)
	local file=$(echo "${v}" | cut -d : -f 2)
	fixup-vendor-patch "${dir}" "${file}" || exit 1

	fixup-vendor-loop "$(expr $_i + 1)"
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
	if [ -d $BASE/patches ]; then
		for patch in $BASE/patches/patch-*; do
			echo $patch
			(cd $DEST/rustc-$RUST_VERSION-src && patch < $patch) || exit 1
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

# Disable llvm static linking against libstdc++
# https://github.com/rust-lang/rust/pull/94832
config() {
	(cd $DEST/rustc-$RUST_VERSION-src &&
	runcmd ./configure \
		--release-channel=${RELEASE_CHANNEL} \
		${CONFIGURE_CARGO_STATIC_FLAGS} --enable-extended \
		--enable-vendor \
		${ADDITIONAL_CONFIGURE_FLAGS} \
		--enable-locked-deps \
		--local-rust-root=${BOOTSTRAP_COMPILER_BASE} \
		--sysconfdir=${DEST_INSTALL}/etc \
		--prefix=${DEST_INSTALL} \
		--python=${PYTHON_BIN} \
		--disable-llvm-static-stdcpp \
		--disable-docs \
		${LLVM_ROOT_OPT}
	)
}

runcmd() {
	echo "Running command: $*"
	$*
}

mk() {
	(cd $DEST/rustc-$RUST_VERSION-src && gmake VERBOSE=YES $1)
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
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py build --verbose --config ./config.toml --jobs $(/sbin/sysctl -n hw.ncpu))
}

xdist() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py dist --verbose --config ./config.toml)
}

xinst() {
	(cd $DEST/rustc-$RUST_VERSION-src && ${PYTHON_BIN} x.py install --verbose --config ./config.toml)
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

RUN-ALL() {
	RUN info clean extract prepatch fixup-vendor config xbuild xdist inst
}
