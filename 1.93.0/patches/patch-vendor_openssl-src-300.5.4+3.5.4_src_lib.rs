--- vendor/openssl-src-300.5.4+3.5.4/src/lib.rs
+++ vendor/openssl-src-300.5.4+3.5.4/src/lib.rs
@@ -248,6 +248,10 @@
 
         if cfg!(feature = "ktls") {
             configure.arg("enable-ktls");
+        }
+
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
         }
 
         if target.contains("musl") {
