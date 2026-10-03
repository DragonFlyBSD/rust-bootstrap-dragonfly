--- src/bootstrap/src/core/config/config.rs.orig
+++ src/bootstrap/src/core/config/config.rs
@@ -882,7 +882,7 @@
 
         let rust_info = git_info(&exec_ctx, omit_git_hash, &src);
 
-        if !is_user_configured_rust_channel && rust_info.is_from_tarball() {
+        if !cfg!(test) && !is_user_configured_rust_channel && rust_info.is_from_tarball() {
             channel = ci_channel.into();
         }
 
@@ -2216,7 +2216,9 @@
 ) {
     let git_info = GitInfo::new(false, src_dir, exec_ctx);
 
-    if git_info.is_from_tarball() && toml.profile.is_none() {
+    // Unit tests provide their own configuration and expect the unprofiled defaults,
+    // independently of whether bootstrap itself was built from a source archive.
+    if !cfg!(test) && git_info.is_from_tarball() && toml.profile.is_none() {
         toml.profile = Some("dist".into());
     }
 
