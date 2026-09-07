# Unicode Warning

If you are working with Unicode data, you very likely don't want this function. Instead, you likely want to insert an entire *grapheme*. Inserting one character from a grapheme bounded, and then inserting other characters unbounded could change the decision for whether or not a grapheme cluster bounds on the left or right.

This function really only works as one would expect if the grapheme cluster is exactly one character big (for example, ASCII data has this property).
