Attempts to insert a fragment into the buffer, preserving chunk boundaries by conditionally inserting delimiters where needed.

If the fragment already contains delimiters on both ends, it will always be inserted verbatim.

If the fragment does *NOT* contain delimiters on both ends, the fragment will first be inserted, and then it will be tested to ensure that both sides of the fragment don't merge into the surrounding chunks. If it did, a delimiter will be inserted to force separation.
