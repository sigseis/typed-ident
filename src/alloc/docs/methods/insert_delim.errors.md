# Errors

If the delimiter itself was valid, but not at the position inserted, then `InvalidPosition` will be returned.

If inserting the provided delimiter at a position `idx` would lead to an invalid fragment, then the error `InvalidInsertion` will be returned, containing either `Direction::Left` or `Direction::Right` for whether the insertion failed because of the data on the left or the right.
