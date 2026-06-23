--- compiler/rustc_codegen_ssa/src/back/link.rs	2025-03-16 00:27:19.000000000 +0800
+++ compiler/rustc_codegen_ssa/src/back/link.rs	2026-06-11 14:48:41.552289000 +0800
@@ -1743,7 +1743,9 @@
 
     #[cfg(unix)]
     fn command_line_too_big(err: &io::Error) -> bool {
-        err.raw_os_error() == Some(::libc::E2BIG)
+        let raw_os_error = err.raw_os_error();
+        raw_os_error == Some(::libc::E2BIG)
+            || (cfg!(target_os = "dragonfly") && raw_os_error == Some(::libc::EFAULT))
     }
 
     #[cfg(windows)]
