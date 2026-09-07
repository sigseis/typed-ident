// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn simple_xid_check(
    s: &str,
    allowed_delims: &'static [char],
    allowed_ident_start_case: impl Fn(char) -> bool,
    allowed_chunk_start_case: impl Fn(char) -> bool,
    allowed_chunk_continue_case: impl Fn(char) -> bool,
) -> bool {
    let mut chars = s.chars();
    let Some(c) = chars.next() else {
        return false;
    };
    let mut last_is_delim = allowed_delims.contains(&c);
    if !last_is_delim && !(unicode_ident::is_xid_start(c) && allowed_ident_start_case(c)) {
        return false;
    }
    for c in chars {
        let curr_is_delim = allowed_delims.contains(&c);
        if !curr_is_delim
            && !(unicode_ident::is_xid_continue(c)
                && if last_is_delim {
                    allowed_chunk_start_case(c)
                } else {
                    allowed_chunk_continue_case(c)
                })
        {
            return false;
        }
        last_is_delim = curr_is_delim;
    }
    true
}

// -----------------------------------------------------------------------------
fn basic_lowercase(c: char) -> bool {
    !c.is_uppercase()
}

// -----------------------------------------------------------------------------
fn basic_mixedcase(_: char) -> bool {
    true
}

// -----------------------------------------------------------------------------
fn basic_uppercase(c: char) -> bool {
    !c.is_lowercase()
}

// =============================================================================
// VALIDATORS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn baseline(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(c) = chars.next() else {
        return false;
    };
    if !(unicode_ident::is_xid_start(c) || c == '-' || c == '_') {
        return false;
    }
    for c in chars {
        if !(unicode_ident::is_xid_continue(c) || c == '-') {
            return false;
        }
    }
    true
}

// -----------------------------------------------------------------------------
pub fn mixed_camel(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_mixedcase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn lower_camel(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_lowercase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn upper_camel(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_uppercase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn mixed_hybrid(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_', '-'],
        /*allowed_ident_start_case=*/ basic_mixedcase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn lower_hybrid(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_', '-'],
        /*allowed_ident_start_case=*/ basic_lowercase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn upper_hybrid(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_', '-'],
        /*allowed_ident_start_case=*/ basic_uppercase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn mixed_kebab(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['-'],
        /*allowed_ident_start_case=*/ basic_mixedcase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn lower_kebab(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['-'],
        /*allowed_ident_start_case=*/ basic_lowercase,
        /*allowed_chunk_start_case=*/ basic_lowercase,
        /*allowed_chunk_continue_case=*/ basic_lowercase,
    )
}

// -----------------------------------------------------------------------------
pub fn upper_kebab(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['-'],
        /*allowed_ident_start_case=*/ basic_uppercase,
        /*allowed_chunk_start_case=*/ basic_uppercase,
        /*allowed_chunk_continue_case=*/ basic_uppercase,
    )
}

// -----------------------------------------------------------------------------
pub fn mixed_snake(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_mixedcase,
        /*allowed_chunk_start_case=*/ basic_mixedcase,
        /*allowed_chunk_continue_case=*/ basic_mixedcase,
    )
}

// -----------------------------------------------------------------------------
pub fn lower_snake(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_lowercase,
        /*allowed_chunk_start_case=*/ basic_lowercase,
        /*allowed_chunk_continue_case=*/ basic_lowercase,
    )
}

// -----------------------------------------------------------------------------
pub fn upper_snake(s: &str) -> bool {
    simple_xid_check(
        /*string=*/ s,
        /*allowed_delims=*/ &['_'],
        /*allowed_ident_start_case=*/ basic_uppercase,
        /*allowed_chunk_start_case=*/ basic_uppercase,
        /*allowed_chunk_continue_case=*/ basic_uppercase,
    )
}
