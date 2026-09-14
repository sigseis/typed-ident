Replace a range of characters with a provided replacement.

This will first convert the string slice to a fragment, and then attempt to use it as a replacement (it is equivalent to first calling [`Fragment::new`] on your input, then calling the fragment-equivalent version of this function instead).

[`Fragment::new`]: crate::core::Fragment::new
