// =============================================================================
// TYPES
// =============================================================================

/// A pass-through casing, which doesn't itself expand to be a cased profile.
///
/// This is useful if you have some generic construct (like a macro) which wants
/// to apply some casing operations, and you want to specify a casing which has
/// no effect on the profile.
pub type Mixed<P> = P;
