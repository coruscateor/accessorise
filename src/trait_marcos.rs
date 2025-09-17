
//Traits

#[macro_export]
macro_rules! trait_get
{

    ($name:ident, $name_type:ty) =>
    {

        fn $name(&self) -> $name_type;

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        #[doc = $documentation]
        fn $name(&self) -> $name_type;

    }

}

#[macro_export]
macro_rules! trait_set
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            fn [<set_ $name>](&mut self, value: $name_type);

        }

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<set_ $name>](&mut self, value: $name_type);

        }

    }

}

#[macro_export]
macro_rules! trait_get_set
{

    ($name:ident, $name_type:ty) =>
    {

        trait_get!($name, $name_type);

        trait_set!($name, $name_type);

    };
    ($name:ident, $name_type:ty, $getter_documentation:literal, $setter_documentation:literal) =>
    {

        trait_get!($name, $name_type, $getter_documentation);

        trait_set!($name, $name_type, $setter_documentation);

    }

}

#[macro_export]
macro_rules! trait_get_ref
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            fn [<$name _ref>](&self) -> &$name_type;

        }

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$name _ref>](&self) -> &$name_type;
            
        }

    }

}

#[macro_export]
macro_rules! trait_get_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            fn [<$name _mut>](&mut self) -> &mut $name_type;

        }

    };
    ($name:ident, $name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$name _mut>](&mut self) -> &mut $name_type;

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