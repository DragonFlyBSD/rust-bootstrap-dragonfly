--- vendor/libc/src/unix/bsd/freebsdlike/dragonfly/mod.rs.intermediate	2024-09-17 13:28:09 UTC
+++ vendor/libc/src/unix/bsd/freebsdlike/dragonfly/mod.rs
@@ -1508,6 +1508,11 @@ pub const SF_NOHISTORY: ::c_ulong = 0x00
 pub const SF_CACHE: ::c_ulong = 0x00800000;
 pub const SF_XLINK: ::c_ulong = 0x01000000;
 
+// For getrandom()
+pub const GRND_NONBLOCK: ::c_uint = 0x1;
+pub const GRND_RANDOM: ::c_uint = 0x2;
+pub const GRND_INSECURE: ::c_uint = 0x4;
+
 // timespec constants
 pub const UTIME_OMIT: c_long = -2;
 pub const UTIME_NOW: c_long = -1;
@@ -1563,6 +1568,10 @@ f! {
         (::mem::size_of::<cpumask_t>() * 8) as ::c_int
     }
 
+    pub fn cpu_setsize(cpumask: &mut cpumask_t) -> ::c_int {
+        (::mem::size_of::<cpumask_t>() * 8) as ::c_int
+    }
+
     pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
         for slot in cpuset.ary.iter_mut() {
             *slot = 0;
@@ -1663,6 +1672,8 @@ extern "C" {
     pub fn sched_getcpu() -> ::c_int;
     pub fn setproctitle(fmt: *const ::c_char, ...);
 
+    pub fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t;
+
     pub fn shmget(key: ::key_t, size: ::size_t, shmflg: ::c_int) -> ::c_int;
     pub fn shmat(shmid: ::c_int, shmaddr: *const ::c_void, shmflg: ::c_int) -> *mut ::c_void;
     pub fn shmdt(shmaddr: *const ::c_void) -> ::c_int;
