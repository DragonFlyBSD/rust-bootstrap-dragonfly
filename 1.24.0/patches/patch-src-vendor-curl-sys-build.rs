--- src/vendor/curl-sys/build.rs.orig
+++ src/vendor/curl-sys/build.rs
@@ -172,7 +172,7 @@ fn main() {
     }
 
     cmd.arg("--without-librtmp");
-    cmd.arg("--without-libidn");
+    cmd.arg("--without-libidn2");
     cmd.arg("--without-libssh2");
     cmd.arg("--without-libpsl");
     cmd.arg("--disable-ldap");
@@ -224,7 +224,7 @@ fn fail(s: &str) -> ! {
 }
 
 fn make() -> Command {
-    let cmd = if cfg!(target_os = "freebsd") {"gmake"} else {"make"};
+    let cmd = if cfg!(target_os = "dragonfly") {"gmake"} else {"make"};
     let mut cmd = Command::new(cmd);
     // We're using the MSYS make which doesn't work with the mingw32-make-style
     // MAKEFLAGS, so remove that from the env if present.

