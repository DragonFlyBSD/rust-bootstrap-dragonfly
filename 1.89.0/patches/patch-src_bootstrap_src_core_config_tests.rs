--- src/bootstrap/src/core/config/tests.rs
+++ src/bootstrap/src/core/config/tests.rs
@@ -28,6 +28,10 @@
     let config = parse("llvm.download-ci-llvm = false");
     assert!(!config.llvm_from_ci);
 
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
+
     let if_unchanged_config = parse("llvm.download-ci-llvm = \"if-unchanged\"");
     if if_unchanged_config.llvm_from_ci && if_unchanged_config.is_running_on_ci {
         let has_changes = if_unchanged_config
@@ -444,6 +448,9 @@
         .collect();
 
     for p in normalised_allowed_paths {
+        if config.rust_info.is_from_tarball() && p == "triagebot.toml" {
+            continue;
+        }
         assert!(config.src.join(p).exists(), "{p} doesn't exist.");
     }
 }
@@ -526,16 +533,20 @@
 
 #[test]
 fn test_ci_flag() {
+    let config_without_ci_llvm = "llvm.download-ci-llvm = false";
+
     let config = Config::parse_inner(Flags::parse(&["check".into(), "--ci=false".into()]), |&_| {
-        toml::from_str("")
+        toml::from_str(config_without_ci_llvm)
     });
     assert!(!config.is_running_on_ci);
 
     let config = Config::parse_inner(Flags::parse(&["check".into(), "--ci=true".into()]), |&_| {
-        toml::from_str("")
+        toml::from_str(config_without_ci_llvm)
     });
     assert!(config.is_running_on_ci);
 
-    let config = Config::parse_inner(Flags::parse(&["check".into()]), |&_| toml::from_str(""));
+    let config = Config::parse_inner(Flags::parse(&["check".into()]), |&_| {
+        toml::from_str(config_without_ci_llvm)
+    });
     assert_eq!(config.is_running_on_ci, CiEnv::is_ci());
 }
