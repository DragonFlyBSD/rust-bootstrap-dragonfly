--- src/bootstrap/src/core/config/tests.rs	2025-03-16 00:27:19.000000000 +0800
+++ src/bootstrap/src/core/config/tests.rs	2026-06-11 15:11:28.372563000 +0800
@@ -37,6 +37,10 @@
     let config = parse("llvm.download-ci-llvm = false");
     assert!(!config.llvm_from_ci);
 
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
+
     let if_unchanged_config = parse("llvm.download-ci-llvm = \"if-unchanged\"");
     if if_unchanged_config.llvm_from_ci {
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
