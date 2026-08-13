# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (__/08/2026)

### Added

commit e2e8ae98fdf686cabedd4f69a2b54b3c14a45c87

- Added repetition rules to the impl_val_getter_setter macro.

- Added repetition rules to the impl_trait_val_getter_setter macro.

- Added repetition rules to the impl_ref_mut_getters macro.

- Added repetition rules to the impl_trait_ref_mut_getters macro.

- Added repetition rules to the trait_val_getter_setter macro.

- Added repetition rules to the trait_ref_getter macro.

- Added repetition rules to the trait_mut_getter macro.

- Added repetition rules to the trait_ref_mut_getters macro.

- Added repetition rules to the trait_val_clone_setter macro.

commit 0fa3ae08550fdcb6ef347f6fad82c774a89a96d8

-- Added two repetition rules to the newly renamed impl_val_clone_setter macro.

- Added two repetition rules to the impl_val_clone_setter macro.

Renamed

-- Added two repetition rules to the newly renamed impl_trait_val_clone_setter macro.

- Added two repetition rules to the impl_trait_val_clone_setter macro.

Renamed

- Added two repetition rules to the trait_val_getter macro.

- Added two repetition rules to the trait_val_setter macro.

commit 5b2e176a2505e7fbdeabb18ac0cc5018a34b1f7d

- Added repetition rules to the new impl_val_getter macro.

- Added repetition rules to the impl_val_clone_getter macro.

- Added the impl_trait_val_getter macro.

- Added repetition rules to the impl_trait_val_clone_getter macro.

commit a208c10c9383f086d57efabbf4e29bf240c6e2ee

- Added repetition rules to the impl_ref_getter macro.

- Added repetition rules to the impl_trait_ref_getter macro.

- Added repetition rules to the impl_mut_getter macro.

- Added repetition rules to the impl_trait_mut_getter macro.

- Added a new impl_val_getter macro.



### Changed

commit e2e8ae98fdf686cabedd4f69a2b54b3c14a45c87

- Updated the readme.

commit 0fa3ae08550fdcb6ef347f6fad82c774a89a96d8

-- Renamed the impl_clone_val_setter macro to impl_val_clone_setter.

- Renamed the impl_set_val_clone macro to impl_val_clone_setter.

Renamed

-- Renamed the impl_trait_clone_val_setter macro to impl_trait_val_clone_setter.

- Renamed the impl_trait_set_val_clone macro to impl_trait_val_clone_setter.

Renamed

-- Renamed the trait_clone_sig_val_setter macro to trait_val_clone_setter.

Rename covered

commit 5b2e176a2505e7fbdeabb18ac0cc5018a34b1f7d

- Renamed the original impl_trait_val_getter macro to impl_trait_val_clone_getter.

commit a208c10c9383f086d57efabbf4e29bf240c6e2ee

- Renamed the original impl_val_getter macro to impl_val_clone_getter and updated the project accordingly.

commit 9bbac61a3addb8bc1926207c47bd0840e9ac0c6d

-- Fixed some macro invocations.



### Deprecated



### Removed



### Fixed



### Security



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
