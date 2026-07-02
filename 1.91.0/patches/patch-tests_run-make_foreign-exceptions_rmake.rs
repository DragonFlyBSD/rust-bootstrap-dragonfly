--- tests/run-make/foreign-exceptions/rmake.rs	2025-03-16 00:27:19.000000000 +0800
+++ tests/run-make/foreign-exceptions/rmake.rs	2026-06-11 14:52:42.992337000 +0800
@@ -7,6 +7,9 @@
 
 //@ needs-unwind
 // Reason: this test exercises panic unwinding
+//@ ignore-dragonfly
+// Reason: DragonFly currently links libgcc_pic statically, and this C++/Rust
+// cross-language unwinding path is not supported yet.
 //@ ignore-cross-compile
 // Reason: the compiled binary is executed
 
