A module providing profiles which adapt a character profile.

The adapter profiles provided by this module are:

* [`Lower`], which will restrict a profile to non-uppercase characters.
* [`LowerCamel`], like [`Lower`] but only restricts ident-start and chunk-start.
* [`Mixed`], a pass-through profile that makes no modifications.
* [`Upper`], which will restrict a profile to non-lowercase characters.
* [`UpperCamel`], like [`Upper`] but only restricts ident-start and chunk-start.

For more details, see the respective documentation for each item.
