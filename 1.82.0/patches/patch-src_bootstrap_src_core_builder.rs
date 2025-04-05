--- src/bootstrap/src/core/builder.rs.orig	2024-10-15 17:22:50 UTC
+++ src/bootstrap/src/core/builder.rs
@@ -1109,7 +1109,7 @@ impl<'a> Builder<'a> {
     /// Returns if `std` should be statically linked into `rustc_driver`.
     /// It's currently not done on `windows-gnu` due to linker bugs.
     pub fn link_std_into_rustc_driver(&self, target: TargetSelection) -> bool {
-        !target.triple.ends_with("-windows-gnu")
+        !target.triple.ends_with("-windows-gnu") && !target.triple.ends_with("dragonfly")
     }
 
     /// Obtain a compiler at a given stage and for a given host (i.e., this is the target that the
