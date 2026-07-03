--- src/tools/rust-analyzer/crates/rust-analyzer/tests/slow-tests/testdir.rs.orig
+++ src/tools/rust-analyzer/crates/rust-analyzer/tests/slow-tests/testdir.rs
@@ -42,12 +42,13 @@
                 target_os = "macos",
                 target_os = "linux",
                 target_os = "windows",
-                target_os = "freebsd"
+                target_os = "freebsd",
+                target_os = "dragonfly"
             ))]
             if symlink {
                 let symlink_path = base.join(format!("{pid}_{cnt}_symlink"));
-                #[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
+                #[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd", target_os = "dragonfly"))]
                 std::os::unix::fs::symlink(path, &symlink_path).unwrap();

                 #[cfg(target_os = "windows")]
