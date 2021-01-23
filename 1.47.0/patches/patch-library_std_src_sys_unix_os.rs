--- library/std/src/sys/unix/os.rs.orig	2020-10-07 07:53:22 UTC
+++ library/std/src/sys/unix/os.rs
@@ -87,6 +87,7 @@ pub fn errno() -> i32 {
 }
 
 #[cfg(target_os = "dragonfly")]
+#[allow(dead_code)] // but not all target cfgs actually end up using it
 pub fn set_errno(e: i32) {
     extern "C" {
         #[thread_local]
