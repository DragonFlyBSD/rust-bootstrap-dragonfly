--- library/std/src/sys/process/unix/common/tests.rs.orig
+++ library/std/src/sys/process/unix/common/tests.rs
@@ -84,6 +84,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -111,6 +114,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -139,6 +145,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -167,6 +176,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
