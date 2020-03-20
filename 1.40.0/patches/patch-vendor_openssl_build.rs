--- vendor/openssl/build.rs.orig	2019-09-23 23:15:03 UTC
+++ vendor/openssl/build.rs
@@ -46,6 +46,10 @@ fn main() {
             println!("cargo:rustc-cfg=libressl270");
         }
 
+        if version >= 0x2_07_01_00_0 {
+            println!("cargo:rustc-cfg=libressl271");
+        }
+
         if version >= 0x2_07_03_00_0 {
             println!("cargo:rustc-cfg=libressl273");
         }
@@ -53,5 +57,25 @@ fn main() {
         if version >= 0x2_08_00_00_0 {
             println!("cargo:rustc-cfg=libressl280");
         }
+
+        if version >= 0x2_09_01_00_0 {
+            println!("cargo:rustc-cfg=libressl291");
+        }
+
+        if version >= 0x2_09_02_00_0 {
+            println!("cargo:rustc-cfg=libressl292");
+        }
+
+        if version >= 0x3_00_00_00_0 {
+            println!("cargo:rustc-cfg=libressl300");
+        }
+
+        if version >= 0x3_00_01_00_0 {
+            println!("cargo:rustc-cfg=libressl301");
+        }
+
+        if version >= 0x3_00_02_00_0 {
+            println!("cargo:rustc-cfg=libressl302");
+        }
     }
 }
