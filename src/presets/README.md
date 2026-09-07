Presets to set commonly used identifier formats.

This module contains multiple submodules, which allows you to narrow-in on a desired configuration.

* `generic` - Configurations which are generic over their boundary options and profile.
* `ascii` - Identifier which use the [`Ascii`] profile and [`Default`] options.
* `strict` - Identifier which use the [`Strict`] profile and [`Default`] options.
* `unicode` - Identifier which use the [`Unicode`] profile and [`Default`] options.

[`Ascii`]: crate::syntax::profile::Ascii
[`Options`]: crate::syntax::boundary::Options
[`Strict`]: crate::syntax::profile::Strict
[`Unicode`]: crate::syntax::profile::Unicode

# Preset Formats

Formats have been broken into two high-level categories; those that prefer delimiters, and those that prefer delimiting by chunk case boundaries.

Identifiers are named based on a combination of *casing* and *format*. `Mixed` cased identifiers use no named prefix on the type, because they're technically a superset of all cased variants of the given identifier.

Some identifiers end up mapping to use the same configuration. That's okay, it just means that the two identifiers are format-equivalent (like mixed-case snake and camel). You should use types based on *intent* not based on the data.

**Prefers Delimiters**

These are identifiers which prefer there be a delimiter between chunks.

| **Delimiter\\Casing**  | **Mixed-Case**  | **Lower-Case**  | **Upper-Case**  |
|------------------------|-----------------|-----------------|-----------------|
| **Low Line (`_`)**     | [`Mixed_Snake`] | [`lower_snake`] | [`UPPER_SNAKE`] |
| **Hyphen-Minus (`-`)** | [`Mixed-Kebab`] | [`lower-kebab`] | [`UPPER-KEBAB`] |

[`Mixed_Snake`]: crate::presets::generic::SnakeIdent
[`lower_snake`]: crate::presets::generic::LowerSnakeIdent
[`UPPER_SNAKE`]: crate::presets::generic::UpperSnakeIdent
[`Mixed-Kebab`]: crate::presets::generic::KebabIdent
[`lower-kebab`]: crate::presets::generic::LowerKebabIdent
[`UPPER-KEBAB`]: crate::presets::generic::UpperKebabIdent

**Prefers Chunk Boundary**

These are identifiers which prefer there *NOT* to be delimiters between chunks, instead depending on casing to separate chunks. (However, delimiters are still allowed, they are only introduced whenever casing would not be able to separate two chunks).

| **Delimiter\\Casing**     | **Mixed-Case**  | **Lower-Case**  | **Upper-Case**  |
|---------------------------|-----------------|-----------------|-----------------|
| **Low Line (`_`)**        | [`MixedCamel`]  | [`lowerCamel`]  | [`UpperCamel`]  |
| **Flat Line (`-` / `_`)** | [`MixedHybrid`] | [`lowerHybrid`] | [`UpperHybrid`] |

[`MixedCamel`]: crate::presets::generic::CamelIdent
[`lowerCamel`]: crate::presets::generic::LowerCamelIdent
[`UpperCamel`]: crate::presets::generic::UpperCamelIdent
[`MixedHybrid`]: crate::presets::generic::HybridIdent
[`lowerHybrid`]: crate::presets::generic::LowerHybridIdent
[`UpperHybrid`]: crate::presets::generic::UpperHybridIdent
