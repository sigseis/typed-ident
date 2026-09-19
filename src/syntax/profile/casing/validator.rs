// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::syntax::profile::casing::Casing;
use crate::syntax::{CasedProfile, Delimiter, SyntaxError};
use core::marker::PhantomData;

// =============================================================================
// TYPES
// =============================================================================

/// An identifier profile where all character and delimiter checks can be
/// independent from one-another.
pub(super) struct Validator<C, D, P>(PhantomData<(C, D, P)>);

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<C: Casing, D: Delimiter, P: CasedProfile> Validator<C, D, P> {
    #[inline(always)]
    fn is_valid<const ALLOW_DELIMS: bool>(
        s: &str,
        ident_start_chunk: impl FnOnce(char, &mut C) -> bool,
        ident_start_delim: impl FnOnce(char) -> bool,
    ) -> Result<(), SyntaxError> {
        // A specific validation process is selected based on the configuration.
        //
        // Most reasonable identifiers are at least fragment append-closed, and
        // that is actually really helpful, because it means we can greatly
        // simplify the validation logic.
        //
        // It's worth it - validated with `cargo asm`:
        //
        // 1. Mark `Ident::new` function as `#[inline(never)]`
        // 2. `clear && cargo asm -p typed-ident --all-features --example api new`
        // 3. Don't forget to restore that function back to `#[inline]`
        //
        // I've hidden a command that uses preset profiles with `APPEND_CLOSED`
        // set to `Empty` (~open). For Strict, this is no different, as it is
        // already ~open. But for ASCII and Unicode, comparing to the `*Open`
        // profile variants can be enlightening.
        //
        // Looking at a basic configuration (unit delimiter, no casing rules):
        // * ASCII -> 193 lines (closed), 339 lines (open)
        // * Unicode -> 273 lines (closed), 465 lines (open)
        match (!ALLOW_DELIMS || D::APPEND_CLOSED.at_least_fragment())
            && P::APPEND_CLOSED.at_least_fragment()
            && C::UNIFORM
        {
            true => Self::is_valid_append_closed::<ALLOW_DELIMS>(
                s,
                ident_start_chunk,
                ident_start_delim,
            ),
            false => Self::is_valid_append_opened::<ALLOW_DELIMS>(
                s,
                ident_start_chunk,
                ident_start_delim,
            ),
        }
    }

    #[inline]
    fn is_valid_append_opened<const ALLOW_DELIMS: bool>(
        s: &str,
        ident_start_chunk: impl FnOnce(char, &mut C) -> bool,
        ident_start_delim: impl FnOnce(char) -> bool,
    ) -> Result<(), SyntaxError> {
        let mut casing = C::default();
        let mut chars = s.char_indices();
        if let Some((_, c)) = chars.next() {
            // First character must be any kind of delim or in-profile char.
            let mut last_is_delim = if ident_start_delim(c) {
                if !ALLOW_DELIMS {
                    return Err(SyntaxError::Format(0));
                }
                true
            } else if ident_start_chunk(c, &mut casing) {
                false
            } else {
                return Err(SyntaxError::Format(0));
            };

            // Next character must be either:
            //
            // 1. Any kind of chunk delimiter (non-start delim), *or...*
            // 2. An in-profile char, depending on if the prior was a delim.
            //    a. If prior was a delim, next char must be `is_chunk_start`.
            //    a. Otherwise, next char must be `is_chunk_continue`.
            for (idx, c) in chars {
                last_is_delim = if D::is_chunk_delim(c) {
                    if !ALLOW_DELIMS {
                        return Err(SyntaxError::Format(0));
                    }
                    true
                } else {
                    if last_is_delim {
                        if !P::is_chunk_start(c) || !casing.chunk_start_case(c) {
                            return Err(SyntaxError::Format(idx));
                        }
                    } else {
                        if !P::is_chunk_continue(c) || !casing.chunk_continue_case(c) {
                            return Err(SyntaxError::Format(idx));
                        }
                    }
                    false
                };
            }
        }

        Ok(())
    }

    #[inline]
    fn is_valid_append_closed<const ALLOW_DELIMS: bool>(
        s: &str,
        ident_start_chunk: impl FnOnce(char, &mut C) -> bool,
        ident_start_delim: impl FnOnce(char) -> bool,
    ) -> Result<(), SyntaxError> {
        // Greatly simplified if both `D` and `P` are fragment append-closed.
        //
        // Recall what `APPEND_CLOSED` means for `D` and `P`:
        // * For delimiters, it means `is_chunk_delim` is a superset of
        //   `is_ident_start`.
        //   * This means that `is_delim` = `is_chunk_delim`, and so for a
        //     fragment you only need to use `is_delim` everywhere.
        // * For profiles, it means `is_chunk_continue` is a superset of
        //   `is_ident_start` and `is_chunk_start`, *and* that `is_chunk_start`
        //   is identical to `is_chunk_continue`.
        //   * This means that `in_profile` = `is_chunk_continue`, since it must
        //     be the superset, and so for a fragment you only need to use
        //     `in_profile` everywhere.
        //
        // Basically this translates to:
        //
        //   As long as every character is either `D::is_delim` or
        //   `P::in_profile`, then it's a valid fragment!
        let mut casing = C::default();
        let mut chars = s.char_indices();
        if let Some((_, c)) = chars.next() {
            let valid = match ALLOW_DELIMS {
                true => ident_start_delim(c) || ident_start_chunk(c, &mut casing),
                false => !D::is_delim(c) && ident_start_chunk(c, &mut casing),
            };
            if !valid {
                return Err(SyntaxError::Format(0));
            }
        }
        for (idx, c) in chars {
            let valid = match ALLOW_DELIMS {
                true => D::is_delim(c) || (P::is_chunk_char(c) && casing.chunk_continue_case(c)),
                false => !D::is_delim(c) && P::is_chunk_char(c) && casing.chunk_continue_case(c),
            };
            if !valid {
                return Err(SyntaxError::Format(idx));
            }
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
impl<C: Casing, D: Delimiter, P: CasedProfile> Validator<C, D, P> {
    #[inline]
    pub fn is_chunk(s: &str) -> Result<(), SyntaxError> {
        Self::is_valid::<false>(
            s,
            |c, casing| P::is_chunk_char(c) && casing.chunk_continue_case(c),
            D::is_delim,
        )
    }
    #[inline]
    pub fn is_fragment(s: &str) -> Result<(), SyntaxError> {
        Self::is_valid::<true>(
            s,
            |c, casing| P::is_chunk_char(c) && casing.chunk_continue_case(c),
            D::is_delim,
        )
    }
    #[inline]
    pub fn is_ident(s: &str) -> Result<(), SyntaxError> {
        if s.is_empty() {
            return Err(SyntaxError::Empty);
        }
        Self::is_valid::<true>(
            s,
            |c, casing| P::is_ident_start_char(c) && casing.chunk_start_case(c),
            D::is_ident_start_delim,
        )
    }
    #[inline]
    pub fn is_ident_fragment(fragment: &str) -> Result<(), SyntaxError> {
        // If the casing checks are uniform, the order of the casing does not
        // matter, in which case we can simply check the first character to see
        // if it's validly an identifier character or not.
        if C::UNIFORM {
            let Some(first) = fragment.chars().next() else {
                return Err(SyntaxError::Empty);
            };
            if !D::is_ident_start_delim(first) && !P::is_ident_start_char(first) {
                return Err(SyntaxError::Format(0));
            }
            Ok(())
        } else {
            Self::is_ident(fragment)
        }
    }
}
