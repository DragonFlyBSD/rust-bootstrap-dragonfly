--- tests/run-make/embed-source-dwarf/rmake.rs.orig
+++ tests/run-make/embed-source-dwarf/rmake.rs
@@ -65,6 +65,7 @@
     }
 
     dbg!(&sources);
-    assert_eq!(sources.len(), 1);
+    // Available standard-library sources may also be embedded in the executable.
+    // Check the input source without assuming it is the only source present.
     assert_eq!(sources.get("main.rs").unwrap(), "// hello\nfn main() {}\n");
 }
