--- tests/ui/deprecation/deprecated_no_stack_check.rs
+++ tests/ui/deprecation/deprecated_no_stack_check.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![deny(warnings)]
 #![feature(no_stack_check)]
--- tests/ui/feature-gates/feature-gate-coverage-attribute.rs
+++ tests/ui/feature-gates/feature-gate-coverage-attribute.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![crate_type = "lib"]
 #![feature(no_coverage)] //~ ERROR feature has been removed [E0557]
--- tests/ui/feature-gates/gated-bad-feature.rs
+++ tests/ui/feature-gates/gated-bad-feature.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
 //~^ ERROR malformed `feature`
 //~| ERROR malformed `feature`
--- tests/ui/feature-gates/removed-features-note-version-and-pr-issue-141619.rs
+++ tests/ui/feature-gates/removed-features-note-version-and-pr-issue-141619.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![feature(external_doc)] //~ ERROR feature has been removed
 #![doc(include("README.md"))] //~ ERROR unknown `doc` attribute `include`
--- tests/ui/macros/macro-reexport-removed.rs
+++ tests/ui/macros/macro-reexport-removed.rs
@@ -1,5 +1,5 @@
 //@ aux-build:two_macros.rs
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![feature(macro_reexport)] //~ ERROR feature has been removed
 
--- tests/ui/rustdoc/renamed-features-rustdoc_internals.rs
+++ tests/ui/rustdoc/renamed-features-rustdoc_internals.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![feature(doc_keyword)] //~ ERROR
 #![feature(doc_primitive)] //~ ERROR
--- tests/ui/traits/const-traits/const-trait-impl-parameter-mismatch.rs
+++ tests/ui/traits/const-traits/const-trait-impl-parameter-mismatch.rs
@@ -6,7 +6,7 @@
 // Regression test for issue #125877.
 
 //@ compile-flags: -Znext-solver
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![feature(const_trait_impl, effects)]
 //~^ ERROR feature has been removed
--- tests/ui/unsized-locals/yote.rs
+++ tests/ui/unsized-locals/yote.rs
@@ -1,4 +1,4 @@
-//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
+//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?( \(built from a source tarball\))?" -> "you are using $$RUSTC_VERSION"
 
 #![feature(unsized_locals)] //~ERROR feature has been removed
 #![crate_type = "lib"]
