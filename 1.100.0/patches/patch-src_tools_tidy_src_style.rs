--- src/tools/tidy/src/style.rs.orig
+++ src/tools/tidy/src/style.rs
@@ -327,7 +327,7 @@
 fn check_file_style(base_path: &Path, check: &mut RunningCheck, file: &Path, contents: &str) {
     // In some cases, a style check would be triggered by its own implementation
     // or comments. A simple workaround is to just allowlist this file.
-    let this_file = Path::new(file!());
+    let this_file = Path::new("src/tools/tidy/src/style.rs");
     let codegen_file = Path::new("src/tools/tidy/src/codegen.rs");
 
     let path_str = file.to_string_lossy();
