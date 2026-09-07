Syntax definitions for chunk character profiles.

Profiles are broken into two sub-categories:

* [`case`], which mutates a character profile based on some casing rules.
* [`chars`], which presents a base set of rules for a chunk character profile.

The expected way to use these types is via generic composition. For instance:

```rust
use typed_ident::syntax::profile::{case, chars};
type LowerAsciiProfile = case::Lower<chars::Ascii>;
```

For more details, see the respective documentation for each item.
