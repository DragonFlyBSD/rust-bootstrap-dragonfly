--- library/std/src/sys/thread_local/mod.rs	2025-03-16 00:27:19.000000000 +0800
+++ library/std/src/sys/thread_local/mod.rs	2026-06-11 13:39:16.841453000 +0800
@@ -59,8 +59,7 @@
             target_os = "fuchsia",
             target_os = "redox",
             target_os = "hurd",
-            target_os = "netbsd",
-            target_os = "dragonfly"
+            target_os = "netbsd"
         ))] {
             mod linux_like;
             mod list;
