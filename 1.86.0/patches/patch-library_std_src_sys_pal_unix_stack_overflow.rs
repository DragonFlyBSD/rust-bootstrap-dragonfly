--- library/std/src/sys/pal/unix/stack_overflow.rs	2025-03-16 00:27:19.000000000 +0800
+++ library/std/src/sys/pal/unix/stack_overflow.rs	2026-06-11 14:01:27.661720000 +0800
@@ -28,6 +28,7 @@
 #[cfg(any(
     target_os = "linux",
     target_os = "freebsd",
+    target_os = "dragonfly",
     target_os = "hurd",
     target_os = "macos",
     target_os = "netbsd",
@@ -311,6 +312,7 @@
     #[cfg(any(
         target_os = "android",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "netbsd",
         target_os = "hurd",
         target_os = "linux",
@@ -319,11 +321,11 @@
     unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
         let mut ret = None;
         let mut attr: libc::pthread_attr_t = crate::mem::zeroed();
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         assert_eq!(libc::pthread_attr_init(&mut attr), 0);
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         let e = libc::pthread_attr_get_np(libc::pthread_self(), &mut attr);
-        #[cfg(not(target_os = "freebsd"))]
+        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly")))]
         let e = libc::pthread_getattr_np(libc::pthread_self(), &mut attr);
         if e == 0 {
             let mut stackaddr = crate::ptr::null_mut();
@@ -331,7 +333,7 @@
             assert_eq!(libc::pthread_attr_getstack(&attr, &mut stackaddr, &mut stacksize), 0);
             ret = Some(stackaddr);
         }
-        if e == 0 || cfg!(target_os = "freebsd") {
+        if e == 0 || cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);
         }
         ret
@@ -367,7 +369,7 @@
                 install_main_guard_linux_musl(page_size)
             } else if cfg!(target_os = "freebsd") {
                 install_main_guard_freebsd(page_size)
-            } else if cfg!(any(target_os = "netbsd", target_os = "openbsd")) {
+            } else if cfg!(any(target_os = "dragonfly", target_os = "netbsd", target_os = "openbsd")) {
                 install_main_guard_bsds(page_size)
             } else {
                 install_main_guard_default(page_size)
@@ -500,6 +502,7 @@
     #[cfg(any(
         target_os = "android",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "hurd",
         target_os = "linux",
         target_os = "netbsd",
@@ -509,11 +512,11 @@
     unsafe fn current_guard() -> Option<Range<usize>> {
         let mut ret = None;
         let mut attr: libc::pthread_attr_t = crate::mem::zeroed();
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         assert_eq!(libc::pthread_attr_init(&mut attr), 0);
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         let e = libc::pthread_attr_get_np(libc::pthread_self(), &mut attr);
-        #[cfg(not(target_os = "freebsd"))]
+        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly")))]
         let e = libc::pthread_getattr_np(libc::pthread_self(), &mut attr);
         if e == 0 {
             let mut guardsize = 0;
@@ -533,7 +536,7 @@
             assert_eq!(libc::pthread_attr_getstack(&attr, &mut stackptr, &mut size), 0);
 
             let stackaddr = stackptr.addr();
-            ret = if cfg!(any(target_os = "freebsd", target_os = "netbsd", target_os = "hurd")) {
+            ret = if cfg!(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "hurd")) {
                 Some(stackaddr - guardsize..stackaddr)
             } else if cfg!(all(target_os = "linux", target_env = "musl")) {
                 Some(stackaddr - guardsize..stackaddr)
@@ -550,7 +553,7 @@
                 Some(stackaddr..stackaddr + guardsize)
             };
         }
-        if e == 0 || cfg!(target_os = "freebsd") {
+        if e == 0 || cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             assert_eq!(libc::pthread_attr_destroy(&mut attr), 0);
         }
         ret
@@ -568,6 +571,7 @@
 #[cfg(not(any(
     target_os = "linux",
     target_os = "freebsd",
+    target_os = "dragonfly",
     target_os = "hurd",
     target_os = "macos",
     target_os = "netbsd",
