ba0d31ef6a71c63308ebe258be8b6184b467da67

- Updated the version string to 0.2.0-alpha.

- Renamed the macros module to impl_macros.

- Added the trait_macros module and moved all the trait-centric macros into it.

- Disabled the trait_get_clone macro.



b86532153665ef7a66abbd2a14243452cfba77c9

- Renamed the impl_get macro to impl_get_copy and updated referencing macros accordingly.

- Renamed all macro parameters named $name and $name_type to $field_name and $field_name_type respectively.

- Renamed the impl_trait_get macro to impl_trait_get_copy and updated referencing macros accordingly.

- Renamed the impl_set macro to impl_set_move and updated referencing macros accordingly.

- Renamed the impl_trait_set macro to impl_trait_set_move.

- Renamed the impl_get_set macro to impl_get_copy_set_move.

- Updated the tests module.

- Renamed the trait_get macro to trait_get_val and updated referencing macros accordingly.

- Renamed the trait_set macro to trait_set_val and updated referencing macros accordingly.

- Renamed the trait_get_set macro to trait_get_set_val.



3299851ba4424c1c297bf49dd684e5802ae1ee43

- Disabled the impl_get_copy and impl_trait_get_copy macros.

- Renamed the impl_set_move macro to impl_set_val.

- Renamed the impl_trait_set_move macro to impl_trait_set_val.

- Disabled the impl_get_copy_set_move macro.

- Renamed the impl_get_clone macro to impl_get_val.

- Renamed the impl_set_clone macro to impl_set_val_clone.

- Renamed the impl_trait_set_clone macro to impl_trait_set_val_clone.

- Added the impl_get_set_val macro.

- Added the impl_trait_get_set_val macro.

- Added the impl_get_ref_mut macro.

- Added the impl_trait_get_ref_mut macro.

- Updated the tests module.

- Added the trait_get_ref_mut macro.

- Added the trait_set_val_clone_sig macro.



