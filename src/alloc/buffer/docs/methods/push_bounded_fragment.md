Attempts to push a fragment into the buffer, preserving chunk boundaries by conditionally inserting delimiters if needed.

If the fragment already contains delimiters on the left, it will always be inserted verbatim.

If the fragment does *NOT* contain delimiters on the left, the fragment will first be pushed, and then it will be tested to ensure that the left side of the fragment didn't merge into the prior chunk. If it did, a delimiter will be inserted to force separation.
