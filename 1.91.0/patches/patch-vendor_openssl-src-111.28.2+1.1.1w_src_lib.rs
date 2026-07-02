--- vendor/openssl-src-111.28.2+1.1.1w/src/lib.rs
+++ vendor/openssl-src-111.28.2+1.1.1w/src/lib.rs
@@ -198,6 +198,10 @@
             if !cfg!(feature = "force-engine") {
                 configure.arg("no-engine");
             }
+        }
+
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
         }
 
         if target.contains("musl") {
