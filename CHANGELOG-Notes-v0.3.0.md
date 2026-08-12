commit e2e8ae98fdf686cabedd4f69a2b54b3c14a45c87
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Aug 11 20:00:35 2026 +1200

    - Updated the readme.
    
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
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 29 19:50:50 2026 +1200

    - Renamed the impl_clone_val_setter macro to impl_val_clone_setter.
    
    - Added two repetition rules to the newly renamed impl_val_clone_setter macro.
    
    - Renamed the impl_trait_clone_val_setter macro to impl_trait_val_clone_setter.
    
    - Added two repetition rules to the newly renamed impl_trait_val_clone_setter macro.
    
    - Added two repetition rules to the trait_val_getter macro.
    
    - Added two repetition rules to the trait_val_setter macro.
    
    - Renamed the trait_clone_sig_val_setter macro to trait_val_clone_setter.

commit 5b2e176a2505e7fbdeabb18ac0cc5018a34b1f7d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jul 28 15:34:22 2026 +1200

    - Added repetition rules to the new impl_val_getter macro.
    
    - Added repetition rules to the impl_val_clone_getter macro.
    
    - Added the impl_trait_val_getter macro.
    
    - Renamed the original impl_trait_val_getter macro to impl_trait_val_clone_getter.
    
    - Added repetition rules to the impl_trait_val_clone_getter macro.

commit a208c10c9383f086d57efabbf4e29bf240c6e2ee
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jul 28 14:35:00 2026 +1200

    - Added repetition rules to the impl_ref_getter macro.
    
    - Added repetition rules to the impl_trait_ref_getter macro.
    
    - Added repetition rules to the impl_mut_getter macro.
    
    - Added repetition rules to the impl_trait_mut_getter macro.
    
    - Added a new impl_val_getter macro.
    
    - Renamed the original impl_val_getter macro to impl_val_clone_getter and updated the project accordingly.

commit 9bbac61a3addb8bc1926207c47bd0840e9ac0c6d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 24 19:09:45 2026 +1200

    Fixed some macro invocations.

commit c29fd4799bbad92d265f965a1e228f5c2229007d
Merge: be8551c 3f7d465
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 24 17:01:01 2026 +1200

    Merge branch 'v0_3_0' of https://github.com/coruscateor/accessorise into v0_3_0

commit be8551c379eea8fdc0029460eabe828b35ce4c3e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 24 16:23:14 2026 +1200

    - Removed the paste dev-dependency.
    
    - Added the pastey dev-dependency and updated the project accordingly.
    
    - Added a rule to the impl_val_setter macro which takes an ident parameter and a ty parameter in curly braces, requiring one or more repetitions in this format, outputting setter method definitions.
    
    - Added a rule to the impl_val_setter macro which takes an ident parameter, a ty parameter and a literal parameter in curly braces, requiring one or more repetitions in this format, outputting documented setter method definitions.
    
    - Added a rule to the impl_trait_val_setter macro which takes an ident parameter and a ty parameter in curly braces, requiring one or more repetitions in this format, outputting setter method definitions.
    
    - Added a rule to the impl_trait_val_setter macro which takes an ident parameter, a ty parameter and a literal parameter in curly braces, requiring one or more repetitions in this format, outputting documented setter method definitions.
    
    - Removed the tests lib-module.
    
    - Added the tests test module.
    
    - Moved the contents of the tests lib-module into the newly added tests module.
    
    - Added the repetition_tests test module.

commit 3f7d465fa9872b759df693d789ba83d0b681269f
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jul 24 16:23:14 2026 +1200

    - Removed the paste dev-dependency.
    
    - Added the pastey dev-dependency and updated the project accordingly.
    
    - Added a rule to the impl_val_setter macro which takes an ident parameter and a ty parameter in curly braces, requiring one or more repetitions in this format, outputting setter method definitions.
    
    - Added a rule to the impl_val_setter macro which takes an ident parameter, a ty parameter and a literal parameter in curly braces, requiring one or more repetitions in this format, outputting documented setter method definitions.
    
    - Removed the tests lib-module.
    
    - Added the tests test module.
    
    - Moved the contents of the tests lib-module into the newly added tests module.
    
    - Added the repetition_tests test module.

commit 66f4b096dbf39918cc8c85b03a070e917f1a60b8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jul 23 14:37:38 2026 +1200

    - Updated the macro invocations in the impl_val_getter_setter macro.
    
    - Updated the macro invocations in the impl_trait_val_getter_setter macro.
    
    - Updated the macro invocations in the impl_ref_mut_getters macro.
    
    - Updated the macro invocations in the impl_trait_ref_mut_getters macro.
    
    - Updated the tests module to use the new macro names.
    
    - Updated the macro invocations in the trait_val_getter_setter macro.
    
    - Renamed the trait_mut_ref_getters macro to trait_ref_mut_getters.

commit b86fd3a613727395cea64bf7ad491b899d9d024a
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jul 22 20:48:27 2026 +1200

    - Renamed the impl_set_val macro to impl_val_setter.
    
    - Renamed the impl_trait_set_val macro to impl_trait_val_setter.
    
    - Renamed the impl_get_ref macro to impl_ref_getter.
    
    - Renamed the impl_trait_get_ref macro to impl_trait_ref_getter.
    
    - Renamed the impl_get_mut macro to impl_mut_getter.
    
    - Renamed the impl_trait_get_mut macro to impl_trait_mut_getter.
    
    - Renamed the impl_get_val macro to impl_val_getter.
    
    - Renamed the impl_trait_get_val macro to impl_trait_val_getter.
    
    - Renamed the impl_set_val_clone macro to impl_clone_val_setter.
    
    - Renamed the impl_trait_set_val_clone macro to impl_trait_clone_val_setter.
    
    - Renamed the impl_get_set_val macro to impl_val_getter_setter.
    
    - Renamed the impl_trait_get_set_val macro to impl_trait_val_getter_setter.
    
    - Renamed the impl_get_ref_mut macro to impl_ref_mut_getters.
    
    - Renamed the impl_trait_get_ref_mut macro to impl_trait_ref_mut_getters.
    
    - Renamed the trait_get_val macro to trait_val_getter.
    
    - Renamed the trait_set_val macro to trait_val_setter.
    
    - Renamed the trait_get_set_val macro to trait_val_getter_setter.
    
    - Renamed the trait_get_ref macro to trait_ref_getter.
    
    - Renamed the trait_get_mut macro to trait_mut_getter.
    
    - Renamed the trait_get_ref_mut macro to trait_mut_ref_getters.
    
    - Renamed the trait_set_val_clone_sig macro to trait_clone_sig_val_setter.
    
    WIP
