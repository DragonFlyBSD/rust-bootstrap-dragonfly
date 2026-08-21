--- compiler/rustc_codegen_ssa/src/back/linker.rs	2025-03-16 00:27:19.000000000 +0800
+++ compiler/rustc_codegen_ssa/src/back/linker.rs	2026-06-11 14:28:13.682043000 +0800
@@ -816,11 +816,21 @@
             let res: io::Result<()> = try {
                 let mut f = File::create_buffered(&path)?;
                 writeln!(f, "{{")?;
-                if !symbols.is_empty() {
+                let preserve_dragonfly_startup_symbols = crate_type == CrateType::Executable
+                    && self.sess.target.os == "dragonfly";
+                if !symbols.is_empty() || preserve_dragonfly_startup_symbols {
                     writeln!(f, "  global:")?;
                     for sym in symbols {
                         debug!("    {sym};");
                         writeln!(f, "    {sym};")?;
+                    }
+                    if preserve_dragonfly_startup_symbols {
+                        // DragonFly startup objects define process globals for DSOs; hiding
+                        // them with local: * makes ld.bfd reject PIE executable links.
+                        for sym in ["__progname", "environ"] {
+                            debug!("    {sym};");
+                            writeln!(f, "    {sym};")?;
+                        }
                     }
                 }
                 writeln!(f, "\n  local:\n    *;\n}};")?;
@@ -840,7 +848,10 @@
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
 
