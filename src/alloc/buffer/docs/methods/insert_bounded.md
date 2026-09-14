Attempts to insert a character into the buffer, preserving chunk boundaries by conditionally inserting delimiters where needed.

If the character is a delimiter, it will always be inserted verbatim.

If the character is *NOT* a delimiter, the character will first be inserted, and then it will be tested to ensure that both sides of the character don't merge into the surrounding chunks. If it did, a delimiter will be inserted to force separation.
