--- vendor/libc-0.2.108/src/unix/bsd/freebsdlike/dragonfly/mod.rs.orig	2022-02-23 05:34:25 UTC
+++ vendor/libc-0.2.108/src/unix/bsd/freebsdlike/dragonfly/mod.rs
@@ -106,11 +106,6 @@ s! {
         pub f_uid_uuid: ::uuid_t,
     }
 
-    #[deprecated(
-        since = "0.2.107",
-        note = "stat.st_blksize is an i64 and stat.st_qspare1 is replaced with \
-                stat.st_blksize in DragonFly 5.8"
-    )]
     pub struct stat {
         pub st_ino: ::ino_t,
         pub st_nlink: ::nlink_t,
@@ -128,11 +123,11 @@ s! {
         pub st_ctime_nsec: ::c_long,
         pub st_size: ::off_t,
         pub st_blocks: i64,
-        pub st_blksize: u32,
+        pub __old_st_blksize: u32,
         pub st_flags: u32,
         pub st_gen: u32,
         pub st_lspare: i32,
-        pub st_qspare1: i64,
+        pub st_blksize: i64,
         pub st_qspare2: i64,
     }
 
