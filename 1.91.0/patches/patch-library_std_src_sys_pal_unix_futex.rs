--- library/std/src/sys/pal/unix/futex.rs.orig
+++ library/std/src/sys/pal/unix/futex.rs
@@ -198,17 +198,20 @@
 
 #[cfg(target_os = "dragonfly")]
 pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
+    let has_timeout = timeout.is_some();
+
+    // DragonFly umtx_sleep takes a timeout in microseconds.
     // A timeout of 0 means infinite.
-    // We round smaller timeouts up to 1 millisecond.
+    // We round smaller timeouts up to 1 microsecond.
     // Overflows are rounded up to an infinite timeout.
-    let timeout_ms =
-        timeout.and_then(|d| Some(i32::try_from(d.as_millis()).ok()?.max(1))).unwrap_or(0);
+    let timeout_us =
+        timeout.and_then(|d| Some(i32::try_from(d.as_micros()).ok()?.max(1))).unwrap_or(0);
 
     let r = unsafe {
-        libc::umtx_sleep(futex as *const Atomic<u32> as *const i32, expected as i32, timeout_ms)
+        libc::umtx_sleep(futex as *const Atomic<u32> as *const i32, expected as i32, timeout_us)
     };
 
-    r == 0 || super::os::errno() != libc::ETIMEDOUT
+    r == 0 || !has_timeout || super::os::errno() != libc::EWOULDBLOCK
 }
 
 // DragonflyBSD doesn't tell us how many threads are woken up, so this always returns false.
