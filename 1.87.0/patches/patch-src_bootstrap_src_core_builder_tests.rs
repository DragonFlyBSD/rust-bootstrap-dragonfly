--- src/bootstrap/src/core/builder/tests.rs
+++ src/bootstrap/src/core/builder/tests.rs
@@ -1071,6 +1071,10 @@
         .join(exe("llvm-config", builder.config.build));
     assert_eq!(expected, actual);
 
+    if config.rust_info.is_from_tarball() {
+        return;
+    }
+
     let config = configure(
         r#"
             [llvm]
