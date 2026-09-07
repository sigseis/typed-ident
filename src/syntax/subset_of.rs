// =============================================================================
// TYPES
// =============================================================================

/// A type that suggests that one syntax rule is the subset of another.
///
/// This type is really only sensible on either the [`Profile`] or [`Delimiter`]
/// types. It doesn't do anything for [`Boundary`] or [`Segmentation`].
///
/// Implementing this incorrectly cannot lead to memory issues, but it *can*
/// lead to incorrect logic that will do things you don't expect (like
/// identifying a string as an identifier format that it is not).
///
/// It's valid to *not* implement this trait if you aren't certain, it just
/// prevents certain zero-cost casts from being possible.
///
/// # Burden of Proof
///
/// While it's not an unsafe trait itself, since it's tricky and easy to get
/// wrong, it's highly recommended that each implementation of `SubsetOf`
/// demonstrates why that implementation is sane.
///
/// The format of this is "Proof: (Brief Reason)", and then it can be
/// followed-up with more details if necessary to make the reason clearer. See
/// some of the implementations of this trait in this crate as an example.
///
/// # Always Sane: `SubsetOf<Self>`
///
/// It is *always* sane (and recommended) to implement this against yourself.
/// We don't do this by default to leave the generic impl space open for more
/// complex generic impls.
///
/// * `impl SubsetOf<T> for T {}`
///
/// For more complex cases...
///
/// You can implement this when the validation property of a syntax rule are
/// *all* supersets (or equal-to) the validation properties of your defined
/// syntax (here, referred to as `Self`).
///
/// For [`Delimiter`], you can implement `SubsetOf<Super>` if:
///
/// * `Self::is_ident_start` ⊆ `Super::is_ident_start`, *and...*
/// * `Self::is_chunk_delim` ⊆ `Super::is_chunk_delim`
///
/// For [`Profile`], you can implement `SubsetOf<Super>` if:
///
/// * `Self::is_ident_start` ⊆ `Super::is_ident_start`, *and...*
/// * `Self::is_chunk_start` ⊆ `Super::is_chunk_start`, *and...*
/// * `Self::is_chunk_continue` ⊆ `Super::is_chunk_continue`
///
/// Another way to say this is:
///
/// For any character that returns `true` for a function, you can call the same
/// function in the prospective `Super` type and *also* get `true` - then
/// `Self: SubsetOf<Super>`.
///
/// [`Boundary`]: crate::syntax::boundary::Boundary
/// [`Delimiter`]: crate::syntax::delimiter::Delimiter
/// [`Profile`]: crate::syntax::profile::Profile
/// [`Segmentation`]: crate::syntax::segmentation::Segmentation
pub trait SubsetOf<T: ?Sized> {}
