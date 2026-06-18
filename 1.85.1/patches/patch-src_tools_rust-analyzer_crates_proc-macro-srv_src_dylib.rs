--- src/tools/rust-analyzer/crates/proc-macro-srv/src/dylib.rs.orig
+++ src/tools/rust-analyzer/crates/proc-macro-srv/src/dylib.rs
@@ -64,8 +64,15 @@
 
     const RTLD_NOW: c_int = 0x00002;
     const RTLD_DEEPBIND: c_int = 0x00008;
+    const RTLD_NODELETE: c_int = 0x01000;
 
-    unsafe { UnixLibrary::open(Some(file), RTLD_NOW | RTLD_DEEPBIND).map(|lib| lib.into()) }
+    let flags = if cfg!(target_os = "dragonfly") {
+        RTLD_NOW | RTLD_NODELETE
+    } else {
+        RTLD_NOW | RTLD_DEEPBIND
+    };
+
+    unsafe { UnixLibrary::open(Some(file), flags).map(|lib| lib.into()) }
 }
 
 #[derive(Debug)]
