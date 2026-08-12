use super::*;

use pastey::paste;

trait TestTrait
{

    trait_val_getter!(a_number, i32);

    trait_val_setter!(a_number, i32);

    trait_ref_getter!(a_string, String);

    trait_mut_getter!(a_string, String);

    trait_val_getter!(a_number_doc, i8, "This is a getter declaration, presumably for a number field.");

    trait_val_setter!(a_number_doc, i8, "This is a setter declaration, presumably for a number field.");

    trait_ref_getter!(a_string_doc, String, "This is a getter declaration, presumably for a String field.");

    trait_mut_getter!(a_string_doc, String, "This is a setter declaration, presumably for a String field.");

}

#[derive(Default)]
struct TestStruct
{

    a_number: i32,
    a_string: String,
    a_number_doc: i8,
    a_string_doc: String,
    some_numbers: Vec<i32>

}

impl TestStruct
{

    impl_ref_getter!(a_number, i32);

    impl_val_setter!(a_number, i32);

    impl_ref_getter!(a_string, String);

    impl_ref_getter!(a_string_doc, String, "Returns a cloned String.");

    impl_ref_getter!(some_numbers, Vec<i32>, "Returns some numbers by reference.");

    impl_mut_getter!(some_numbers, Vec<i32>, "Returns some numbers by mutable reference.");

    impl_val_clone_getter!(some_numbers, Vec<i32>);

    impl_val_setter!(some_numbers, Vec<i32>);

}

impl TestTrait for TestStruct
{

    impl_trait_val_getter!(a_number, i32);

    impl_trait_val_setter!(a_number, i32);

    impl_trait_ref_getter!(a_string, String);

    impl_trait_mut_getter!(a_string, String);

    impl_trait_val_getter!(a_number_doc, i8, "This is a getter implementation for a number field.");

    impl_trait_val_setter!(a_number_doc, i8, "This is a setter implementation for a number field.");

    impl_trait_ref_getter!(a_string_doc, String, "This is a getter implementation for a String field.");

    impl_trait_mut_getter!(a_string_doc, String, "This is a setter implementation for a String field.");
    
}

#[test]
fn it_works()
{
    
    let mut test_struct = TestStruct::default();

    let _number = test_struct.a_number();

    test_struct.set_a_number(5);

    //test_struct.

}

