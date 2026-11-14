--- src/bootstrap/src/core/builder/tests.rs.orig
+++ src/bootstrap/src/core/builder/tests.rs
@@ -7,6 +7,7 @@
 use super::*;
 use crate::core::build_steps::llvm::LlvmKind;
 use crate::core::config::Config;
+use crate::core::download::is_download_ci_available;
 use crate::utils::cache::ExecutedStep;
 use crate::utils::helpers::get_host_target;
 use crate::utils::tests::git::{GitCtx, git_test};
@@ -72,7 +73,13 @@
         ctx.create_nonupstream_merge(&["library/foo"]);
 
         let config = parse_config_download_rustc_at(ctx.get_path(), "if-unchanged", false);
-        assert_eq!(config.download_rustc_commit, Some(sha));
+        let expected =
+            if is_download_ci_available(&config.host_target.triple, config.llvm_assertions) {
+                Some(sha)
+            } else {
+                None
+            };
+        assert_eq!(config.download_rustc_commit, expected);
     });
 }
 
@@ -85,7 +92,13 @@
         ctx.create_nonupstream_merge(&["src/tools/foo"]);
 
         let config = parse_config_download_rustc_at(ctx.get_path(), "if-unchanged", true);
-        assert_eq!(config.download_rustc_commit, Some(sha));
+        let expected =
+            if is_download_ci_available(&config.host_target.triple, config.llvm_assertions) {
+                Some(sha)
+            } else {
+                None
+            };
+        assert_eq!(config.download_rustc_commit, expected);
     });
 }
 
@@ -286,6 +299,10 @@
         .join("llvm/bin")
         .join(exe("llvm-config", builder.config.host_target));
     assert_eq!(expected, actual);
+
+    if builder.config.rust_info.is_from_tarball() {
+        return;
+    }
 
     let config = configure(
         r#"
