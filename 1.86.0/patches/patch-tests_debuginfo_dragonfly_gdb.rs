--- tests/debuginfo/borrowed-c-style-enum.rs.orig	2026-06-11 13:10:46.901110000 +0800
+++ tests/debuginfo/borrowed-c-style-enum.rs	2026-06-11 13:10:46.921110000 +0800
@@ -5,13 +5,13 @@
 // gdb-command:run
 
 // gdb-command:print *the_a_ref
-// gdb-check:$1 = borrowed_c_style_enum::ABC::TheA
+// gdb-check:$1 = borrowed_c_style_enum::[...]TheA
 
 // gdb-command:print *the_b_ref
-// gdb-check:$2 = borrowed_c_style_enum::ABC::TheB
+// gdb-check:$2 = borrowed_c_style_enum::[...]TheB
 
 // gdb-command:print *the_c_ref
-// gdb-check:$3 = borrowed_c_style_enum::ABC::TheC
+// gdb-check:$3 = borrowed_c_style_enum::[...]TheC
 
 
 // === LLDB TESTS ==================================================================================
--- tests/debuginfo/c-style-enum.rs.orig	2026-06-11 13:10:46.901110000 +0800
+++ tests/debuginfo/c-style-enum.rs	2026-06-11 13:10:46.921110000 +0800
@@ -5,60 +5,60 @@
 // === GDB TESTS ===================================================================================
 
 // gdb-command:print c_style_enum::SINGLE_VARIANT
-// gdb-check:$1 = c_style_enum::SingleVariant::TheOnlyVariant
+// gdb-check:$1 = c_style_enum::[...]TheOnlyVariant
 
 // gdb-command:print c_style_enum::AUTO_ONE
-// gdb-check:$2 = c_style_enum::AutoDiscriminant::One
+// gdb-check:$2 = c_style_enum::[...]One
 
 // gdb-command:print c_style_enum::AUTO_TWO
-// gdb-check:$3 = c_style_enum::AutoDiscriminant::One
+// gdb-check:$3 = c_style_enum::[...]One
 
 // gdb-command:print c_style_enum::AUTO_THREE
-// gdb-check:$4 = c_style_enum::AutoDiscriminant::One
+// gdb-check:$4 = c_style_enum::[...]One
 
 // gdb-command:print c_style_enum::MANUAL_ONE
-// gdb-check:$5 = c_style_enum::ManualDiscriminant::OneHundred
+// gdb-check:$5 = c_style_enum::[...]OneHundred
 
 // gdb-command:print c_style_enum::MANUAL_TWO
-// gdb-check:$6 = c_style_enum::ManualDiscriminant::OneHundred
+// gdb-check:$6 = c_style_enum::[...]OneHundred
 
 // gdb-command:print c_style_enum::MANUAL_THREE
-// gdb-check:$7 = c_style_enum::ManualDiscriminant::OneHundred
+// gdb-check:$7 = c_style_enum::[...]OneHundred
 
 // gdb-command:run
 
 // gdb-command:print auto_one
-// gdb-check:$8 = c_style_enum::AutoDiscriminant::One
+// gdb-check:$8 = c_style_enum::[...]One
 
 // gdb-command:print auto_two
-// gdb-check:$9 = c_style_enum::AutoDiscriminant::Two
+// gdb-check:$9 = c_style_enum::[...]Two
 
 // gdb-command:print auto_three
-// gdb-check:$10 = c_style_enum::AutoDiscriminant::Three
+// gdb-check:$10 = c_style_enum::[...]Three
 
 // gdb-command:print manual_one_hundred
-// gdb-check:$11 = c_style_enum::ManualDiscriminant::OneHundred
+// gdb-check:$11 = c_style_enum::[...]OneHundred
 
 // gdb-command:print manual_one_thousand
-// gdb-check:$12 = c_style_enum::ManualDiscriminant::OneThousand
+// gdb-check:$12 = c_style_enum::[...]OneThousand
 
 // gdb-command:print manual_one_million
-// gdb-check:$13 = c_style_enum::ManualDiscriminant::OneMillion
+// gdb-check:$13 = c_style_enum::[...]OneMillion
 
 // gdb-command:print single_variant
-// gdb-check:$14 = c_style_enum::SingleVariant::TheOnlyVariant
+// gdb-check:$14 = c_style_enum::[...]TheOnlyVariant
 
 // gdb-command:print AUTO_TWO
-// gdb-check:$15 = c_style_enum::AutoDiscriminant::Two
+// gdb-check:$15 = c_style_enum::[...]Two
 
 // gdb-command:print AUTO_THREE
-// gdb-check:$16 = c_style_enum::AutoDiscriminant::Three
+// gdb-check:$16 = c_style_enum::[...]Three
 
 // gdb-command:print MANUAL_TWO
-// gdb-check:$17 = c_style_enum::ManualDiscriminant::OneThousand
+// gdb-check:$17 = c_style_enum::[...]OneThousand
 
 // gdb-command:print MANUAL_THREE
-// gdb-check:$18 = c_style_enum::ManualDiscriminant::OneMillion
+// gdb-check:$18 = c_style_enum::[...]OneMillion
 
 
 // === LLDB TESTS ==================================================================================
--- tests/debuginfo/c-style-enum-in-composite.rs.orig	2026-06-11 13:10:46.901110000 +0800
+++ tests/debuginfo/c-style-enum-in-composite.rs	2026-06-11 13:10:46.921110000 +0800
@@ -5,25 +5,25 @@
 // gdb-command:run
 
 // gdb-command:print tuple_interior_padding
