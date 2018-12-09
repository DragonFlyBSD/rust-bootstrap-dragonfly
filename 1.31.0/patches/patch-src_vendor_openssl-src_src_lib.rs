--- src/vendor/openssl-src/src/lib.rs.orig	2018-12-08 18:57:16 UTC
+++ src/vendor/openssl-src/src/lib.rs
@@ -51,6 +51,13 @@ impl Build {
         self
     }
 
+    fn cmd_make(&self) -> Command {
+        match &self.host.as_ref().expect("HOST dir not set")[..] {
+            "x86_64-unknown-dragonfly" => Command::new("gmake"),
+            _ => Command::new("make"),
+        }
+    }
+
     pub fn build(&mut self) -> Artifacts {
         let target = &self.target.as_ref().expect("TARGET dir not set")[..];
         let host = &self.host.as_ref().expect("HOST dir not set")[..];
@@ -161,6 +168,7 @@ impl Build {
             "x86_64-pc-windows-gnu" => "mingw64",
             "x86_64-pc-windows-msvc" => "VC-WIN64A",
             "x86_64-unknown-freebsd" => "BSD-x86_64",
+            "x86_64-unknown-dragonfly" => "BSD-x86_64",
             "x86_64-unknown-linux-gnu" => "linux-x86_64",
             "x86_64-unknown-linux-musl" => "linux-x86_64",
             "x86_64-unknown-netbsd" => "BSD-x86_64",
@@ -264,11 +272,11 @@ impl Build {
             install.arg("install_sw").current_dir(&inner_dir);
             self.run_command(install, "installing OpenSSL");
         } else {
-            let mut depend = Command::new("make");
+            let mut depend = self.cmd_make();
             depend.arg("depend").current_dir(&inner_dir);
             self.run_command(depend, "building OpenSSL dependencies");
 
-            let mut build = Command::new("make");
+            let mut build = self.cmd_make();
             build.current_dir(&inner_dir);
             if !cfg!(windows) {
                 if let Some(s) = env::var_os("CARGO_MAKEFLAGS") {
@@ -277,7 +285,7 @@ impl Build {
             }
             self.run_command(build, "building OpenSSL");
 
-            let mut install = Command::new("make");
+            let mut install = self.cmd_make();
             install.arg("install_sw").current_dir(&inner_dir);
             self.run_command(install, "installing OpenSSL");
         }
