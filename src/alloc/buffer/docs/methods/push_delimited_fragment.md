Attempts to push a fragment into the buffer, preserving chunk boundaries by ensuring a delimiter is present on the left side (if needed).

If the fragment already contains delimiters on the left, it will be inserted verbatim.

If the fragment does *NOT* contain delimiters on the left, and the prior fragment did not contain a delimiter on the right, then a delimiter will be pushed first. Finally, the fragment itself will be pushed.
