Attempts to push a fragment into the buffer, preserving chunk boundaries by ensuring a delimiter is present on the left side (if needed).

This will first convert the string slice to a fragment, and then attempting to push the fragment (it is equivalent to first calling [`Fragment::new`] on your input, then calling the fragment-equivalent version of this function instead).

If the fragment already contains delimiters on the left, it will be inserted verbatim.

If the fragment does *NOT* contain delimiters on the left, and the prior fragment did not contain a delimiter on the right, then a delimiter will be pushed first. Finally, the fragment itself will be pushed.

[`Fragment::new`]: crate::core::Fragment::new
