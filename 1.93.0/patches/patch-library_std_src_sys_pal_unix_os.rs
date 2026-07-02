--- library/std/src/sys/pal/unix/os.rs
+++ library/std/src/sys/pal/unix/os.rs
@@ -26,10 +26,11 @@
 }
 
 unsafe extern "C" {
-    #[cfg(not(any(target_os = "dragonfly", target_os = "vxworks", target_os = "rtems")))]
+    #[cfg(not(any(target_os = "vxworks", target_os = "rtems")))]
     #[cfg_attr(
         any(
             target_os = "linux",
+            target_os = "dragonfly",
             target_os = "emscripten",
             target_os = "fuchsia",
             target_os = "l4re",
