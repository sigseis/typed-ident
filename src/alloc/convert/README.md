Infallible string conversion to another cased format.

The main conversions traits defined by this module are:

* [`ToLowerCamel`] - for identifiers of the format `lowerCamelCase`.
* [`ToLowerHybrid`] - for identifiers of the format `-_lowerHybridCase`.
* [`ToLowerKebab`] - for identifiers of the format `lower-kebab-case`.
* [`ToLowerSnake`] - for identifiers of the format `lower_snake_case`.
* [`ToUpperCamel`] - for identifiers of the format `UpperCamelCase`.
* [`ToUpperHybrid`] - for identifiers of the format `-_UpperHybridCase`.
* [`ToUpperKebab`] - for identifiers of the format `UPPER-KEBAB-CASE`.
* [`ToUpperSnake`] - for identifiers of the format `UPPER_SNAKE_CASE`.

If you include the trait, then you should be able to use any of the defined functions, as all valid identifiers support these conversion operations.

# How Does Conversion Work?

It's basically a shortcut for calling the equivalent formatting traits from the [`fmt`] module over a string destination buffer, then returning the string to you. Failure to push into the string buffer will panic.

You should see the [`fmt`] modules for more information (especially regarding the different forms; canonical, decorated, and delimited).

[`fmt`]: crate::core::fmt

# Why Aren't Results Typed?

For two main reasons:

1. *(Most importantly)* string conversion may move you outside of the initial character profile.
2. It's not guaranteed that the user wants to assign into a typed identifier, anyways.

Reason (2) is pretty obvious, but (1) is quite nuanced.

The obvious reason for (1) is the user could define a really weird profile, in which the lower or uppercase equivalent characters are not present. But the more tricky and nuanced reason is: *not all cased Unicode characters have alternative cased mappings*!

For instance, consider the character `ᴨ` (GREEK LETTER SMALL CAPITAL PI). This character is registered as *lowercase*, but what's more - *there's no uppercase mapping for it*. In a perfect world, if a letter were cased it would have some alternative case mapping. If that were true, `to_uppercase` and `to_lowercase` would always change a cased character's casing, but that is *NOT* the case (pun intended).

So, you can get some really tricky failures, like this:

```rust
# use typed_ident::alloc::convert::*;
# use typed_ident::presets::unicode::*;
let ident = LowerSnakeIdent::new("ident_ᴨ")?;
let upper_camel_ish = ident.to_upper_camel_canonical();

// Chunk boundary preserved, but ᴨ is lowercase!
assert_eq!(upper_camel_ish, "Ident_ᴨ");

// And because of this, a strongly-typed format check fails...
assert!(UpperCamelIdent::new(&upper_camel_ish).is_err());

// But, if you drop the upper-cased chunk-start requirement, we're good...
assert!(CamelIdent::new(&upper_camel_ish).is_ok());
# Ok::<(), typed_ident::Error>(())
```

If you'd like to remain working with strong types, my recommendation is this:

* Strongly case-type your input (e.g. something you control constraining over; `LowerCamelCaseIdent`, etc).
* Weakly case-type your conversions (e.g. you are at the whims of Unicode; `SnakeCaseIdent`, etc).

As demonstrated in the above example code, this should always be okay as long as you don't define some weird profile.
