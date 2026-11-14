--- library/std/src/sys/process/unix/common/tests.rs.orig
+++ library/std/src/sys/process/unix/common/tests.rs
@@ -86,6 +86,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         // cat not available
         target_os = "l4re",
         target_arch = "arm",
@@ -120,6 +123,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         // cat not available
         target_os = "l4re",
         target_arch = "arm",
@@ -160,6 +166,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         // cat not available
         target_os = "l4re",
         target_arch = "arm",
@@ -200,6 +209,9 @@
     any(
         // See test_process_mask
         target_os = "macos",
+        // DragonFly can leave these process-group tests hanging after the
+        // signal is sent, so skip them in the bootstrap test run.
+        target_os = "dragonfly",
         // cat not available
         target_os = "l4re",
         target_arch = "arm",
