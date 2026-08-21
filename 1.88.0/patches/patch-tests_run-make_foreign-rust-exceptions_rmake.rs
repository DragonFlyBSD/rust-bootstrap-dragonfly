--- tests/run-make/foreign-rust-exceptions/rmake.rs	2025-03-16 00:27:19.000000000 +0800
+++ tests/run-make/foreign-rust-exceptions/rmake.rs	2026-06-11 14:52:42.992337000 +0800
@@ -9,6 +9,9 @@
 // Reason: the compiled binary is executed
 //@ needs-unwind
 // Reason: unwinding panics is exercised in this test
+//@ ignore-dragonfly
+// Reason: DragonFly currently links libgcc_pic statically, so cross-DSO Rust
+// panic classification is not supported yet.
 
 //@ ignore-i686-pc-windows-gnu
 // Reason: This test doesn't work on 32-bit MinGW as cdylib has its own copy of unwinder
