--- vendor/openssl-src-111.28.2+1.1.1w/src/lib.rs.orig
+++ vendor/openssl-src-111.28.2+1.1.1w/src/lib.rs
@@ -200,6 +200,10 @@
             }
         }
 
+        if target.contains("dragonfly") {
+            configure.arg("no-devcryptoeng");
+        }
+
         if target.contains("musl") {
             // MUSL doesn't implement some of the libc functions that the async
             // stuff depends on, and we don't bind to any of that in any case.
