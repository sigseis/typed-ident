// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! test_as_ref {
    (
        from=$from:ident,
        into=$into:ident,
    ) => {{
        use crate::syntax::boundary::*;
        use crate::syntax::delimiter::*;
        use crate::syntax::profile::*;

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        type TargetFrom<B, D, P> = $from<B, D, P>;
        type TargetInto<B, D, P> = $into<B, D, P>;

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Mixed<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Mixed<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Mixed<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Mixed<Strict>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Mixed<Strict>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Mixed<Strict>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Mixed<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Mixed<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Mixed<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Lower<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Lower<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Lower<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Lower<Strict>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Lower<Strict>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Lower<Strict>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Lower<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Lower<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Lower<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Lower<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Upper<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Upper<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Upper<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Upper<Strict>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Upper<Strict>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Upper<Strict>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, Upper<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, Upper<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, Upper<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Uniform<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Upper<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, LowerCamel<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, LowerCamel<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, LowerCamel<Ascii>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, LowerCamel<Strict>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, LowerCamel<Strict>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, LowerCamel<Strict>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, LowerCamel<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, LowerCamel<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, LowerCamel<Unicode>>::new("a")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, LowerCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, LowerCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, UpperCamel<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, UpperCamel<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, UpperCamel<Ascii>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Ascii>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, UpperCamel<Strict>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, UpperCamel<Strict>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, UpperCamel<Strict>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Strict>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let value = TargetFrom::<Standard, LowLine, UpperCamel<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, LowLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, LowLine, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, AsciiFlatLine, UpperCamel<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();

        let value = TargetFrom::<Standard, HyphenMinus, UpperCamel<Unicode>>::new("A")?;
        let _: &TargetInto<Standard, HyphenMinus, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Mixed<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, Camel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, HyphenMinus, UpperCamel<Unicode>> = value.as_ref();
        let _: &TargetInto<Standard, AsciiFlatLine, UpperCamel<Unicode>> = value.as_ref();
    }};
}

// =============================================================================
// EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub(crate) use test_as_ref;
