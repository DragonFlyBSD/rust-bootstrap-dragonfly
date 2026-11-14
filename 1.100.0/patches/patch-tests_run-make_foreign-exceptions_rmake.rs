--- tests/run-make/foreign-exceptions/rmake.rs.orig
+++ tests/run-make/foreign-exceptions/rmake.rs
@@ -7,6 +7,9 @@
 
 //@ needs-unwind
 // Reason: this test exercises panic unwinding
+//@ ignore-dragonfly
+// Reason: DragonFly currently links libgcc_pic statically, and this C++/Rust
+// cross-language unwinding path is not supported yet.
 //@ ignore-cross-compile
 // Reason: the compiled binary is executed
 
