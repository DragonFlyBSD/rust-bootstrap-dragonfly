--- tests/run-make/emit-to-stdout/rmake.rs	2025-03-16 00:27:19.000000000 +0800
+++ tests/run-make/emit-to-stdout/rmake.rs	2026-06-11 14:52:05.000000000 +0800
@@ -7,6 +7,8 @@
 //! See <https://github.com/rust-lang/rust/pull/111626>.
 
 use std::fs::File;
+#[cfg(target_os = "dragonfly")]
+use std::os::fd::FromRawFd;
 
 use run_make_support::{diff, run_in_tmpdir, rustc};
 
@@ -17,15 +19,49 @@
     diff().expected_file(name).actual_text("stdout", &out).run();
 }
 
+#[cfg(not(any(windows, target_os = "dragonfly")))]
+fn terminal_file() -> File {
+    File::create("/dev/ptmx").unwrap()
+}
+
+#[cfg(target_os = "dragonfly")]
+fn terminal_file() -> File {
+    #[link(name = "util")]
+    extern "C" {
+        fn openpty(
+            amaster: *mut i32,
+            aslave: *mut i32,
+            name: *mut std::ffi::c_char,
+            termp: *const std::ffi::c_void,
+            winp: *const std::ffi::c_void,
+        ) -> i32;
+    }
+
+    let mut master = -1;
+    let mut slave = -1;
+    let rc = unsafe {
+        openpty(
+            &mut master,
+            &mut slave,
+            std::ptr::null_mut(),
+            std::ptr::null(),
+            std::ptr::null(),
+        )
+    };
+    assert_eq!(rc, 0, "openpty failed: {}", std::io::Error::last_os_error());
+    let master = unsafe { File::from_raw_fd(master) };
+    std::mem::forget(master);
+    unsafe { File::from_raw_fd(slave) }
+}
+
+#[cfg(windows)]
+fn terminal_file() -> File {
+    File::options().read(true).write(true).open(r"\\.\CONOUT$").unwrap()
+}
+
 // Test that emitting binary formats to a terminal gives the correct error
 fn run_terminal_err_diff(name: &str) {
-    #[cfg(not(windows))]
-    let terminal = File::create("/dev/ptmx").unwrap();
-    // FIXME: If this test fails and the compiler does print to the console,
-    // then this will produce a lot of output.
-    // We should spawn a new console instead to print stdout.
-    #[cfg(windows)]
-    let terminal = File::options().read(true).write(true).open(r"\\.\CONOUT$").unwrap();
+    let terminal = terminal_file();
 
     let err = File::create(name).unwrap();
     rustc().emit(format!("{name}=-")).input("test.rs").stdout(terminal).stderr(err).run_fail();
