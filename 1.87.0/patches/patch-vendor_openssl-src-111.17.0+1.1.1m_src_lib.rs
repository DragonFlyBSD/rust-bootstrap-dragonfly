--- vendor/openssl-src-111.17.0+1.1.1m/src/lib.rs.orig
+++ vendor/openssl-src-111.17.0+1.1.1m/src/lib.rs
@@ -179,6 +179,10 @@
             // API of engine.c ld fail in Windows.
             configure.arg("no-engine");
         }
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
+        }
+
 
         if target.contains("musl") {
             // MUSL doesn't implement some of the libc functions that the async
