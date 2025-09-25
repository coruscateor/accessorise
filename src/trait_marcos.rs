
//Traits

#[macro_export]
macro_rules! trait_get_val
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        fn $field_name(&self) -> $field_name_type;

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        #[doc = $documentation]
        fn $field_name(&self) -> $field_name_type;

    }

}

#[macro_export]
macro_rules! trait_set_val
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<set_ $field_name>](&mut self, value: $field_name_type);

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<set_ $field_name>](&mut self, value: $field_name_type);

        }

    }

}

#[macro_export]
macro_rules! trait_get_set_val
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        trait_get_val!($field_name, $field_name_type);

        trait_set_val!($field_name, $field_name_type);

    };
    ($field_name:ident, $field_name_type:ty, $getter_documentation:literal, $setter_documentation:literal) =>
    {

        trait_get_val!($field_name, $field_name_type, $getter_documentation);

        trait_set_val!($field_name, $field_name_type, $setter_documentation);

    }

}

#[macro_export]
macro_rules! trait_get_ref
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<$field_name _ref>](&self) -> &$field_name_type;

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$field_name _ref>](&self) -> &$field_name_type;
            
        }

    }

}

#[macro_export]
macro_rules! trait_get_mut
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<$field_name _mut>](&mut self) -> &mut $field_name_type;

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$field_name _mut>](&mut self) -> &mut $field_name_type;

        }

    }

}

//Cloning

//Getter

//Disbaled

/*

#[macro_export]
macro_rules! trait_get_clone
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            fn $name(&self) -> $name_type;

        }

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn $name(&self) -> $name_type;
            
        }

    }

}

*/

//Setter

/*
#[macro_export]
macro_rules! trait_set_clone
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            fn [<set_ $name _clone>](&mut self, value: &$name_type);

        }

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<set_ $name _clone>](&mut self, value: &$name_type);

        }

    }

}
*/