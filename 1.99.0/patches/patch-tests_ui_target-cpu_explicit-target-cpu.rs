--- tests/ui/target-cpu/explicit-target-cpu.rs.orig
+++ tests/ui/target-cpu/explicit-target-cpu.rs
@@ -9,6 +9,9 @@
 //@[amdgcn_cpu] compile-flags: --target=amdgcn-amd-amdhsa
 //@[amdgcn_cpu] needs-llvm-components: amdgpu
 //@[amdgcn_cpu] compile-flags: -Ctarget-cpu=gfx900
+// DragonFly compiletest passes the host C compiler as target linker, but this
+// amdgcn target needs the bundled lld linker.
+//@[amdgcn_cpu] compile-flags: -Clinker={{sysroot-base}}/lib/rustlib/{{target}}/bin/rust-lld
 //@[amdgcn_cpu] build-pass
 
 //@ revisions: avr_nocpu avr_cpu
