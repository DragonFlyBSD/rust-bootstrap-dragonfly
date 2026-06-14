--- src/bootstrap/src/core/build_steps/dist.rs.orig
+++ src/bootstrap/src/core/build_steps/dist.rs
@@ -27,7 +27,7 @@
 use crate::utils::channel::{self, Info};
 use crate::utils::exec::{BootstrapCommand, command};
 use crate::utils::helpers::{
-    exe, is_dylib, move_file, t, target_supports_cranelift_backend, timeit,
+    exe, hex_encode, is_dylib, move_file, t, target_supports_cranelift_backend, timeit,
 };
 use crate::utils::tarball::{GeneratedTarball, OverlayKind, Tarball};
 use crate::{Compiler, DependencyType, LLVM_TOOLS, Mode};
@@ -44,6 +44,68 @@
     builder.out.join("tmp/dist")
 }
 
+
+fn update_dragonfly_vendor_checksum(builder: &Builder<'_>, crate_dir: &Path, file: &str, contents: &[u8]) {
+    use sha2::Digest;
+
+    let mut hasher = sha2::Sha256::new();
+    hasher.update(contents);
+    let checksum = hex_encode(hasher.finalize().as_slice());
+
+    let checksum_path = crate_dir.join(".cargo-checksum.json");
+    let mut checksum_json: serde_json::Value =
+        t!(serde_json::from_slice(&t!(fs::read(&checksum_path))));
+    let files = checksum_json
+        .get_mut("files")
+        .and_then(|files| files.as_object_mut())
+        .expect("vendor checksum file should contain a files object");
+    files.insert(file.to_string(), serde_json::Value::String(checksum));
+    builder.create(&checksum_path, &t!(serde_json::to_string(&checksum_json)));
+}
+
+fn patch_dragonfly_vendor_openssl_src(builder: &Builder<'_>, plain_dst_src: &Path) {
+    let crate_dir = plain_dst_src.join("vendor/openssl-src-111.28.2+1.1.1w");
+    if !crate_dir.exists() {
+        return;
+    }
+
+    let lib_rs = crate_dir.join("src/lib.rs");
+    let mut contents = t!(fs::read_to_string(&lib_rs));
+    if !contents.contains("no-devcryptoeng") {
+        let needle = "        if target.contains(\"musl\") {\n            // MUSL doesn't implement some of the libc functions that the async\n            // stuff depends on, and we don't bind to any of that in any case.\n            configure.arg(\"no-async\");\n        }\n";
+        let replacement = "        if target.contains(\"dragonfly\") {\n            configure.arg(\"no-devcryptoeng\");\n        }\n\n        if target.contains(\"musl\") {\n            // MUSL doesn't implement some of the libc functions that the async\n            // stuff depends on, and we don't bind to any of that in any case.\n            configure.arg(\"no-async\");\n        }\n";
+
+        if !contents.contains(needle) {
+            panic!("openssl-src crate layout changed; cannot patch DragonFly devcrypto config");
+        }
+        contents = contents.replacen(needle, replacement, 1);
+        builder.create(&lib_rs, &contents);
+    }
+
+    update_dragonfly_vendor_checksum(builder, &crate_dir, "src/lib.rs", contents.as_bytes());
+}
+
+fn patch_dragonfly_vendor_notify(builder: &Builder<'_>, plain_dst_src: &Path) {
+    let crate_dir = plain_dst_src.join("vendor/notify-8.0.0");
+    if !crate_dir.exists() {
+        return;
+    }
+
+    let cargo_toml = crate_dir.join("Cargo.toml");
+    let mut contents = t!(fs::read_to_string(&cargo_toml));
+    if contents.contains("dragonflybsd") {
+        contents = contents.replace("target_os = \"dragonflybsd\"", "target_os = \"dragonfly\"");
+        builder.create(&cargo_toml, &contents);
+    }
+
+    update_dragonfly_vendor_checksum(builder, &crate_dir, "Cargo.toml", contents.as_bytes());
+}
+
+fn patch_dragonfly_vendor_crates(builder: &Builder<'_>, plain_dst_src: &Path) {
+    patch_dragonfly_vendor_openssl_src(builder, plain_dst_src);
+    patch_dragonfly_vendor_notify(builder, plain_dst_src);
+}
+
 fn should_build_extended_tool(builder: &Builder<'_>, tool: &str) -> bool {
     if !builder.config.extended {
         return false;
@@ -1073,6 +1135,7 @@
             let cargo_config_dir = plain_dst_src.join(".cargo");
             builder.create_dir(&cargo_config_dir);
             builder.create(&cargo_config_dir.join("config.toml"), &vendor.config);
+            patch_dragonfly_vendor_crates(builder, plain_dst_src);
         }
 
         // Delete extraneous directories
