--- src/tools/rust-analyzer/crates/proc-macro-srv/src/dylib.rs.orig
+++ src/tools/rust-analyzer/crates/proc-macro-srv/src/dylib.rs
@@ -37,7 +37,15 @@ fn load_library(file: &Utf8Path) -> Resu
     #[cfg(not(target_env = "gnu"))]
     const RTLD_DEEPBIND: std::os::raw::c_int = 0x0;
 
-    unsafe { UnixLibrary::open(Some(file), RTLD_NOW | RTLD_DEEPBIND).map(|lib| lib.into()) }
+    const RTLD_NODELETE: std::os::raw::c_int = 0x01000;
+
+    let flags = if cfg!(target_os = "dragonfly") {
+        RTLD_NOW | RTLD_NODELETE
+    } else {
+        RTLD_NOW | RTLD_DEEPBIND
+    };
+
+    unsafe { UnixLibrary::open(Some(file), flags).map(|lib| lib.into()) }
 }
 
 #[derive(Debug)]
