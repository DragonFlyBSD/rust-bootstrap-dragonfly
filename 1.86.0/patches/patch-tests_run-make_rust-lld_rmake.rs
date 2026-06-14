--- tests/run-make/rust-lld/rmake.rs	2025-03-16 00:27:19.000000000 +0800
+++ tests/run-make/rust-lld/rmake.rs	2026-06-11 14:52:42.992337000 +0800
@@ -2,6 +2,9 @@
 // see https://github.com/rust-lang/compiler-team/issues/510 for more info
 
 //@ needs-rust-lld
+//@ ignore-dragonfly
+// Reason: DragonFly does not currently route the cc linker flavor through
+// bundled rust-lld.
 //@ ignore-s390x lld does not yet support s390x as target
 
 use run_make_support::regex::Regex;
