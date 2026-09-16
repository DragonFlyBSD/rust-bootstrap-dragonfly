--- tests/run-make/rust-lld/rmake.rs.orig
+++ tests/run-make/rust-lld/rmake.rs
@@ -2,6 +2,9 @@
 // `-Clink-self-contained` CLI flags.
 
 //@ needs-rust-lld
+//@ ignore-dragonfly
+// Reason: DragonFly does not currently route the cc linker flavor through
+// bundled rust-lld.
 //@ ignore-s390x lld does not yet support s390x as target
 
 use run_make_support::linker::{assert_rustc_doesnt_use_lld, assert_rustc_uses_lld};
