--- src/bootstrap/src/core/build_steps/dist.rs.orig
+++ src/bootstrap/src/core/build_steps/dist.rs
@@ -26,7 +26,7 @@
 use crate::utils::channel::{self, Info};
 use crate::utils::exec::{BootstrapCommand, command};
 use crate::utils::helpers::{
-    exe, is_dylib, move_file, t, target_supports_cranelift_backend, timeit,
+    exe, hex_encode, is_dylib, move_file, t, target_supports_cranelift_backend, timeit,
 };
 use crate::utils::tarball::{GeneratedTarball, OverlayKind, Tarball};
 use crate::{Compiler, DependencyType, LLVM_TOOLS, Mode};
@@ -43,6 +43,45 @@
     builder.out.join("tmp/dist")
 }
 
+fn patch_dragonfly_vendor_openssl_src(builder: &Builder<'_>, plain_dst_src: &Path) {
+    use sha2::Digest;
+
+    let crate_dir = plain_dst_src.join("vendor/openssl-src-111.28.2+1.1.1w");
+    if !crate_dir.exists() {
+        return;
+    }
+
+    let lib_rs = crate_dir.join("src/lib.rs");
+    let mut contents = t!(fs::read_to_string(&lib_rs));
+    if !contents.contains("no-devcryptoeng") {
+        let needle = "        if target.contains(\"musl\") || target.contains(\"windows\") {\n            // Engine module fails to compile on musl (it needs linux/version.h\n            // right now) but we don't actually need this most of the time.\n            // API of engine.c ld fail in Windows.\n            // Disable engine module unless force-engine feature specified\n            if !cfg!(feature = \"force-engine\") {\n                configure.arg(\"no-engine\");\n            }\n        }\n\n";
+        let replacement = "        if target.contains(\"musl\") || target.contains(\"windows\") {\n            // Engine module fails to compile on musl (it needs linux/version.h\n            // right now) but we don't actually need this most of the time.\n            // API of engine.c ld fail in Windows.\n            // Disable engine module unless force-engine feature specified\n            if !cfg!(feature = \"force-engine\") {\n                configure.arg(\"no-engine\");\n            }\n        }\n\n        if target.contains(\"dragonfly\") {\n            configure.arg(\"no-devcryptoeng\");\n        }\n\n";
+
+        if !contents.contains(needle) {
+            panic!("openssl-src crate layout changed; cannot patch DragonFly devcrypto config");
+        }
+        contents = contents.replacen(needle, replacement, 1);
+        builder.create(&lib_rs, &contents);
+    }
+
+    let mut hasher = sha2::Sha256::new();
+    hasher.update(contents.as_bytes());
+    let lib_rs_checksum = hex_encode(hasher.finalize().as_slice());
+
+    let checksum_path = crate_dir.join(".cargo-checksum.json");
+    let mut checksum_json: serde_json::Value =
+        t!(serde_json::from_slice(&t!(fs::read(&checksum_path))));
+    let files = checksum_json
+        .get_mut("files")
+        .and_then(|files| files.as_object_mut())
+        .expect("openssl-src checksum file should contain a files object");
+    files.insert(
+        "src/lib.rs".to_string(),
+        serde_json::Value::String(lib_rs_checksum),
+    );
+    builder.create(&checksum_path, &t!(serde_json::to_string(&checksum_json)));
+}
+
 fn should_build_extended_tool(builder: &Builder<'_>, tool: &str) -> bool {
     if !builder.config.extended {
         return false;
@@ -1063,6 +1102,7 @@
             let cargo_config_dir = plain_dst_src.join(".cargo");
             builder.create_dir(&cargo_config_dir);
             builder.create(&cargo_config_dir.join("config.toml"), &config);
+            patch_dragonfly_vendor_openssl_src(builder, plain_dst_src);
         }
 
         // Delete extraneous directories
