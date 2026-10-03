--- tests/run-make/dynamic-loading-cdylib/rmake.rs.orig
+++ tests/run-make/dynamic-loading-cdylib/rmake.rs
@@ -22,8 +22,14 @@
         #[cfg(unix)]
         let output_filename = format!("output_{}_unix.txt", command_arg);
 
+        let expected = std::fs::read_to_string(&output_filename).unwrap();
+        // DragonFly uses pthread-key destructors, which do not run on main-thread
+        // process exit (see LocalKey's documented platform-specific behavior).
+        #[cfg(target_os = "dragonfly")]
+        let expected = expected.strip_suffix("dropping, last result: 6\n").unwrap();
+
         diff()
-            .expected_file(output_filename)
+            .expected_text(&output_filename, expected)
             .actual_text("actual", out_raw)
             .normalize(r#"\r"#, "")
             .run();
