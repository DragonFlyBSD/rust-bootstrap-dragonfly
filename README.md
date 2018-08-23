# rust-bootstrap-dragonfly

Boostrapping the Rust compiler on DragonFlyBSD (Scripts only, no binaries)

## Build dependencies

* devel/llvm50
* (devel/llvm60)
* gcc7
* lang/python27
* devel/cmake
* security/libssh
* perl5 (for shasum)

## Howto build

	# To build rust <= 1.27
	cd $HOME
	mkdir rust
	cd $HOME/rust && git clone https://github.com/mneumann/rust-bootstrap-dragonfly-dist.git
	export BOOTSTRAP_DIR=$HOME/rust/rust-bootstrap-dragonfly-dist

	cd $HOME/rust && git clone https://github.com/mneumann/rust-bootstrap-dragonfly.git
	cd $HOME/rust/rust-bootstrap-dragonfly/1.26.0 && ./build.sh $HOME/rust/build-1.26.0

    # To build rust > 1.27
	cd $HOME
	mkdir rust
	# replace 1.27.0 below by the previous version, e.g. for 1.28.0 use 1.27.0
	cd $HOME/rust && git clone https://github.com/mneumann/rust-dragonfly-dist-1.27.0.git
	export BOOTSTRAP_DIR=$HOME/rust/rust-dragonfly-dist-1.27.0

	cd $HOME/rust && git clone https://github.com/mneumann/rust-bootstrap-dragonfly.git
	cd $HOME/rust/rust-bootstrap-dragonfly/1.28.0 && ./build.sh $HOME/rust/build-1.28.0
