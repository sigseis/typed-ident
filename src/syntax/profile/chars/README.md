A module providing base character profiles for identifiers.

To restrict these profiles based on casing rules, see the [`case`] module.

The character profiles provided by this module are:

* [`Ascii`], in-which the ident-start must be ASCII alphabetic, and all other characters may be ASCII alphanumeric.
* [`Unicode`], which is a Unicode XID profile (except with low line, ZWNJ, and ZWJ removed).
* [`Strict`], which is [`Unicode`], except we don't allow `Mc`, `Me`, or `Mn` characters at the start of a chunk.

For more details, see the respective documentation for each item.

[`case`]: crate::syntax::profile::case
