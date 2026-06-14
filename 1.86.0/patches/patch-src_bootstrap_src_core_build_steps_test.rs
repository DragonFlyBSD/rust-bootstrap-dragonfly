--- src/bootstrap/src/core/build_steps/test.rs
+++ src/bootstrap/src/core/build_steps/test.rs
@@ -2097,7 +2097,12 @@
         // needed when diffing test output.
         cmd.env("RUSTC_FORCE_RUSTC_VERSION", "compiletest");
         cmd.env("DOC_RUST_LANG_ORG_CHANNEL", builder.doc_rust_lang_org_channel());
-        builder.add_rust_test_threads(&mut cmd);
+        if target.contains("dragonfly") && suite == "assembly" {
+            // DragonFly's threaded compiletest runner can park indefinitely in this suite.
+            cmd.env("RUST_TEST_THREADS", "1");
+        } else {
+            builder.add_rust_test_threads(&mut cmd);
+        }
 
         if builder.config.sanitizers_enabled(target) {
             cmd.env("RUSTC_SANITIZER_SUPPORT", "1");
@@ -3058,11 +3058,39 @@
             .arg("--strip-components=1")
             .current_dir(&dir)
             .run(builder);
-        command("./configure")
-            .args(&builder.config.configure_args)
-            .arg("--enable-vendor")
-            .current_dir(&dir)
-            .run(builder);
+        let configure_has_vendor = builder
+            .config
+            .configure_args
+            .iter()
+            .any(|arg| arg == "--enable-vendor" || arg == "--disable-vendor");
+        let configure_has_gdb = builder
+            .config
+            .configure_args
+            .iter()
+            .enumerate()
+            .any(|(index, arg)| {
+                arg.strip_prefix("--set=")
+                    .is_some_and(|value| value.starts_with("build.gdb="))
+                    || arg == "--set"
+                        && builder
+                            .config
+                            .configure_args
+                            .get(index + 1)
+                            .is_some_and(|value| value.starts_with("build.gdb="))
+            });
+        let mut configure = command("./configure");
+        configure.args(&builder.config.configure_args);
+        if !configure_has_vendor {
+            configure.arg("--enable-vendor");
+        }
+        if !configure_has_gdb {
+            if let Some(gdb) = &builder.config.gdb {
+                configure
+                    .arg("--set")
+                    .arg(format!("build.gdb={}", gdb.display()));
+            }
+        }
+        configure.current_dir(&dir).run(builder);
         command(helpers::make(&builder.config.build.triple))
             .arg("check")
             .current_dir(&dir)
