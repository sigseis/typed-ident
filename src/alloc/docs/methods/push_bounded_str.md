Attempts to push a string slice into the buffer, preserving chunk boundaries by conditionally inserting delimiters if needed.

This will first convert the string slice to a fragment, and then attempting to push the fragment (it is equivalent to first calling [`Fragment::new`] on your input, then calling the fragment-equivalent version of this function instead).

If the fragment already contains delimiters on the left, it will always be inserted verbatim.

If the fragment does *NOT* contain delimiters on the left, the fragment will first be pushed, and then it will be tested to ensure that the left side of the fragment didn't merge into the prior chunk. If it did, a delimiter will be inserted to force separation.

[`Fragment::new`]: crate::core::Fragment::new
