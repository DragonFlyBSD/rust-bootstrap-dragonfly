--- tests/ui/panics/panic-abort-backtrace-without-debuginfo.rs
+++ tests/ui/panics/panic-abort-backtrace-without-debuginfo.rs
@@ -16,6 +16,7 @@
 //@ ignore-apple
 //@ ignore-arm-unknown-linux-gnueabihf FIXME(#146996) Try removing this once #146996 has been fixed.
 //@ ignore-msvc Backtraces on Windows requires debuginfo which we can't use here
+//@ ignore-dragonfly No-debuginfo abort backtraces omit the caller frame.
 //@ ignore-backends: gcc

 static FN_1: &str = "this_function_must_be_in_the_backtrace";
