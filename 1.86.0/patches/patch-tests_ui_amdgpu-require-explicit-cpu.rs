--- tests/ui/amdgpu-require-explicit-cpu.rs.orig	2025-04-03 01:01:04.000000000 +0800
+++ tests/ui/amdgpu-require-explicit-cpu.rs	2026-06-12 01:26:00.000000000 +0800
@@ -6,6 +6,9 @@
 //@[nocpu] error-pattern: target requires explicitly specifying a cpu
 //@[nocpu] build-fail
 //@[cpu] compile-flags: -Ctarget-cpu=gfx900
+// DragonFly compiletest passes the host C compiler as target linker, but this
+// amdgcn target needs the bundled lld linker.
+//@[cpu] compile-flags: -Clinker={{sysroot-base}}/lib/rustlib/{{target}}/bin/rust-lld
 //@[cpu] build-pass

 #![feature(no_core, lang_items)]
