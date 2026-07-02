--- src/bootstrap/src/core/config/tests.rs
+++ src/bootstrap/src/core/config/tests.rs
@@ -37,6 +37,10 @@
     let config = TestCtx::new().config("check").create_config();
     assert!(!config.llvm_from_ci);
 
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
+
     // this doesn't make sense, as we are overriding it later.
     let if_unchanged_config = TestCtx::new()
         .config("check")
@@ -422,6 +426,9 @@
         .collect();
 
     for p in normalised_allowed_paths {
+        if config.rust_info.is_from_tarball() && p == "triagebot.toml" {
+            continue;
+        }
         assert!(config.src.join(p).exists(), "{p} doesn't exist.");
     }
 }
@@ -490,13 +497,28 @@
 
 #[test]
 fn test_ci_flag() {
-    let config = TestCtx::new().config("check").arg("--ci").arg("false").create_config();
+    let config_without_ci_llvm = "llvm.download-ci-llvm = false";
+
+    let config = TestCtx::new()
+        .config("check")
+        .arg("--ci")
+        .arg("false")
+        .with_default_toml_config(config_without_ci_llvm)
+        .create_config();
     assert!(!config.is_running_on_ci);
 
-    let config = TestCtx::new().config("check").arg("--ci").arg("true").create_config();
+    let config = TestCtx::new()
+        .config("check")
+        .arg("--ci")
+        .arg("true")
+        .with_default_toml_config(config_without_ci_llvm)
+        .create_config();
     assert!(config.is_running_on_ci);
 
-    let config = TestCtx::new().config("check").create_config();
+    let config = TestCtx::new()
+        .config("check")
+        .with_default_toml_config(config_without_ci_llvm)
+        .create_config();
     assert_eq!(config.is_running_on_ci, CiEnv::is_ci());
 }
 
