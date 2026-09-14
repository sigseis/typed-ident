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
