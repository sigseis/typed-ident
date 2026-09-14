Splits the buffer into two halves at a given byte index.

Returns a newly allocated buffer. `self` contains bytes `[0, at)`, and the returned buffer contains bytes `[at, len)`. `at` must be on the boundary of a UTF-8 code point.

Note that the capacity of `self` does not change.
