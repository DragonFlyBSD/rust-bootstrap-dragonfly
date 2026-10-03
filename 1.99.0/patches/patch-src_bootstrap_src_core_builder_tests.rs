--- src/bootstrap/src/core/builder/tests.rs.orig
+++ src/bootstrap/src/core/builder/tests.rs
@@ -6,6 +6,7 @@
 
 use super::*;
 use crate::core::config::Config;
+use crate::core::download::is_download_ci_available;
 use crate::utils::cache::ExecutedStep;
 use crate::utils::helpers::get_host_target;
 use crate::utils::tests::git::{GitCtx, git_test};
@@ -71,7 +72,13 @@
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
 
@@ -84,7 +91,13 @@
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
 
@@ -292,6 +305,10 @@
     let actual = drop_win_disk_prefix_if_present(actual);
     assert_eq!(expected, actual);
     assert_eq!(expected, actual);
+
+    if builder.config.rust_info.is_from_tarball() {
+        return;
+    }
 
     let config = configure(
         r#"
