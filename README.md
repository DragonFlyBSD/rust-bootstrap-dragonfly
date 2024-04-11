# rust-bootstrap-dragonfly

Boostrapping the Rust compiler on DragonFlyBSD (Scripts only, no binaries)

## Build dependencies

* devel/llvm50
* (devel/llvm60)
* gcc7
* lang/python (e.g. python39)
* devel/cmake
* security/libssh
* perl5 (for shasum)

## Howto build

Currently, only Rust >= 1.28.0 can be build, due to changing where
the bootstrap files are fetched. 

	mkdir -p $HOME/rust
	cd $HOME/rust && git clone https://github.com/mneumann/rust-bootstrap-dragonfly.git
	cd $HOME/rust/rust-bootstrap-dragonfly/1.28.0 && ./build.sh $HOME/rust/build-1.28.0
