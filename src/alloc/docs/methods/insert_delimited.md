Attempts to insert a character into the buffer, preserving chunk boundaries by ensuring a delimiter is present on each side (where needed).

If the character is a delimiter, it will always be inserted verbatim.

If the character is *NOT* a delimiter, then a delimiter will be inserted a number of times depending on how many sides have chunk data immediately next to the insertion point. Finally, the character itself will be inserted such that the inserted delimiters will fall on the left or right depending on where they were intended.
