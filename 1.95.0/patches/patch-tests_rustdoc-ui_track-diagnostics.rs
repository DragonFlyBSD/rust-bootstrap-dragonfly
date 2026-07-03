--- tests/rustdoc-ui/track-diagnostics.rs
+++ tests/rustdoc-ui/track-diagnostics.rs
@@ -3,7 +3,7 @@
 // Normalize the emitted location so this doesn't need
 // updating everytime someone adds or removes a line.
 //@ normalize-stderr: ".rs:\d+:\d+" -> ".rs:LL:CC"
-
+//@ normalize-stderr: "/rustc(?:-dev)?/[a-z0-9.]+/" -> ""
 struct A;
 struct B;
