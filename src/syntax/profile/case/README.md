A module providing profiles which adapt a character profile.

The adapter profiles provided by this module are:

* [`Camel`], which restricts a profile to either `LowerCamel` or `UpperCamel`
* [`Lower`], which will restrict a profile to non-uppercase characters
* [`LowerCamel`], like `Lower` but only restricts ident-start and chunk-start
* [`Mixed`], a pass-through profile that makes no further restrictions
* [`Uniform`], which restricts a profile to either `Lower` or `Upper`
* [`Upper`], which will restrict a profile to non-lowercase characters
* [`UpperCamel`], like `Upper` but only restricts ident-start and chunk-start

For more details, see the respective documentation for each item.
