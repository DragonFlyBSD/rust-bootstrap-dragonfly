--- src/bootstrap/src/core/build_steps/test.rs
+++ src/bootstrap/src/core/build_steps/test.rs
@@ -2444,7 +2444,12 @@
         // needed when diffing test output.
         cmd.env("RUSTC_FORCE_RUSTC_VERSION", "compiletest");
         cmd.env("DOC_RUST_LANG_ORG_CHANNEL", builder.doc_rust_lang_org_channel());
-        builder.add_rust_test_threads(&mut cmd);
+        if target.contains("dragonfly") && suite == "assembly" {
+            // DragonFly threaded compiletest runner can park indefinitely in this suite.
+            cmd.env("RUST_TEST_THREADS", "1");
+        } else {
+            builder.add_rust_test_threads(&mut cmd);
+        }

         if builder.config.sanitizers_enabled(target) {
             cmd.env("RUSTC_SANITIZER_SUPPORT", "1");
@@ -3413,15 +3418,32 @@
         .arg("--strip-components=1")
         .current_dir(plain_src_dir)
         .run(builder);
-    command("./configure")
-        .arg("--set")
-        .arg("rust.omit-git-hash=false")
-        .arg("--set")
-        .arg("rust.remap-debuginfo=false")
-        .args(&configure_args)
-        .arg("--enable-vendor")
-        .current_dir(plain_src_dir)
-        .run(builder);
+    let configure_has_vendor = configure_args
+        .iter()
+        .any(|arg| arg == "--enable-vendor" || arg == "--disable-vendor");
+    let configure_has_gdb = configure_args.iter().enumerate().any(|(index, arg)| {
+        arg.strip_prefix("--set=")
+            .is_some_and(|value| value.starts_with("build.gdb="))
+            || arg == "--set"
+                && configure_args
+                    .get(index + 1)
+                    .is_some_and(|value| value.starts_with("build.gdb="))
+    });
+    let mut configure = command("./configure");
+    configure.arg("--set").arg("rust.omit-git-hash=false");
+    configure.arg("--set").arg("rust.remap-debuginfo=false");
+    configure.args(&configure_args);
+    if !configure_has_vendor {
+        configure.arg("--enable-vendor");
+    }
+    if !configure_has_gdb {
+        if let Some(gdb) = &builder.config.gdb {
+            configure
+                .arg("--set")
+                .arg(format!("build.gdb={}", gdb.display()));
+        }
+    }
+    configure.current_dir(plain_src_dir).run(builder);
     command(helpers::make(&builder.config.host_target.triple))
         .arg("check")
         // Do not run the build as if we were in CI, otherwise git would be assumed to be
