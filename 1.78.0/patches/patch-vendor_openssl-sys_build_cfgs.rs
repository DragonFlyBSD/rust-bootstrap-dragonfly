--- vendor/openssl-sys/build/cfgs.rs.orig	2020-01-27 17:21:00 UTC
+++ vendor/openssl-sys/build/cfgs.rs
@@ -28,6 +28,21 @@ pub fn get(openssl_version: Option<u64>,
         if libressl_version >= 0x2_09_01_00_0 {
             cfgs.push("libressl291");
         }
+        if libressl_version >= 0x2_09_02_00_0 {
+            cfgs.push("libressl292");
+        }
+
+        if libressl_version >= 0x3_00_00_00_0 {
+            cfgs.push("libressl300");
+        }
+
+        if libressl_version >= 0x3_00_01_00_0 {
+            cfgs.push("libressl301");
+        }
+
+        if libressl_version >= 0x3_00_02_00_0 {
+            cfgs.push("libressl302");
+        }
     } else {
         let openssl_version = openssl_version.unwrap();
 
