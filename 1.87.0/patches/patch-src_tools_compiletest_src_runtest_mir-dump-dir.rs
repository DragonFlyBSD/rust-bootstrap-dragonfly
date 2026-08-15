--- src/tools/compiletest/src/runtest.rs.orig
+++ src/tools/compiletest/src/runtest.rs
@@ -1427,7 +1427,11 @@
         let mut mir_dump_dir = PathBuf::from(self.config.build_base.as_path());
         debug!("input_file: {:?}", self.testpaths.file);
         mir_dump_dir.push(&self.testpaths.relative_dir);
-        mir_dump_dir.push(self.testpaths.file.file_stem().unwrap());
+        let mut dir_name = PathBuf::from(self.testpaths.file.file_stem().unwrap());
+        if let Some(revision) = self.revision {
+            dir_name = dir_name.with_extra_extension(revision);
+        }
+        mir_dump_dir.push(dir_name);
         mir_dump_dir
     }
 
