Core type definitions that make up this crate.

The main types defined by this module are:

* [`Chunk`] is a subslice of an identifier which contains no delimiter characters.
* [`Fragment`] is a subslice of an identifier which may not be a valid identifier.
* [`Ident`] is a reference to a string slice that forms a valid identifier.

Usually, you start from the perspective of an [`Ident`]. Then, certain operations may return to you either a [`Fragment`] or a [`Chunk`]. For example, you can perform slicing operations on an identifier, but this operation may not return a valid identifier - so it returns a fragment. So, fragment and chunk are usually incidental types that you get during certain operations.

For more details, see the respective documentation for each item.
