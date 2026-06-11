--- compiler/rustc_codegen_ssa/src/back/linker.rs	2025-03-16 00:27:19.000000000 +0800
+++ compiler/rustc_codegen_ssa/src/back/linker.rs	2026-06-11 14:28:13.682043000 +0800
@@ -840,7 +840,10 @@
         } else {
             let mut arg = OsString::from("--version-script=");
             arg.push(path);
-            self.link_arg(arg).link_arg("--no-undefined-version");
+            self.link_arg(arg);
+            if self.sess.target.os != "dragonfly" {
+                self.link_arg("--no-undefined-version");
+            }
         }
     }
 
