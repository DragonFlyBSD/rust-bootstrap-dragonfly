--- src/tools/compiletest/src/runtest.rs
+++ src/tools/compiletest/src/runtest.rs
@@ -1824,6 +1824,13 @@
             }
         }

+        if self.config.mode == TestMode::DebugInfo && self.config.target_cfg().os == "dragonfly" {
+            // DragonFly produces PIE executables by default. GDB 15.1 on
+            // DragonFly currently sets source-line breakpoints at unresolved
+            // PIE-relative addresses for these tests, so use ET_EXEC here.
+            compiler.args(&["-C", "relocation-model=static"]);
+        }
+
         if self.props.remap_src_base {
             compiler.arg(format!(
                 "--remap-path-prefix={}={}",
