macro_rules! test_as_ref_comprehensive {
    ($from:ident as $to:ident) => {{
        use crate::syntax::boundary::*;
        use crate::syntax::delimiter::*;
        use crate::syntax::profile::*;

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Ascii>::new("a")?;
        let _: &$to<Standard, LowLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Ascii>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Ascii>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Strict>::new("a")?;
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Strict>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Strict>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Unicode>::new("a")?;
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Unicode>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Unicode>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Lower<Ascii>>::new("a")?;
        let _: &$to<Standard, LowLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Lower<Ascii>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Lower<Ascii>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Lower<Strict>>::new("a")?;
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Lower<Strict>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Lower<Strict>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Lower<Unicode>>::new("a")?;
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Lower<Unicode>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Lower<Unicode>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Lower<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Upper<Ascii>>::new("A")?;
        let _: &$to<Standard, LowLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Upper<Ascii>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Upper<Ascii>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Upper<Strict>>::new("A")?;
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Upper<Strict>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Upper<Strict>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, Upper<Unicode>>::new("A")?;
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, Upper<Unicode>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, Upper<Unicode>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Upper<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, LowerCamel<Ascii>>::new("a")?;
        let _: &$to<Standard, LowLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, LowerCamel<Ascii>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, LowerCamel<Ascii>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, LowerCamel<Strict>>::new("a")?;
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, LowerCamel<Strict>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, LowerCamel<Strict>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, LowerCamel<Unicode>>::new("a")?;
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, LowerCamel<Unicode>>::new("a")?;
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, LowerCamel<Unicode>>::new("a")?;
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, LowerCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, LowerCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, UpperCamel<Ascii>>::new("A")?;
        let _: &$to<Standard, LowLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, UpperCamel<Ascii>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, UpperCamel<Ascii>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Ascii> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Ascii> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Ascii>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, UpperCamel<Strict>>::new("A")?;
        let _: &$to<Standard, LowLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, UpperCamel<Strict>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, UpperCamel<Strict>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Strict> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Strict> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Strict>> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        let fragment = $from::<Standard, LowLine, UpperCamel<Unicode>>::new("A")?;
        let _: &$to<Standard, LowLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, LowLine, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, AsciiFlatLine, UpperCamel<Unicode>>::new("A")?;
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();

        let fragment = $from::<Standard, HyphenMinus, UpperCamel<Unicode>>::new("A")?;
        let _: &$to<Standard, HyphenMinus, Unicode> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, Unicode> = fragment.as_ref();
        let _: &$to<Standard, HyphenMinus, UpperCamel<Unicode>> = fragment.as_ref();
        let _: &$to<Standard, AsciiFlatLine, UpperCamel<Unicode>> = fragment.as_ref();
    }};
}
