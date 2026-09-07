// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::Profile;

// =============================================================================
// TRAITS
// =============================================================================

/// A trait which further distinguishes a profile as an adapter of another base
/// profile, with some new casing rules mixed in.
///
/// # Note
///
/// A `CasedProfile` should not also be a [`CharProfile`], it's either one or
/// the other (either it defines a base set of characters, *OR* it defines an
/// adapter for a provided set of base characters).
///
/// Defining both won't lead to logical errors, but it may not compile (due to
/// overlapping trait implementations), and at worst you may cause code bloat by
/// virtue of your profile being usable in places where it's not expected.
///
/// [`CharProfile`]: crate::syntax::profile::CharProfile
pub trait CasedProfile: Profile {}
