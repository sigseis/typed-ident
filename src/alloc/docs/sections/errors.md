# Errors

On failure to interpret the provided data as a valid fragment (a character does not match the profile, or an unexpected/disallowed delimiter is used) `InvalidFormat` will be returned, and [`byte_offset`] will be set to the index of the invalid character.

If the fragment was valid, but we failed to insert it, `InvalidLeftJoin` or `InvalidRightJoin` will be returned, depending on whether the insertion failed because of the data on the left or the right of the join.

[`byte_offset`]: crate::core::Error::byte_offset
