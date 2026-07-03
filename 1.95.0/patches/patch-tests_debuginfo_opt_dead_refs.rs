--- tests/debuginfo/opt/dead_refs.rs
+++ tests/debuginfo/opt/dead_refs.rs
@@ -2,6 +2,7 @@
 //@ min-gdb-version: 13.0
 //@ compile-flags: -g -Copt-level=3
 //@ disable-gdb-pretty-printers
+//@ ignore-dragonfly: DragonFly GDB reports optimized dead references as optimized out.

 // Checks that we still can access dead variables from debuginfos.
