--- src/libstd/build.rs.orig	2018-03-25 14:26:14 UTC
+++ src/libstd/build.rs
@@ -42,9 +42,13 @@ fn main() {
     } else if target.contains("freebsd") {
         println!("cargo:rustc-link-lib=execinfo");
         println!("cargo:rustc-link-lib=pthread");
-    } else if target.contains("dragonfly") || target.contains("bitrig") ||
-              target.contains("netbsd") || target.contains("openbsd") {
+    } else if target.contains("bitrig") || target.contains("netbsd") ||
+              target.contains("openbsd") {
         println!("cargo:rustc-link-lib=pthread");
+    } else if target.contains("dragonfly") {
+        println!("cargo:rustc-link-lib=pthread");
+        println!("cargo:rustc-link-search=native=/usr/lib");
+        println!("cargo:rustc-link-lib=static=execinfo");
     } else if target.contains("solaris") {
         println!("cargo:rustc-link-lib=socket");
         println!("cargo:rustc-link-lib=posix4");
