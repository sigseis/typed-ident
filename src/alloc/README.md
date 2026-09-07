A module for defining typed identifier functionality that relies on allocation.

The main things this module provides are:

* [`FragmentBuf`] - The ability to dynamically build a fragment.
* [`IdentBuf`] - The ability to dynamically build an identifier.
* [`convert`] - The ability to convert identifiers from one format to another.

See the respective items for more details.

# What's in the Box!?

Some functions may return boxed versions of identifiers instead of identifier buffers. The reason for this is the non-empty property of identifiers.

Buffers have to be able to be empty, because they're useful in building or representing a dynamic identifier that may contain no data. So the user has to deal with that if they may be in that state (see [`IdentBuf::as_ident`]).

However, some operations are purely additive - so instead of returning an `IdentBuf`, we opt to return a `Box<Ident>`, to signify that the operation will always return a valid identifier.

Do note that [`FragmentBuf`] is different - since an empty fragment is allowed, for those cases we just return `FragmentBuf` for all modifying operations.
