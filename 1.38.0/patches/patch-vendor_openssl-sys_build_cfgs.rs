--- vendor/openssl-sys/build/cfgs.rs.orig	2019-12-11 12:22:14 UTC
+++ vendor/openssl-sys/build/cfgs.rs
@@ -13,6 +13,9 @@ pub fn get(openssl_version: Option<u64>,
         if libressl_version >= 0x2_07_00_00_0 {
             cfgs.push("libressl270");
         }
+        if libressl_version >= 0x2_07_01_00_0 {
+            cfgs.push("libressl271");
+        }
         if libressl_version >= 0x2_07_03_00_0 {
             cfgs.push("libressl273");
         }
@@ -22,6 +25,25 @@ pub fn get(openssl_version: Option<u64>,
         if libressl_version >= 0x2_08_01_00_0 {
             cfgs.push("libressl281");
         }
+        if libressl_version >= 0x2_09_01_00_0 {
+            cfgs.push("libressl291");
+        }
+
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
 
