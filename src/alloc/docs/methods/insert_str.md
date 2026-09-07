Attempts to insert a string slice into the buffer.

This will first convert the string slice to a fragment, and then attempting to insert the fragment (it is equivalent to first calling [`Fragment::new`] on your input, then calling the fragment-equivalent version of this function instead).

[`Fragment::new`]: crate::core::Fragment::new
