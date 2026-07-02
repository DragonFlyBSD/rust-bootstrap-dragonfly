--- tests/debuginfo/strings-and-strs.rs.orig
+++ tests/debuginfo/strings-and-strs.rs
@@ -8,7 +8,7 @@
 //@ gdb-command:run
 
 //@ gdb-command:print plain_string
-//@ gdb-check:$1 = alloc::string::String {vec: alloc::vec::Vec<u8, alloc::alloc::Global> {buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global> {inner: alloc::raw_vec::RawVecInner<alloc::alloc::Global> {ptr: core::ptr::unique::Unique<u8> {pointer: core::ptr::non_null::NonNull<u8> {pointer: 0x[...]}, _marker: core::marker::PhantomData<u8>}, cap: core::num::niche_types::UsizeNoHighBit (5), alloc: alloc::alloc::Global}, _marker: core::marker::PhantomData<u8>}, len: 5}}
+//@ gdb-check:$1 = "Hello"
 
 //@ gdb-command:print plain_str
 //@ gdb-check:$2 = "Hello"
@@ -20,7 +20,7 @@
 //@ gdb-check:$4 = ("Hello", "World")
 
 //@ gdb-command:print str_in_rc
-//@ gdb-check:$5 = alloc::rc::Rc<&str, alloc::alloc::Global> {ptr: core::ptr::non_null::NonNull<alloc::rc::RcInner<&str>> {pointer: 0x[...]}, phantom: core::marker::PhantomData<alloc::rc::RcInner<&str>>, alloc: alloc::alloc::Global}
+//@ gdb-check:$5 = Rc(strong=1, weak=0) = {value = "Hello", strong = 1, weak = 0}
 
 // === LLDB TESTS ==================================================================================
 //@ lldb-command:run
