--- vendor/openssl-src-300.4.2+3.4.1/src/lib.rs.orig
+++ vendor/openssl-src-300.4.2+3.4.1/src/lib.rs
@@ -249,6 +249,10 @@
         if cfg!(feature = "ktls") {
             configure.arg("enable-ktls");
         }
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
+        }
+
 
         if target.contains("musl") {
             // Engine module fails to compile on musl (it needs linux/version.h
