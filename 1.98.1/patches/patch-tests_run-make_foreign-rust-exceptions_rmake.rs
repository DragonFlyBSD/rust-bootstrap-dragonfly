--- tests/run-make/foreign-rust-exceptions/rmake.rs.orig
+++ tests/run-make/foreign-rust-exceptions/rmake.rs
@@ -9,6 +9,9 @@
 // Reason: the compiled binary is executed
 //@ needs-unwind
 // Reason: unwinding panics is exercised in this test
+//@ ignore-dragonfly
+// Reason: DragonFly currently links libgcc_pic statically, so cross-DSO Rust
+// panic classification is not supported yet.
 
 //@ ignore-i686-pc-windows-gnu
 // Reason: This test doesn't work on 32-bit MinGW as cdylib has its own copy of unwinder
