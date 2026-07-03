--- tests/rustdoc-ui/ice-bug-report-url.rs
+++ tests/rustdoc-ui/ice-bug-report-url.rs
@@ -4,7 +4,7 @@

 //@ normalize-stderr: "note: compiler flags.*\n\n" -> ""
 //@ normalize-stderr: "note: rustc.*running on.*" -> "note: rustc {version} running on {platform}"
-//@ normalize-stderr: "thread.*panicked at compiler.*" -> ""
+//@ normalize-stderr: "thread.*panicked at (?:/rustc(?:-dev)?/[a-z0-9.]+/)?compiler.*" -> ""
 //@ normalize-stderr: " +\d{1,}: .*\n" -> ""
 //@ normalize-stderr: " + at .*\n" -> ""
 //@ normalize-stderr: ".*note: Some details are omitted.*\n" -> ""
