use super::*;

use pastey::paste;

#[derive(Default)]
struct TestStruct
{

    a_number: i32,
    a_bool: bool,
    a_string: String,
    a_number_doc: i8,
    a_string_doc: String,
    some_numbers: Vec<i32>,
    some_bools: Vec<bool>

}

impl TestStruct
{

    impl_ref_getter!({a_number, i32}, {a_string, String});

    impl_val_setter!({a_number, i32}, {some_numbers, Vec<i32>});

    impl_val_setter!({a_bool, bool, "Sets a bool"}, {some_bools, Vec<bool>, "Sets some bools"});

    impl_ref_getter!({a_string_doc, String, "Returns a reference to a String."}, {some_numbers, Vec<i32>, "Returns a reference to some numbers."});

    impl_mut_getter!({some_numbers, Vec<i32>, "Returns a mutable reference to some numbers."}, {some_bools, Vec<bool>, "Returns a mutable reference to some bools."}, {a_string_doc, String, "Returns mutable reference to a String."});

    impl_val_clone_getter!({some_numbers, Vec<i32>, ""}, {a_string_doc, String, "Returns a clone of a String."}, {some_bools, Vec<bool>, ""}, {a_string, String, ""});

    //impl_val_setter!(some_numbers, Vec<i32>);

}

#[test]
fn it_works()
{

    //use super::tests::TestStruct;
    
    let mut test_struct = TestStruct::default();

    test_struct.set_a_number(12);

    test_struct.set_some_numbers(vec![3, 4, 78906]);

    //let _number = test_struct.a_number();

    test_struct.set_a_number(5);

    //test_struct.

}