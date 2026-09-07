Infallible formatting operations to another cased format.

The main formatting traits defined by this module are:

* [`AsLowerCamel`] - for identifiers of the format `lowerCamelCase`.
* [`AsLowerHybrid`] - for identifiers of the format `-_lowerHybridCase`.
* [`AsLowerKebab`] - for identifiers of the format `lower-kebab-case`.
* [`AsLowerSnake`] - for identifiers of the format `lower_snake_case`.
* [`AsUpperCamel`] - for identifiers of the format `UpperCamelCase`.
* [`AsUpperHybrid`] - for identifiers of the format `-_UpperHybridCase`.
* [`AsUpperKebab`] - for identifiers of the format `UPPER-KEBAB-CASE`.
* [`AsUpperSnake`] - for identifiers of the format `UPPER_SNAKE_CASE`.

If you include the trait, then you should be able to use any of the defined functions, as all valid identifiers support these formatting operations.

# How Does Formatting Work?

There's two kinds of identifier formats; ones that want explicit delimiters, and ones that can depend on casing (in addition to delimiters where casing is insufficient).

The **Explicit delimiter identifiers** (`kebab` and `snake`), the algorithm is pretty simple - you iterate over segments, ensuring there's a delimiter between chunks, and converting to the target case. That's really all there is to it.

The **Implicit delimiter identifiers** (`camel` and `hybrid`), things are a little more complicated.

During the conversion process, we keep track of the last-inserted character's case, and we will do a boundary case check to see if the current and next character forms a boundary. If it does - we accept that as satisfactory, and push the character. If it *does not* - then we need to introduce a delimiter to keep the chunks separated.

The goal of any formatting operation is *not to join chunks*.

## What Happens to the Delimiters?

Unlike a traditional case conversion library, this library *knows* it's dealing with identifiers.

Like it or not, delimiters sometimes are used as a part of identifiers to pass along some implicit information. For example, `_ident` in Rust is an identifier that is not used. The leading `_` isn't required, but we add it to suggest this implicit fact that it is unused.

We informally call such non-essential delimiters "decorative".

There's three formats you can apply over your formatting operations:

1. `Canonical` - Get rid of all non-essential delimiters.
2. `Decorated` - Keep all purely decorative delimiters (but allow stripping of non-decorative ones).
3. `Delimited` - Keep all delimiters, regardless of whether they're decorative or not.

This library allows you to choose, so it's really up to you. All of the functions are named with an explicit mode of operation at the end ([`as_lower_camel_canonical`] vs [`as_lower_camel_decorated`], etc). So you *will* have to make a choice.

Perhaps this is best demonstrated with an example:

```rust
# use typed_ident::core::fmt::*;
# use typed_ident::presets::unicode::*;
assert_eq!(
    SnakeIdent::new("__foo_bar__baz__")?.as_upper_camel_canonical().to_string(),
    "FooBarBaz"
);
assert_eq!(
    SnakeIdent::new("__foo_bar__baz__")?.as_upper_camel_decorated().to_string(),
    "__FooBar__Baz__"
);
assert_eq!(
    SnakeIdent::new("__foo_bar__baz__")?.as_upper_camel_delimited().to_string(),
    "__Foo_Bar__Baz__"
);

// Note: Delimited is based on whether or not there were already delimiters:
assert_eq!(
    CamelIdent::new("__fooBar__baz__")?.as_upper_camel_delimited().to_string(),
    "__FooBar__Baz__"
);
# Ok::<(), typed_ident::Error>(())
```

I avoided having a seemingly-default function like `as_lower_camel`, because I believe people will disagree on what the default should be. Most case conversion libraries implement "canonical" form, but if I were to select a default for this library, I would select "decorated" form (since, as mentioned, decoration is sometimes important on an identifier).

*NOTE: `kebab` and `snake` don't have a "delimited" format, because by-definition they already require a delimiter. So `Decorated` includes this case within its definition. Delimited format only makes sense on formats that don't require delimiters (camel-like).*

[`as_lower_camel_canonical`]: AsLowerCamel::as_lower_camel_canonical
[`as_lower_camel_decorated`]: AsLowerCamel::as_lower_camel_decorated


## Edge-Case: Multiple Uppercase Expansion

Some letters can expand into multiple characters when made uppercase or lowercase.

For situations involving `CamelCase`, where the intent is to have exactly one uppercase character start a word, the string of uppercase characters will be itself made into `Camel` casing. That is to say, the first cased character will be transformed uppercase, but any additional characters will be forced lowercase.

```rust
# use typed_ident::core::fmt::*;
# use typed_ident::presets::unicode::*;
// If at chunk-start...
assert_eq!(
    CamelIdent::new("ßtest")?.as_upper_camel_canonical().to_string(),
    "Sstest" // ß -> SS -> Ss
);
assert_eq!(
    CamelIdent::new("ﬄtest")?.as_upper_camel_canonical().to_string(),
    "Ffltest" // ﬄ -> FFL -> Ffl
);
assert_eq!(
    CamelIdent::new("Test_ßtest")?.as_upper_camel_canonical().to_string(),
    "TestSstest" // ß -> SS -> Ss
);
assert_eq!(
    CamelIdent::new("Test_ﬄtest")?.as_upper_camel_canonical().to_string(),
    "TestFfltest" // ﬄ -> FFL -> Ffl
);
// etc.
# Ok::<(), typed_ident::Error>(())
```

## Edge-Case: Σ *always* Maps to σ for Lowercase

The justification for this is that we don't actually respect locale anywhere else in the crate, so making an exception here feels odd.

The only other kind-of locale thing we do is handle greek/non-greek titlecase for determining casing. However, this has nothing to do with the locale. more to do with the fact that those characters are *digraphs*, which are pairs of letters. The non-greek titlecase characters are a combination of an uppercase and a lowercase.

So, TL;DR, as far as formatting is concerned, `Σ` maps to `σ` *always*.

```rust
# use typed_ident::core::fmt::*;
# use typed_ident::presets::unicode::*;
assert_eq!(
    CamelIdent::new("Σ")?.as_lower_camel_canonical().to_string(),
    "σ" // Despite being at word-end
);
// etc.
# Ok::<(), typed_ident::Error>(())
```
