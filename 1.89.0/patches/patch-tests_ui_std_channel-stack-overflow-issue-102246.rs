--- tests/ui/std/channel-stack-overflow-issue-102246.rs
+++ tests/ui/std/channel-stack-overflow-issue-102246.rs
@@ -18,6 +18,13 @@
 //
 // The test explicitly specifies the stack size, because not all platforms have the same default
 // size.
+// DragonFly pthread stacks are page-granular. A non-page-aligned stack size can fault in the
+// thread startup path before this test reaches the channel code, so keep the stack smaller than
+// the old on-stack channel block while staying page-aligned.
+#[cfg(target_os = "dragonfly")]
+const STACK_SIZE: usize = (N*SLOTS) - 4096;
+
+#[cfg(not(target_os = "dragonfly"))]
 const STACK_SIZE: usize = (N*SLOTS) - 1;
 
 struct BigStruct {
