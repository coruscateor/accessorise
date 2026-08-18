# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (18/08/2026)

### Added

- Added repetition rules to the impl_val_getter_setter macro.

- Added repetition rules to the impl_trait_val_getter_setter macro.

- Added repetition rules to the impl_ref_mut_getters macro.

- Added repetition rules to the impl_trait_ref_mut_getters macro.

- Added repetition rules to the trait_val_getter_setter macro.

- Added repetition rules to the trait_ref_getter macro.

- Added repetition rules to the trait_mut_getter macro.

- Added repetition rules to the trait_ref_mut_getters macro.

- Added repetition rules to the trait_val_clone_setter macro.

- Added two repetition rules to the impl_val_clone_setter macro.

- Added two repetition rules to the impl_trait_val_clone_setter macro.

- Added two repetition rules to the trait_val_getter macro.

- Added two repetition rules to the trait_val_setter macro.

- Added repetition rules to the new impl_val_getter macro.

- Added repetition rules to the impl_val_clone_getter macro.

- Added the impl_trait_val_getter macro.

- Added repetition rules to the impl_trait_val_clone_getter macro.

- Added repetition rules to the impl_ref_getter macro.

- Added repetition rules to the impl_trait_ref_getter macro.

- Added repetition rules to the impl_mut_getter macro.

- Added repetition rules to the impl_trait_mut_getter macro.

- Added a new impl_val_getter macro.

- Added the pastey dev-dependency and updated the project accordingly.

- Added a rule to the impl_val_setter macro which takes an ident parameter and a ty parameter in curly braces, requiring one or more repetitions in this format, outputting setter method definitions.

- Added a rule to the impl_val_setter macro which takes an ident parameter, a ty parameter and a literal parameter in curly braces, requiring one or more repetitions in this format, outputting documented setter method definitions.

- Added a rule to the impl_trait_val_setter macro which takes an ident parameter and a ty parameter in curly braces, requiring one or more repetitions in this format, outputting setter method definitions.

- Added a rule to the impl_trait_val_setter macro which takes an ident parameter, a ty parameter and a literal parameter in curly braces, requiring one or more repetitions in this format, outputting documented setter method definitions.

- Added the tests test module.

- Added the repetition_tests test module.

- Added an authors key with the value "Paul Saunders" to the package sec…tion of the cargo.toml file.



### Changed

- Updated the readme.

- Renamed the impl_set_val_clone macro to impl_val_clone_setter.

- Renamed the impl_trait_set_val_clone macro to impl_trait_val_clone_setter.

- Renamed the original impl_trait_val_getter macro to impl_trait_val_clone_getter.

- Renamed the original impl_val_getter macro to impl_val_clone_getter and updated the project accordingly.

- Moved the contents of the tests lib-module into the newly added tests module.

- Updated the macro invocations in the impl_val_getter_setter macro.

- Updated the macro invocations in the impl_trait_val_getter_setter macro.

- Updated the macro invocations in the impl_ref_mut_getters macro.

- Updated the macro invocations in the impl_trait_ref_mut_getters macro.

- Updated the tests module to use the new macro names.

- Updated the macro invocations in the trait_val_getter_setter macro.

- Renamed the trait_get_ref_mut macro to trait_ref_mut_getters.

- Renamed the impl_set_val macro to impl_val_setter.

- Renamed the impl_trait_set_val macro to impl_trait_val_setter.

- Renamed the impl_get_ref macro to impl_ref_getter.

- Renamed the impl_trait_get_ref macro to impl_trait_ref_getter.

- Renamed the impl_get_mut macro to impl_mut_getter.

- Renamed the impl_trait_get_mut macro to impl_trait_mut_getter.

- Renamed the impl_get_val macro to impl_val_getter.

- Renamed the impl_trait_get_val macro to impl_trait_val_getter.

- Renamed the impl_trait_get_set_val macro to impl_trait_val_getter_setter.

- Renamed the impl_get_ref_mut macro to impl_ref_mut_getters.

- Renamed the impl_trait_get_ref_mut macro to impl_trait_ref_mut_getters.

- Renamed the trait_get_val macro to trait_val_getter.

- Renamed the trait_set_val macro to trait_val_setter.

- Renamed the trait_get_set_val macro to trait_val_getter_setter.

- Renamed the trait_get_ref macro to trait_ref_getter.

- Renamed the trait_get_mut macro to trait_mut_getter.



### Removed

- Removed the paste dev-dependency.

- Removed the tests lib-module.



## Version 0.2.0 (15/10/2025)

Added

- Added the trait_macros module and moved all the trait-centric macros into it.

- Added the impl_get_set_val macro.

- Added the impl_trait_get_set_val macro.

- Added the impl_get_ref_mut macro.

- Added the impl_trait_get_ref_mut macro.

- Added the trait_get_ref_mut macro.

- Added the trait_set_val_clone_sig macro.



Changed

- Renamed the macros module to impl_macros.

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

- Renamed the impl_set_move macro to impl_set_val.

- Renamed the impl_trait_set_move macro to impl_trait_set_val.

- Renamed the impl_get_clone macro to impl_get_val.

- Renamed the impl_set_clone macro to impl_set_val_clone.

- Renamed the impl_trait_set_clone macro to impl_trait_set_val_clone.

- Updated the Readme.

- Updated the project description.

- Cleaned up the code a bit.



Removed

- Removed the impl_get_copy and the impl_trait_get_copy macros.

- Removed the impl_get_copy_set_move macro.

- Removed the trait_get_clone macro.



## Version 0.1.0 (17/04/2025)

- Initial release
