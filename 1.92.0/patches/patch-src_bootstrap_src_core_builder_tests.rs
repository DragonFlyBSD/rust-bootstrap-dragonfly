--- src/bootstrap/src/core/builder/tests.rs
+++ src/bootstrap/src/core/builder/tests.rs
@@ -7,6 +7,7 @@
 use super::*;
 use crate::Flags;
 use crate::core::build_steps::doc::DocumentationFormat;
+use crate::core::download::is_download_ci_available;
 use crate::core::config::Config;
 use crate::utils::tests::git::{GitCtx, git_test};
 
@@ -276,7 +277,12 @@
         ctx.create_nonupstream_merge(&["library/foo"]);
 
         let config = parse_config_download_rustc_at(ctx.get_path(), "if-unchanged", false);
-        assert_eq!(config.download_rustc_commit, Some(sha));
+        let expected = if is_download_ci_available(&config.host_target.triple, config.llvm_assertions) {
+            Some(sha)
+        } else {
+            None
+        };
+        assert_eq!(config.download_rustc_commit, expected);
     });
 }
 
@@ -289,7 +295,12 @@
         ctx.create_nonupstream_merge(&["src/tools/foo"]);
 
         let config = parse_config_download_rustc_at(ctx.get_path(), "if-unchanged", true);
-        assert_eq!(config.download_rustc_commit, Some(sha));
+        let expected = if is_download_ci_available(&config.host_target.triple, config.llvm_assertions) {
+            Some(sha)
+        } else {
+            None
+        };
+        assert_eq!(config.download_rustc_commit, expected);
     });
 }
 
@@ -1143,6 +1154,10 @@
         .join("llvm/bin")
         .join(exe("llvm-config", builder.config.build));
     assert_eq!(expected, actual);
+
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
 
     let config = configure(
         r#"
