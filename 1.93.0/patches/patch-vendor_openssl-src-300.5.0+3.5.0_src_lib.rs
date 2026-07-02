--- vendor/openssl-src-300.5.0+3.5.0/src/lib.rs
+++ vendor/openssl-src-300.5.0+3.5.0/src/lib.rs
@@ -248,6 +248,10 @@
 
         if cfg!(feature = "ktls") {
             configure.arg("enable-ktls");
+        }
+
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
         }
 
         if target.contains("musl") {
