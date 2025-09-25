
//Impl Blocks

#[macro_export]
macro_rules! impl_get_copy
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        pub fn $field_name(&self) -> $field_name_type
        {

            self.$field_name

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        #[doc = $documentation]
        pub fn $field_name(&self) -> $field_name_type
        {

            self.$field_name

        }

    }

}

//Impl Trait

#[macro_export]
macro_rules! impl_trait_get_copy
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        fn $field_name(&self) -> $field_name_type
        {

            self.$field_name

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        #[doc = $documentation]
        fn $field_name(&self) -> $field_name_type
        {

            self.$field_name

        }

    }

}

//Set

#[macro_export]
macro_rules! impl_set_move
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn [<set_ $field_name>](&mut self, value: $field_name_type)
            {

                self.$field_name = value;

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            pub fn [<set_ $field_name>](&mut self, value: $field_name_type)
            {

                self.$field_name = value;

            }

        }

    }

}

//Impl Trait

#[macro_export]
macro_rules! impl_trait_set_move
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<set_ $field_name>](&mut self, value: $field_name_type)
            {

                self.$field_name = value;

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<set_ $field_name>](&mut self, value: $field_name_type)
            {

                self.$field_name = value;

            }

        }

    }

}

#[macro_export]
macro_rules! impl_get_copy_set_move
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        impl_get_copy!($field_name, $field_name_type);

        impl_set_move!($field_name, $field_name_type);

    };
    ($field_name:ident, $field_name_type:ty, $getter_documentation:literal, $setter_documentation:literal) =>
    {

        impl_gett_copy!($field_name, $field_name_type, $getter_documentation);

        impl_set_move!($field_name, $field_name_type, $setter_documentation:);

    }

}

//References

//Ref

#[macro_export]
macro_rules! impl_get_ref
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn [<$field_name _ref>](&self) -> &$field_name_type
            {

                &self.$field_name

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            pub fn [<$field_name _ref>](&self) -> &$field_name_type
            {

                &self.$field_name

            }

        }

    }

}

//Impl Trait

#[macro_export]
macro_rules! impl_trait_get_ref
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<$field_name _ref>](&self) -> &$field_name_type
            {

                &self.$field_name

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$field_name _ref>](&self) -> &$field_name_type
            {

                &self.$field_name

            }

        }

    }

}

//Mut

#[macro_export]
macro_rules! impl_get_mut
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn [<$field_name _mut>](&mut self) -> &mut $field_name_type
            {

                &mut self.$field_name

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            pub fn [<$field_name _mut>](&mut self) -> &mut $field_name_type
            {

                &mut self.$field_name

            }

        }

    }

}

//Impl Trait

//Getter

#[macro_export]
macro_rules! impl_trait_get_mut
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<$field_name _mut>](&mut self) -> &mut $field_name_type
            {

                &mut self.$field_name

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<$field_name _mut>](&mut self) -> &mut $field_name_type
            {

                &mut self.$field_name

            }

        }

    }

}

//Cloning

#[macro_export]
macro_rules! impl_get_clone
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn $field_name(&self) -> $field_name_type
            {

                self.$field_name.clone()

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            pub fn $field_name(&self) -> $field_name_type
            {

                self.$field_name.clone()

            }
            
        }

    }

}

//Impl Trait

#[macro_export]
macro_rules! impl_trait_get_clone
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn $field_name(&self) -> $field_name_type
            {

                self.$field_name.clone()

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn $field_name(&self) -> $field_name_type
            {

                self.$field_name.clone()

            }
            
        }

    }

}

//Setter

#[macro_export]
macro_rules! impl_set_clone
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn [<set_ $field_name>](&mut self, value: &$field_name_type)
            {

                self.$field_name = value.clone();

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            pub fn [<set_ $field_name>](&mut self, value: &$field_name_type)
            {

                self.$field_name = value.clone();

            }

        }

    }

}

//Impl Trait

#[macro_export]
macro_rules! impl_trait_set_clone
{

    ($field_name:ident, $field_name_type:ty) =>
    {

        paste!
        {

            fn [<set_ $field_name>](&mut self, value: &$field_name_type)
            {

                self.$field_name = value.clone();

            }

        }

    };
    ($field_name:ident, $field_name_type:ty, $documentation:literal) =>
    {

        paste!
        {

            #[doc = $documentation]
            fn [<set_ $field_name>](&mut self, value: &$field_name_type)
            {

                self.$field_name = value.clone();

            }

        }

    }

}

/*
#[macro_export]
macro_rules! impl_aliased_get_ref
{

    ($field:ident, $alias:ident, $field_name_type:ty) =>
    {

        pub fn $alias(&self) -> &$field_name_type
        {

            &self.$field

        }

    }

}

#[macro_export]
macro_rules! impl_aliased_get_mut
{

    ($field:ident, $alias:ident, $field_name_type:ty) =>
    {

        paste!
        {

            pub fn $alias(&mut self) -> &mut $field_name_type
            {

                &mut self.$field

            }

        }

    }

}
*/






