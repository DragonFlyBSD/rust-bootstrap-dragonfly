--- src/bootstrap/src/core/builder/tests.rs.orig
+++ src/bootstrap/src/core/builder/tests.rs
@@ -10,6 +10,7 @@
 use crate::core::build_steps::doc::DocumentationFormat;
 use crate::core::builder::cli_paths::PATH_REMAP;
 use crate::core::config::Config;
+use crate::core::download::is_download_ci_available;
 use crate::utils::cache::ExecutedStep;
 use crate::utils::helpers::get_host_target;
 use crate::utils::tests::git::{GitCtx, git_test};
@@ -186,7 +187,12 @@
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
 
@@ -199,7 +205,12 @@
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
 
@@ -410,6 +421,10 @@
     let actual = drop_win_disk_prefix_if_present(actual);
     assert_eq!(expected, actual);
     assert_eq!(expected, actual);
+
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
 
     let config = configure(
         r#"
