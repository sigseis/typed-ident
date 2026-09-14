Attempts to push a character into the buffer, preserving chunk boundaries by ensuring a delimiter is present on the left side (if needed).

If the character is a delimiter, it will always be pushed verbatim.

If the character is *NOT* a delimiter, then a delimiter will be pushed first if there is chunk data at the end of the buffer. Finally, the character itself will be pushed.
