--- src/tools/compiletest/src/runtest.rs.orig	2026-06-11 13:08:16.851080000 +0800
+++ src/tools/compiletest/src/runtest.rs	2026-06-11 13:08:16.871080000 +0800
@@ -1632,6 +1632,13 @@
             }
         }
 
+        if self.config.mode == DebugInfo && self.config.target_cfg().os == "dragonfly" {
+            // DragonFly produces PIE executables by default. GDB 15.1 on
+            // DragonFly currently sets source-line breakpoints at unresolved
+            // PIE-relative addresses for these tests, so use ET_EXEC here.
+            rustc.args(&["-C", "relocation-model=static"]);
+        }
+
         if self.props.remap_src_base {
             rustc.arg(format!(
                 "--remap-path-prefix={}={}",