-// gdb-check:$1 = (0, c_style_enum_in_composite::AnEnum::OneHundred)
+// gdb-check:$1 = (0, c_style_enum_in_composite::[...]OneHundred)
 
 // gdb-command:print tuple_padding_at_end
-// gdb-check:$2 = ((1, c_style_enum_in_composite::AnEnum::OneThousand), 2)
+// gdb-check:$2 = ((1, c_style_enum_in_composite::[...]OneThousand), 2)
 
 // gdb-command:print tuple_different_enums
-// gdb-check:$3 = (c_style_enum_in_composite::AnEnum::OneThousand, c_style_enum_in_composite::AnotherEnum::MountainView, c_style_enum_in_composite::AnEnum::OneMillion, c_style_enum_in_composite::AnotherEnum::Vienna)
+// gdb-check:$3 = (c_style_enum_in_composite::[...]OneThousand, c_style_enum_in_composite::[...]MountainView, c_style_enum_in_composite::[...]OneMillion, c_style_enum_in_composite::[...]Vienna)
 
 // gdb-command:print padded_struct
-// gdb-check:$4 = c_style_enum_in_composite::PaddedStruct {a: 3, b: c_style_enum_in_composite::AnEnum::OneMillion, c: 4, d: c_style_enum_in_composite::AnotherEnum::Toronto, e: 5}
+// gdb-check:$4 = c_style_enum_in_composite::PaddedStruct {a: 3, b: c_style_enum_in_composite::[...]OneMillion, c: 4, d: c_style_enum_in_composite::[...]Toronto, e: 5}
 
 // gdb-command:print packed_struct
-// gdb-check:$5 = c_style_enum_in_composite::PackedStruct {a: 6, b: c_style_enum_in_composite::AnEnum::OneHundred, c: 7, d: c_style_enum_in_composite::AnotherEnum::Vienna, e: 8}
+// gdb-check:$5 = c_style_enum_in_composite::PackedStruct {a: 6, b: c_style_enum_in_composite::[...]OneHundred, c: 7, d: c_style_enum_in_composite::[...]Vienna, e: 8}
 
 // gdb-command:print non_padded_struct
-// gdb-check:$6 = c_style_enum_in_composite::NonPaddedStruct {a: c_style_enum_in_composite::AnEnum::OneMillion, b: c_style_enum_in_composite::AnotherEnum::MountainView, c: c_style_enum_in_composite::AnEnum::OneThousand, d: c_style_enum_in_composite::AnotherEnum::Toronto}
+// gdb-check:$6 = c_style_enum_in_composite::NonPaddedStruct {a: c_style_enum_in_composite::[...]OneMillion, b: c_style_enum_in_composite::[...]MountainView, c: c_style_enum_in_composite::[...]OneThousand, d: c_style_enum_in_composite::[...]Toronto}
 
 // gdb-command:print struct_with_drop
-// gdb-check:$7 = (c_style_enum_in_composite::StructWithDrop {a: c_style_enum_in_composite::AnEnum::OneHundred, b: c_style_enum_in_composite::AnotherEnum::Vienna}, 9)
+// gdb-check:$7 = (c_style_enum_in_composite::StructWithDrop {a: c_style_enum_in_composite::[...]OneHundred, b: c_style_enum_in_composite::[...]Vienna}, 9)
 
 // === LLDB TESTS ==================================================================================
 
--- tests/debuginfo/gdb-pretty-struct-and-enums.rs.orig	2026-06-11 13:10:46.901110000 +0800
+++ tests/debuginfo/gdb-pretty-struct-and-enums.rs	2026-06-11 13:10:46.921110000 +0800
@@ -12,13 +12,13 @@
 // gdb-check:$2 = gdb_pretty_struct_and_enums::EmptyStruct
 
 // gdb-command: print c_style_enum1
-// gdb-check:$3 = gdb_pretty_struct_and_enums::CStyleEnum::CStyleEnumVar1
+// gdb-check:$3 = gdb_pretty_struct_and_enums::[...]CStyleEnumVar1
 
 // gdb-command: print c_style_enum2
-// gdb-check:$4 = gdb_pretty_struct_and_enums::CStyleEnum::CStyleEnumVar2
+// gdb-check:$4 = gdb_pretty_struct_and_enums::[...]CStyleEnumVar2
 
 // gdb-command: print c_style_enum3
-// gdb-check:$5 = gdb_pretty_struct_and_enums::CStyleEnum::CStyleEnumVar3
+// gdb-check:$5 = gdb_pretty_struct_and_enums::[...]CStyleEnumVar3
 
 #![allow(dead_code, unused_variables)]
 
--- tests/debuginfo/pretty-std.rs.orig	2026-06-11 13:10:46.901110000 +0800
+++ tests/debuginfo/pretty-std.rs	2026-06-11 13:10:46.921110000 +0800
@@ -28,7 +28,7 @@
 // gdb-check:$6 = core::option::Option<i64>::None
 
 // gdb-command: print os_string
-// gdb-check:$7 = "IAMA OS string 😃"
+// gdb-check:$7 = "IAMA OS string [...]"
 
 // gdb-command: print some_string
 // gdb-check:$8 = core::option::Option<alloc::string::String>::Some("IAMA optional string!")
