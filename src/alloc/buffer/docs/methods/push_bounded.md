Attempts to push a character into the buffer, preserving chunk boundaries by conditionally inserting delimiters if needed.

If the character is a delimiter, it will always be pushed verbatim.

If the character is *NOT* a delimiter, the character will first be pushed, and then it will be tested to ensure that the left side of the character don't merge into the prior chunks. If it did, a delimiter will be inserted to force separation.
