// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn gen_ident_start_char<I: Identifier + ?Sized>(
    case_mapping: impl Fn(char, bool) -> Option<char>,
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
) -> char {
    loop {
        let Some(c) = case_mapping(chars.sample(rng), /*is_start=*/ true) else {
            continue;
        };
        if I::Profile::is_ident_start_char(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_chunk_start_char<I: Identifier + ?Sized>(
    case_mapping: impl Fn(char, bool) -> Option<char>,
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
) -> char {
    loop {
        let Some(c) = case_mapping(chars.sample(rng), /*is_start=*/ true) else {
            continue;
        };
        if I::Profile::is_chunk_start(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_chunk_continue_char<I: Identifier + ?Sized>(
    case_mapping: impl Fn(char, bool) -> Option<char>,
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
) -> char {
    loop {
        let Some(c) = case_mapping(chars.sample(rng), /*is_start=*/ false) else {
            continue;
        };
        if I::Profile::is_chunk_continue(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_ident_chunk<I: Identifier + ?Sized>(
    case_mapping: impl Fn(char, bool) -> Option<char>,
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
    len: usize,
) -> String {
    let mut chunk = String::with_capacity(len);
    chunk.push(gen_chunk_start_char::<I>(&case_mapping, chars, rng));
    for _ in 1..len {
        chunk.push(gen_chunk_continue_char::<I>(&case_mapping, chars, rng));
    }
    chunk
}

// -----------------------------------------------------------------------------
fn gen_ident_start<I: Identifier + ?Sized>(
    case_mapping: impl Fn(char, bool) -> Option<char>,
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
    len: usize,
) -> String {
    let mut chunk = String::with_capacity(len);
    chunk.push(gen_ident_start_char::<I>(&case_mapping, chars, rng));
    for _ in 1..len {
        chunk.push(gen_chunk_continue_char::<I>(&case_mapping, chars, rng));
    }
    chunk
}

// -----------------------------------------------------------------------------
#[allow(clippy::too_many_arguments)]
fn gen_ident<I: Identifier + ?Sized>(
    ident_type: &str,
    case_mapping: impl Fn(char, bool) -> Option<char>,
    delims: &[char],
    chars: &impl Distribution<char>,
    rng: &mut SmallRng,
    delims_dist: &impl Distribution<usize>,
    len: usize,
    mut chunks: usize,
) -> String {
    if chunks > len || chunks == 0 {
        chunks = 1;
    }
    let chunk_sizes = std::cmp::max(len / chunks, 1);

    let mut ident = String::with_capacity(len);
    ident.push_str(&gen_ident_start::<I>(
        &case_mapping,
        chars,
        rng,
        chunk_sizes,
    ));
    for _ in 1..chunks {
        ident.push(delims[delims_dist.sample(rng)]);
        ident.push_str(&gen_ident_chunk::<I>(
            &case_mapping,
            chars,
            rng,
            chunk_sizes,
        ));
    }

    // Test the identifier before returning it.
    assert!(
        I::new(&ident).is_ok(),
        "typed: failed to generate a valid identifier for {ident_type}: {ident}"
    );
    assert!(
        crate::validate::baseline(&ident),
        "baseline: failed to generate a valid identifier for {ident_type}: {ident}"
    );

    ident
}

// -----------------------------------------------------------------------------
fn gen_idents<I: Identifier + ?Sized>(
    ident_type: &str,
    case_mapping: impl Fn(char, bool, bool) -> Option<char>,
    delims: &[char],
    chars: &impl Distribution<char>,
) -> Vec<String> {
    let mut rng = SmallRng::seed_from_u64(1);
    let chunks = Uniform::new_inclusive(1, MAX_IDENTIFIER_CHUNKS).unwrap();
    let length = Uniform::new_inclusive(MIN_IDENTIFIER_LENGTH, MAX_IDENTIFIER_LENGTH).unwrap();
    let delims_dist = Uniform::new(0, delims.len()).unwrap();

    let mut identifiers = Vec::with_capacity(POINTS_MAX);
    for _ in 0..POINTS_MAX {
        let length = length.sample(&mut rng);
        let chunks = chunks.sample(&mut rng);
        let upper = rng.random_bool(0.5);
        identifiers.push(gen_ident::<I>(
            ident_type,
            |c, i| case_mapping(c, i, upper),
            delims,
            chars,
            &mut rng,
            &delims_dist,
            length,
            chunks,
        ));
    }
    identifiers
}

// =============================================================================
// CASE MAPPING
// =============================================================================

// -----------------------------------------------------------------------------
pub fn dependent(c: char, _is_start: bool, is_upper: bool) -> Option<char> {
    match is_upper {
        false => c.to_lowercase().find(|c| c.is_lowercase()),
        true => c.to_uppercase().find(|c| c.is_uppercase()),
    }
}

// -----------------------------------------------------------------------------
pub fn dependent_start(c: char, is_start: bool, is_upper: bool) -> Option<char> {
    match is_start {
        false => Some(c),
        true => match is_upper {
            false => c.to_lowercase().find(|c| c.is_lowercase()),
            true => c.to_uppercase().find(|c| c.is_uppercase()),
        },
    }
}

// -----------------------------------------------------------------------------
pub fn independent(c: char, _is_start: bool, _is_upper: bool) -> Option<char> {
    // Independent generator classes don't impact across characters.
    Some(c)
}

// =============================================================================
// GENERATORS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn ascii<I: Identifier + ?Sized>(
    ident_type: &str,
    case_mapping: impl Fn(char, bool, bool) -> Option<char>,
    delims: &[char],
) -> Vec<String> {
    gen_idents::<I>(
        ident_type,
        case_mapping,
        delims,
        &Uniform::new_inclusive(0x20 as char, 0x7F as char).unwrap(),
    )
}

// -----------------------------------------------------------------------------
pub fn unicode<I: Identifier + ?Sized>(
    ident_type: &str,
    case_mapping: impl Fn(char, bool, bool) -> Option<char>,
    delims: &[char],
) -> Vec<String> {
    gen_idents::<I>(
        ident_type,
        case_mapping,
        delims,
        &Uniform::new_inclusive(0x20 as char, char::MAX).unwrap(),
    )
}
