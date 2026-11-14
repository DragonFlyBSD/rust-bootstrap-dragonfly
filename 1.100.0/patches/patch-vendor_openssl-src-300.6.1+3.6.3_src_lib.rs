--- vendor/openssl-src-300.6.1+3.6.3/src/lib.rs.orig
+++ vendor/openssl-src-300.6.1+3.6.3/src/lib.rs
@@ -253,6 +253,10 @@
 
         if cfg!(feature = "ktls") {
             configure.arg("enable-ktls");
+        }
+
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
         }
 
         if target.contains("musl") {
