--- library/std/src/sys/process/unix/common/tests.rs.orig	2026-07-02 00:00:00.000000000 +0800
+++ library/std/src/sys/process/unix/common/tests.rs	2026-07-02 00:00:00.000000000 +0800
@@ -87,6 +87,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -114,6 +117,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -142,6 +148,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
@@ -170,6 +179,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         target_arch = "arm",
         target_arch = "aarch64",
         target_arch = "riscv64",
