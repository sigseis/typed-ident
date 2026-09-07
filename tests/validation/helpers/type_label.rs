// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use std::fmt::Formatter;
use typed_ident::Ident;
use typed_ident::syntax::boundary::{Options, Standard};
use typed_ident::syntax::delimiter::{AsciiFlatLine, HyphenMinus, LowLine};
use typed_ident::syntax::profile::case::{Lower, LowerCamel, Upper, UpperCamel};
use typed_ident::syntax::profile::chars::{Ascii, CharProfile, Strict, Unicode};

// =============================================================================
// TRAITS
// =============================================================================

// -----------------------------------------------------------------------------
pub struct Display(fn(&mut Formatter<'_>) -> std::fmt::Result);
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

// -----------------------------------------------------------------------------
pub trait TypeLabel {
    fn type_label() -> Display {
        Display(Self::type_label_fmt)
    }
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result;
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn boundary_type_label_fmt<O: Options>(f: &mut Formatter<'_>, name: &str) -> std::fmt::Result {
    let options = &[
        if O::CAMEL { Some("Camel") } else { None },
        if O::HAT { Some("Hat") } else { None },
        if O::DIGIT_TO_LOWER {
            Some("DtoL")
        } else {
            None
        },
        if O::DIGIT_TO_UPPER {
            Some("DtoU")
        } else {
            None
        },
        if O::LOWER_TO_DIGIT {
            Some("LtoD")
        } else {
            None
        },
        if O::UPPER_TO_DIGIT {
            Some("UtoD")
        } else {
            None
        },
    ];

    let mut iter = options.iter().copied().flatten();
    f.write_str(name)?;
    let Some(opt) = iter.next() else {
        return Ok(());
    };
    f.write_str("<")?;
    f.write_str(opt)?;
    for opt in iter {
        f.write_str(", ")?;
        f.write_str(opt)?;
    }
    f.write_str(">")
}

// =============================================================================
// TRAITS IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<O: Options> TypeLabel for Standard<O> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        boundary_type_label_fmt::<O>(f, "Standard")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for AsciiFlatLine {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AsciiFlatLine")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for HyphenMinus {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("HyphenMinus")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for LowLine {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("LowLine")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for Ascii {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ascii")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for Strict {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Strict")
    }
}

// -----------------------------------------------------------------------------
impl TypeLabel for Unicode {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unicode")
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile + TypeLabel> TypeLabel for Lower<P> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lower<")?;
        P::type_label_fmt(f)?;
        f.write_str(">")
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile + TypeLabel> TypeLabel for LowerCamel<P> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("LowerCamel<")?;
        P::type_label_fmt(f)?;
        f.write_str(">")
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile + TypeLabel> TypeLabel for Upper<P> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Upper<")?;
        P::type_label_fmt(f)?;
        f.write_str(">")
    }
}

// -----------------------------------------------------------------------------
impl<P: CharProfile + TypeLabel> TypeLabel for UpperCamel<P> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("UpperCamel<")?;
        P::type_label_fmt(f)?;
        f.write_str(">")
    }
}

// -----------------------------------------------------------------------------
impl<B: TypeLabel, D: TypeLabel, P: TypeLabel> TypeLabel for Ident<B, D, P> {
    fn type_label_fmt(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ident<")?;
        B::type_label_fmt(f)?;
        f.write_str(", ")?;
        D::type_label_fmt(f)?;
        f.write_str(", ")?;
        P::type_label_fmt(f)?;
        f.write_str(">")
    }
}
