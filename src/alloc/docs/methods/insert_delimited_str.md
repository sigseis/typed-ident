Attempts to insert a string slice into the buffer, preserving chunk boundaries by ensuring a delimiter is present on each side (where needed).

This will first convert the string slice to a fragment, and then attempting to insert the fragment (it is equivalent to first calling [`Fragment::new`] on your input, then calling the fragment-equivalent version of this function instead).

If the fragment already contains delimiters on both ends, it will always be inserted verbatim.

If the fragment does *NOT* contain delimiters on both ends, then a delimiter will be inserted a number of times depending on how many sides have chunk data immediately next to the insertion point. Finally, the fragment itself will be inserted such that the inserted delimiters will fall on the left or right depending on where they were intended.

[`Fragment::new`]: crate::core::Fragment::new
