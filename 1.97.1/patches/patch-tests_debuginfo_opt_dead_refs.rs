--- tests/debuginfo/opt/dead_refs.rs.orig
+++ tests/debuginfo/opt/dead_refs.rs
@@ -3,6 +3,7 @@
 //@ min-gdb-version: 13.0
 //@ compile-flags: -g -Copt-level=3
 //@ disable-gdb-pretty-printers
+//@ ignore-dragonfly: DragonFly GDB reports optimized dead references as optimized out.
 
 // Checks that we still can access dead variables from debuginfos.
 
