--- library/std/src/sys/pal/unix/stack_overflow.rs.orig
+++ library/std/src/sys/pal/unix/stack_overflow.rs
@@ -30,6 +30,7 @@
     any(
         target_os = "linux",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "hurd",
         target_os = "macos",
         target_os = "netbsd",
@@ -48,6 +49,7 @@
     any(
         target_os = "linux",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "hurd",
         target_os = "macos",
         target_os = "netbsd",
@@ -347,6 +349,7 @@
     #[cfg(any(
         target_os = "android",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "netbsd",
         target_os = "hurd",
         target_os = "linux",
@@ -355,14 +358,14 @@
     unsafe fn get_stack_start() -> Option<*mut libc::c_void> {
         let mut ret = None;
         let mut attr: mem::MaybeUninit<libc::pthread_attr_t> = mem::MaybeUninit::uninit();
-        if !cfg!(target_os = "freebsd") {
+        if !cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             attr = mem::MaybeUninit::zeroed();
         }
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         assert_eq!(libc::pthread_attr_init(attr.as_mut_ptr()), 0);
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         let e = libc::pthread_attr_get_np(libc::pthread_self(), attr.as_mut_ptr());
-        #[cfg(not(target_os = "freebsd"))]
+        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly")))]
         let e = libc::pthread_getattr_np(libc::pthread_self(), attr.as_mut_ptr());
         if e == 0 {
             let mut stackaddr = crate::ptr::null_mut();
@@ -373,7 +376,7 @@
             );
             ret = Some(stackaddr);
         }
-        if e == 0 || cfg!(target_os = "freebsd") {
+        if e == 0 || cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             assert_eq!(libc::pthread_attr_destroy(attr.as_mut_ptr()), 0);
         }
         ret
@@ -413,7 +416,7 @@
                 // The FreeBSD code cannot be checked on non-BSDs.
                 #[cfg(target_os = "freebsd")]
                 install_main_guard_freebsd(page_size)
-            } else if cfg!(any(target_os = "netbsd", target_os = "openbsd")) {
+            } else if cfg!(any(target_os = "dragonfly", target_os = "netbsd", target_os = "openbsd")) {
                 install_main_guard_bsds(page_size)
             } else {
                 install_main_guard_default(page_size)
@@ -559,6 +562,7 @@
     #[cfg(any(
         target_os = "android",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "hurd",
         target_os = "linux",
         target_os = "netbsd",
@@ -569,14 +573,14 @@
         let mut ret = None;
 
         let mut attr: mem::MaybeUninit<libc::pthread_attr_t> = mem::MaybeUninit::uninit();
-        if !cfg!(target_os = "freebsd") {
+        if !cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             attr = mem::MaybeUninit::zeroed();
         }
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         assert_eq!(libc::pthread_attr_init(attr.as_mut_ptr()), 0);
-        #[cfg(target_os = "freebsd")]
+        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
         let e = libc::pthread_attr_get_np(libc::pthread_self(), attr.as_mut_ptr());
-        #[cfg(not(target_os = "freebsd"))]
+        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly")))]
         let e = libc::pthread_getattr_np(libc::pthread_self(), attr.as_mut_ptr());
         if e == 0 {
             let mut guardsize = 0;
@@ -596,7 +600,7 @@
             assert_eq!(libc::pthread_attr_getstack(attr.as_ptr(), &mut stackptr, &mut size), 0);
 
             let stackaddr = stackptr.addr();
-            ret = if cfg!(any(target_os = "freebsd", target_os = "netbsd", target_os = "hurd")) {
+            ret = if cfg!(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "hurd")) {
                 Some(stackaddr - guardsize..stackaddr)
             } else if cfg!(all(target_os = "linux", target_env = "musl")) {
                 Some(stackaddr - guardsize..stackaddr)
@@ -613,7 +617,7 @@
                 Some(stackaddr..stackaddr + guardsize)
             };
         }
-        if e == 0 || cfg!(target_os = "freebsd") {
+        if e == 0 || cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
             assert_eq!(libc::pthread_attr_destroy(attr.as_mut_ptr()), 0);
         }
         ret
@@ -633,6 +637,7 @@
     not(any(
         target_os = "linux",
         target_os = "freebsd",
+        target_os = "dragonfly",
         target_os = "hurd",
         target_os = "macos",
         target_os = "netbsd",
