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
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
) -> char {
    loop {
        let c = dist.sample(rng);
        if I::Profile::is_ident_start(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_chunk_start_char<I: Identifier + ?Sized>(
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
) -> char {
    loop {
        let c = dist.sample(rng);
        if I::Profile::is_chunk_start(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_chunk_continue_char<I: Identifier + ?Sized>(
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
) -> char {
    loop {
        let c = dist.sample(rng);
        if I::Profile::is_chunk_continue(c) {
            return c;
        }
    }
}

// -----------------------------------------------------------------------------
fn gen_ident_chunk<I: Identifier + ?Sized>(
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
    len: usize,
) -> String {
    let mut chunk = String::with_capacity(len);
    chunk.push(gen_chunk_start_char::<I>(rng, dist));
    for _ in 1..len {
        chunk.push(gen_chunk_continue_char::<I>(rng, dist));
    }
    chunk
}

// -----------------------------------------------------------------------------
fn gen_ident_start<I: Identifier + ?Sized>(
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
    len: usize,
) -> String {
    let mut chunk = String::with_capacity(len);
    chunk.push(gen_ident_start_char::<I>(rng, dist));
    for _ in 1..len {
        chunk.push(gen_chunk_continue_char::<I>(rng, dist));
    }
    chunk
}

// -----------------------------------------------------------------------------
fn gen_ident<I: Identifier + ?Sized>(
    rng: &mut SmallRng,
    dist: &impl Distribution<char>,
    delims: &[char],
    delims_dist: &impl Distribution<usize>,
    len: usize,
    mut chunks: usize,
    untyped_validation: impl FnOnce(&str) -> bool,
) -> String {
    if chunks > len || chunks == 0 {
        chunks = 1;
    }
    let chunk_sizes = std::cmp::max(len / chunks, 1);

    let mut ident = String::with_capacity(len);
    ident.push_str(&gen_ident_start::<I>(rng, dist, chunk_sizes));
    for _ in 1..chunks {
        ident.push(delims[delims_dist.sample(rng)]);
        ident.push_str(&gen_ident_chunk::<I>(rng, dist, chunk_sizes));
    }

    // Test the identifier before returning it.
    assert!(I::new(&ident).is_ok());
    assert!(crate::validate::baseline(&ident));
    assert!(untyped_validation(&ident));

    ident
}

// -----------------------------------------------------------------------------
fn gen_idents<I: Identifier + ?Sized>(
    delims: &[char],
    untyped_validation: impl Fn(&str) -> bool,
    utf8: &impl Distribution<char>,
) -> Vec<String> {
    let mut rng = SmallRng::seed_from_u64(1);
    let chunks = Uniform::new_inclusive(1, MAX_IDENTIFIER_CHUNKS).unwrap();
    let length = Uniform::new_inclusive(MIN_IDENTIFIER_LENGTH, MAX_IDENTIFIER_LENGTH).unwrap();
    let delims_dist = Uniform::new(0, delims.len()).unwrap();

    let mut identifiers = Vec::with_capacity(POINTS_MAX);
    for _ in 0..POINTS_MAX {
        let length = length.sample(&mut rng);
        let chunks = chunks.sample(&mut rng);
        identifiers.push(gen_ident::<I>(
            &mut rng,
            &utf8,
            delims,
            &delims_dist,
            length,
            chunks,
            &untyped_validation,
        ));
    }
    identifiers
}

// =============================================================================
// GENERATORS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn ascii<I: Identifier + ?Sized>(
    delims: &[char],
    untyped_validation: impl Fn(&str) -> bool,
) -> Vec<String> {
    gen_idents::<I>(
        delims,
        untyped_validation,
        &Uniform::new_inclusive(0x20 as char, 0x7F as char).unwrap(),
    )
}

// -----------------------------------------------------------------------------
pub fn unicode<I: Identifier + ?Sized>(
    delims: &[char],
    untyped_validation: impl Fn(&str) -> bool,
) -> Vec<String> {
    gen_idents::<I>(
        delims,
        untyped_validation,
        &Uniform::new_inclusive(0x20 as char, char::MAX).unwrap(),
    )
}
