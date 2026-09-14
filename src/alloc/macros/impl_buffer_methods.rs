macro_rules! impl_buffer_methods {
    (
        name=$name:ident,
        op=$op:ident,
    ) => {
        impl<B: Boundary, D: Delimiter, P: CasedProfile> $name<B, D, P> {
            /// Constructs an ident buffer, initializing the contents to a provided
            /// string slice (attempting first to convert the string slice to a valid
            /// ident).
            ///
            /// This is equivalent to `IdentBuf::from_fragment(Fragment::new(s)?)`.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use presets::unicode::upper_camel::UpperCamelIdentBuf;
            /// assert!(UpperCamelIdentBuf::from_str("").is_err());
            /// assert!(UpperCamelIdentBuf::from_str("ValidUpperCamel").is_ok());
            /// assert!(UpperCamelIdentBuf::from_str("continuingUpperCamel").is_err());
            /// assert!(UpperCamelIdentBuf::from_str("not_validUpperCamel").is_err());
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            #[allow(clippy::should_implement_trait)] // It *does* implement the trait.
            pub fn from_str(s: &str) -> Result<Self, Error> {
                core::str::FromStr::from_str(s)
            }

            /// Constructs an ident buffer, initializing the contents to a provided
            /// buffered string (checking first that the string is a valid ident).
            ///
            /// This is similar to [`from_str`], except that it will not allocate a
            /// separate string. It will use the provided string, if it's valid.
            ///
            /// [`from_str`]: Self::from_str
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use presets::unicode::upper_camel::UpperCamelIdentBuf;
            /// assert!(UpperCamelIdentBuf::from_string(String::from("")).is_err());
            /// assert!(UpperCamelIdentBuf::from_string(String::from("ValidUpperCamel")).is_ok());
            /// assert!(UpperCamelIdentBuf::from_string(String::from("continuingUpperCamel")).is_err());
            /// assert!(UpperCamelIdentBuf::from_string(String::from("not_validUpperCamel")).is_err());
            /// # Ok::<(), Error>(())
            #[inline]
            pub fn from_string(s: String) -> Result<Self, Error> {
                crate::alloc::buffer::$op::<B, D, P>::check_str(s.as_str())?;
                Ok(Self::from_string_unchecked(s))
            }

            #[doc = include_str!("docs/methods/insert.md")]
            #[doc = include_str!("docs/sections/unicode_warning.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::from_str("UpperCamel")?;"#)]
            ///
            /// // Inserting at the beginning is ~prepend.
            /// let mut example = buffer.clone();
            /// assert!(example.insert(0, 'V').is_ok());
            /// assert!(example.insert(0, '_').is_ok());
            /// assert_eq!(example, "_VUpperCamel");
            ///
            /// // Inserting at the end is ~append.
            /// let mut example = buffer.clone();
            /// example.push('_')?;
            /// assert!(example.insert(example.len(), 'i').is_err());
            /// assert!(example.insert(example.len(), 'V').is_ok());
            /// assert_eq!(example, "UpperCamel_V");
            ///
            /// // Inserting in the middle can be tricky, as your insertions
            /// // may invalidate the buffer's invariants in surprising ways.
            /// let mut example = buffer.clone();
            /// assert!(example.insert(2, '_').is_err()); // "Up_perCamel" != UpperCamel casing
            /// assert!(example.insert(5, '_').is_ok());
            /// assert_eq!(example, "Upper_Camel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn insert<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
            where
                F: IntoIntermediate<B, D, P>,
            {
                let fragment = fragment.into_intermediate()?;
                crate::alloc::buffer::$op::new(self).insert_str(idx, fragment.as_ref())
            }

            #[doc = include_str!("docs/methods/insert_bounded.md")]
            #[doc = include_str!("docs/sections/unicode_warning.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::from_str("UpperCamel")?;"#)]
            ///
            /// // Inserting at the beginning is ~prepend.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded(0, 'V').is_ok()); // Bounded because of `HAT` rules.
            /// assert!(example.insert_bounded(0, 'V').is_ok()); // But another `V` would not be.
            /// assert_eq!(example, "V_VUpperCamel");
            ///
            /// // Inserting at the end is ~append.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded(example.len(), 'i').is_err()); // "UpperCamel_i" != UpperCamel casing
            /// assert!(example.insert_bounded(example.len(), 'V').is_ok()); // Because of `CAMEL` boundary.
            /// assert_eq!(example, "UpperCamelV");
            ///
            /// // Inserting in the middle can be tricky, as your insertions
            /// // may invalidate the buffer's invariants in surprising ways.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded(2, 'V').is_err()); // "Up_V_perCamel" != UpperCamel casing
            /// assert!(example.insert_bounded(5, 'V').is_ok()); // Surprisingly a `CAMEL` & `HAT` boundary.
            /// assert_eq!(example, "UpperVCamel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn insert_bounded<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
            where
                D: Default,
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_bounded_with(idx, fragment, Default::default())
            }

            #[doc = include_str!("docs/methods/insert_bounded_fragment.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
            ///
            /// // Inserting at the beginning is ~prepend.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded_with(
            ///     0,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_ok()); // Bounded because of `HAT` rules.
            /// assert!(example.insert_bounded_with(
            ///     0,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_ok()); // But another would not be.
            /// assert_eq!(example, "HAT_HATUpperCamel");
            ///
            /// // Inserting at the end is ~append.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded_with(
            ///     example.len(),
            ///     UpperCamelFragment::new("lower")?,
            ///     LowLine,
            /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
            /// assert!(example.insert_bounded_with(
            ///     example.len(),
            ///     UpperCamelFragment::new("Camel")?,
            ///     LowLine,
            /// ).is_ok()); // Because of `CAMEL` boundary.
            /// assert_eq!(example, "UpperCamelCamel");
            ///
            /// // Inserting in the middle can be tricky, as your insertions
            /// // may invalidate the buffer's invariants in surprising ways.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_bounded_with(
            ///     2,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_err()); // "UpHAT_perCamel" != UpperCamel casing
            /// assert!(example.insert_bounded_with(
            ///     5,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_ok()); // Surprisingly a `CAMEL` & `HAT` boundary.
            /// assert_eq!(example, "UpperHATCamel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn insert_bounded_with<F>(
                &mut self,
                idx: usize,
                fragment: F,
                delim: D,
            ) -> Result<(), Error>
            where
                F: IntoIntermediate<B, D, P>,
                {
                let fragment = fragment.into_intermediate()?;
                crate::alloc::buffer::$op::new(self).insert_bounded_str(idx, fragment.as_ref(), delim.as_char())
            }

            #[doc = include_str!("docs/methods/insert_delimited.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::from_str("UpperCamel")?;"#)]
            ///
            /// // Inserting at the beginning is ~prepend.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited(0, 'V').is_ok());
            /// assert_eq!(example, "V_UpperCamel");
            ///
            /// // Inserting at the end is ~append.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited(example.len(), 'i').is_err()); // "UpperCamel_i" != UpperCamel casing
            /// assert!(example.insert_delimited(example.len(), 'V').is_ok());
            /// assert_eq!(example, "UpperCamel_V");
            ///
            /// // Inserting in the middle can be tricky, as your insertions
            /// // may invalidate the buffer's invariants in surprising ways.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited(2, 'V').is_err()); // "Up_V_perCamel" != UpperCamel casing
            /// assert!(example.insert_delimited(5, 'V').is_ok());
            /// assert_eq!(example, "Upper_V_Camel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn insert_delimited<F>(&mut self, idx: usize, fragment: F) -> Result<(), Error>
            where
                D: Default,
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_delimited_with(idx, fragment, D::default())
            }

            #[doc = include_str!("docs/methods/insert_delimited_fragment.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use presets::unicode::upper_camel::*;
            /// let mut buffer = UpperCamelFragmentBuf::from_str("UpperCamel")?;
            ///
            /// // Inserting at the beginning is ~prepend.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited_with(
            ///     0,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_ok());
            /// assert_eq!(example, "HAT_UpperCamel");
            ///
            /// // Inserting at the end is ~append.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited_with(
            ///     example.len(),
            ///     UpperCamelFragment::new("lower")?,
            ///     LowLine,
            /// ).is_err()); // "UpperCamel_lower" != UpperCamel casing
            /// assert!(example.insert_delimited_with(
            ///     example.len(),
            ///     UpperCamelFragment::new("Camel")?,
            ///     LowLine,
            /// ).is_ok());
            /// assert_eq!(example, "UpperCamel_Camel");
            ///
            /// // Inserting in the middle can be tricky, as your insertions
            /// // may invalidate the buffer's invariants in surprising ways.
            /// let mut example = buffer.clone();
            /// assert!(example.insert_delimited_with(
            ///     2,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_err()); // "Up_HAT_perCamel" != UpperCamel casing
            /// assert!(example.insert_delimited_with(
            ///     5,
            ///     UpperCamelFragment::new("HAT")?,
            ///     LowLine,
            /// ).is_ok());
            /// assert_eq!(example, "Upper_HAT_Camel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn insert_delimited_with<F>(
                &mut self,
                idx: usize,
                fragment: F,
                delim: D,
            ) -> Result<(), Error> where
                F: IntoIntermediate<B, D, P>,
                {
                let fragment = fragment.into_intermediate()?;
                crate::alloc::buffer::$op::new(self).insert_delimited_str(idx, fragment.as_ref(), delim.as_char())
            }

            #[doc = include_str!("docs/methods/push.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::new();"#)]
            ///
            /// // You can only push characters that are valid at the given position.
            /// assert!(buffer.push('O').is_ok());
            /// assert!(buffer.push('k').is_ok());
            /// assert_eq!(buffer, "Ok");
            ///
            /// // But you have to be mindful of the format to avoid pushing
            /// // invalid characters. Most commonly, after delimiters.
            /// buffer.push('_')?;
            /// assert!(buffer.push('i').is_err());
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn push<F>(&mut self, fragment: F) -> Result<(), Error> where
                F: IntoIntermediate<B, D, P>,
            {
                self.insert(self.len(), fragment)
            }

            #[doc = include_str!("docs/methods/push_bounded.md")]
            #[doc = include_str!("docs/sections/unicode_warning.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::new();"#)]
            ///
            /// // If there's no data at the start, there's nothing to bound against.
            /// assert!(buffer.push_bounded('O').is_ok());
            /// assert_eq!(buffer, "O");
            ///
            /// // But the very next bounded push would need to bound the contents.
            /// // As such, you cannot select an invalid character.
            /// assert!(buffer.push_bounded('k').is_err()); // "O_k" != UpperCamel casing
            /// assert!(buffer.push_bounded('K').is_ok());
            /// assert_eq!(buffer, "O_K");
            ///
            /// // This won't add a delimiter if the chunks are already bounded.
            /// assert!(buffer.push('o').is_ok()); // Regular push to get a lowercase.
            /// assert!(buffer.push_bounded('K').is_ok()); // A `CAMEL` boundary char.
            /// assert_eq!(buffer, "O_KoK");
            ///
            /// // If there's already a delimiter, another would not be inserted.
            /// assert!(buffer.push(LowLine).is_ok());
            /// assert!(buffer.push_bounded('K').is_ok());
            /// assert_eq!(buffer, "O_KoK_K");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn push_bounded<F>(&mut self, fragment: F) -> Result<(), Error>
            where
                D: Default,
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_bounded(self.len(), fragment)
            }

            #[doc = include_str!("docs/methods/push_bounded.md")]
            #[doc = include_str!("docs/sections/unicode_warning.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::new();"#)]
            ///
            /// // If there's no data at the start, there's nothing to bound against.
            /// assert!(buffer.push_bounded_with('O', LowLine).is_ok());
            /// assert_eq!(buffer, "O");
            ///
            /// // But the very next bounded push would need to bound the contents.
            /// // As such, you cannot select an invalid character.
            /// assert!(buffer.push_bounded_with('k', LowLine).is_err()); // "O_k" != UpperCamel casing
            /// assert!(buffer.push_bounded_with('K', LowLine).is_ok());
            /// assert_eq!(buffer, "O_K");
            ///
            /// // This won't add a delimiter if the chunks are already bounded.
            /// assert!(buffer.push('o').is_ok()); // Regular push to get a lowercase.
            /// assert!(buffer.push_bounded_with('K', LowLine).is_ok()); // A `CAMEL` boundary char.
            /// assert_eq!(buffer, "O_KoK");
            ///
            /// // If there's already a delimiter, another would not be inserted.
            /// assert!(buffer.push(LowLine).is_ok());
            /// assert!(buffer.push_bounded_with('K', LowLine).is_ok());
            /// assert_eq!(buffer, "O_KoK_K");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn push_bounded_with<F>(&mut self, fragment: F, delim: D) -> Result<(), Error>
            where
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_bounded_with(self.len(), fragment, delim)
            }

            #[doc = include_str!("docs/methods/push_delimited.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::new();"#)]
            ///
            /// // If there's no data at the start, there's nothing to delimit against.
            /// assert!(buffer.push_delimited('O').is_ok());
            /// assert_eq!(buffer, "O");
            ///
            /// // But the very next delimited push would need to delimit the contents.
            /// // As such, you cannot select an invalid character.
            /// assert!(buffer.push_delimited('k').is_err()); // "O_k" != UpperCamel casing
            /// assert!(buffer.push_delimited('K').is_ok());
            /// assert_eq!(buffer, "O_K");
            ///
            /// // If there's already a delimiter, another would not be inserted.
            /// assert!(buffer.push(LowLine).is_ok());
            /// assert!(buffer.push_delimited('K').is_ok());
            /// assert_eq!(buffer, "O_K_K");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn push_delimited<F>(&mut self, fragment: F) -> Result<(), Error>
            where
                D: Default,
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_delimited(self.len(), fragment)
            }

            #[doc = include_str!("docs/methods/push_delimited.md")]
            #[doc = include_str!("docs/sections/errors.md")]
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::new();"#)]
            ///
            /// // If there's no data at the start, there's nothing to delimit against.
            /// assert!(buffer.push_delimited_with('O', LowLine).is_ok());
            /// assert_eq!(buffer, "O");
            ///
            /// // But the very next delimited push would need to delimit the contents.
            /// // As such, you cannot select an invalid character.
            /// assert!(buffer.push_delimited_with('k', LowLine).is_err()); // "O_k" != UpperCamel casing
            /// assert!(buffer.push_delimited_with('K', LowLine).is_ok());
            /// assert_eq!(buffer, "O_K");
            ///
            /// // If there's already a delimiter, another would not be inserted.
            /// assert!(buffer.push(LowLine).is_ok());
            /// assert!(buffer.push_delimited_with('K', LowLine).is_ok());
            /// assert_eq!(buffer, "O_K_K");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn push_delimited_with<F>(&mut self, fragment: F, delim: D) -> Result<(), Error>
            where
                F: IntoIntermediate<B, D, P>,
            {
                self.insert_delimited_with(self.len(), fragment, delim)
            }

            #[doc = include_str!("docs/methods/remove.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            ///
            /// # Errors
            ///
            /// If the removal of the character at the provided index would lead to an
            /// invalid buffer, then the character will not be remove and instead the
            /// error `FailedRemove` will be returned.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use presets::unicode::upper_camel::UpperCamelFragmentBuf;
            /// let mut buffer = UpperCamelFragmentBuf::from_str("Upper_Camel")?;
            ///
            /// // This would be valid, because it might be a continuation fragment.
            /// assert!(buffer.remove(0).is_ok());
            /// assert_eq!(buffer, "pper_Camel");
            ///
            /// // However, attempting to remove `C` would fail for `UpperCamel`.
            /// assert!(buffer.remove(5).is_err());
            /// assert_eq!(buffer, "pper_Camel");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn remove(&mut self, idx: usize) -> Result<(), Error> {
                crate::alloc::buffer::$op::new(self).remove(idx)
            }

            #[doc = include_str!("docs/methods/replace_range_str.md")]
            #[doc = include_str!("docs/sections/panics.md")]
            ///
            /// # Errors
            ///
            /// If the replacement of the range provided with the given fragment would
            /// lead to an invalid buffer, then the range will not be remove and instead
            /// the error `InvalidReplace` will be returned.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::*;
            /// # use typed_ident::syntax::delimiter::*;
            /// # use typed_ident::presets::unicode::upper_camel::*;
            #[doc = concat!("let mut buffer = UpperCamel", stringify!($name), r#"::from_str("Upper_Camel")?;"#)]
            ///
            /// // Examples replacing various ranges.
            /// let mut example = buffer.clone();
            /// assert!(example.replace_range(4..7, "R").is_ok());
            /// assert_eq!(example, "UppeRamel");
            ///
            /// let mut example = buffer.clone();
            /// assert!(example.replace_range(4..=7, "R").is_ok());
            /// assert_eq!(example, "UppeRmel");
            ///
            /// let mut example = buffer.clone();
            /// assert!(example.replace_range(..7, "R").is_ok());
            /// assert_eq!(example, "Ramel");
            ///
            /// let mut example = buffer.clone();
            /// assert!(example.replace_range(..=7, "R").is_ok());
            /// assert_eq!(example, "Rmel");
            ///
            /// let mut example = buffer.clone();
            /// assert!(example.replace_range(4.., "R").is_ok());
            /// assert_eq!(example, "UppeR");
            /// # Ok::<(), Error>(())
            /// ```
            #[inline]
            pub fn replace_range<R, F>(&mut self, range: R, replace_with: F) -> Result<(), Error>
            where
                R: RangeBounds<usize>,
                F: IntoIntermediate<B, D, P>,
            {
                let replace_with = replace_with.into_intermediate()?;
                crate::alloc::buffer::$op::new(self).replace_range_str(range, replace_with.as_ref())
            }
        }

        impl<B, D, P> $name<B, D, P> {
            /// Returns a reference to the fragment slice from the buffer.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// let fragment: &str = buffer.as_str();
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn as_fragment(&self) -> &Fragment<B, D, P> {
                Fragment::new_unchecked(self.inner.as_str())
            }

            /// Returns a string slice representation of the buffer data.
            ///
            /// # Examples
            ///
            /// Basic Usage:
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// let fragment: &HybridFragment = buffer.as_fragment();
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn as_str(&self) -> &str {
                self.inner.as_str()
            }

            /// Returns the underlying capacity of the buffer.
            ///
            /// This has the same properties as [`String::capacity`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::new();"#)]
            /// assert_eq!(buffer.capacity(), 0);
            /// buffer.reserve(10);
            /// assert!(buffer.capacity() >= 10);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn capacity(&self) -> usize {
                self.inner.capacity()
            }

            /// Clears the underlying fragment buffer, making it empty.
            ///
            /// This has the same properties as [`String::clear`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// assert_eq!(buffer, "example");
            /// buffer.clear();
            /// assert_eq!(buffer, "");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn clear(&mut self) {
                self.inner.clear();
            }

            /// Convert the buffer into a boxed fragment.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// let boxed: Box<HybridFragment> = buffer.into_boxed_fragment();
            /// assert_eq!(boxed.as_ref(), "example");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn into_boxed_fragment(self) -> std_alloc::boxed::Box<Fragment<B, D, P>> {
                Fragment::new_boxed_unchecked(self.into_string())
            }

            /// Convert the buffer into a boxed string slice.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// let boxed: Box<str> = buffer.into_boxed_str();
            /// assert_eq!(boxed.as_ref(), "example");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn into_boxed_str(self) -> std_alloc::boxed::Box<str> {
                self.inner.into_boxed_str()
            }

            /// Returns `true` if the underlying buffer is empty.
            ///
            /// This has the same properties as [`String::is_empty`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// assert!(!buffer.is_empty());
            /// buffer.clear();
            /// assert!(buffer.is_empty());
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }

            /// Returns the byte length of the buffer
            ///
            /// This has the same properties as [`String::len`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("example")?;"#)]
            /// assert_eq!(buffer.len(), 7);
            /// buffer.clear();
            /// assert_eq!(buffer.len(), 0);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub const fn len(&self) -> usize {
                self.inner.len()
            }

            /// Constructs a new buffer with the capacity set to 0.
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::new();"#)]
            /// assert_eq!(buffer, "");
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[must_use]
            #[inline]
            pub fn new() -> Self {
                Self::with_capacity(0)
            }

            /// Pops the right-most character off the buffer and returns it.
            ///
            /// This has the same properties as [`String::pop`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::from_str("test")?;"#)]
            /// assert_eq!(buffer.pop(), Some('t'));
            /// assert_eq!(buffer.pop(), Some('s'));
            /// assert_eq!(buffer.pop(), Some('e'));
            /// assert_eq!(buffer.pop(), Some('t'));
            /// assert_eq!(buffer.pop(), None);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn pop(&mut self) -> Option<char> {
                self.inner.pop()
            }

            /// Reserves enough buffer space for `additional` more bytes.
            ///
            /// This has the same properties as [`String::reserve`].
            ///
            /// # Note
            ///
            /// This will over-allocate in most situations. If you need to
            /// reserve an *exact* amount of bytes, see [`reserve_exact`].
            ///
            /// [`reserve_exact`]: Self::reserve_exact
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::new();"#)]
            /// assert_eq!(buffer.capacity(), 0);
            /// buffer.reserve(10);
            /// assert!(buffer.capacity() >= 10);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn reserve(&mut self, additional: usize) {
                self.inner.reserve(additional)
            }

            /// Reserves exact buffer space for `additional` more bytes.
            ///
            /// This has the same properties as [`String::reserve_exact`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::new();"#)]
            /// assert_eq!(buffer.capacity(), 0);
            /// buffer.reserve_exact(10);
            /// assert_eq!(buffer.capacity(), 10);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn reserve_exact(&mut self, additional: usize) {
                self.inner.reserve_exact(additional)
            }

            /// Shrinks the buffer to the minimum of the actual length or the
            /// provided `min_capacity` value.
            ///
            /// This has the same properties as [`String::shrink_to`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::with_capacity(10);"#)]
            /// assert!(buffer.capacity() >= 10);
            /// buffer.shrink_to(5);
            /// assert_eq!(buffer.capacity(), 5);
            /// buffer.push("example")?;
            /// assert!(buffer.capacity() >= 7);
            /// buffer.shrink_to(0);
            /// assert_eq!(buffer.capacity(), 7);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn shrink_to(&mut self, min_capacity: usize) {
                self.inner.shrink_to(min_capacity)
            }

            /// Shrinks the buffer to the size of the content.
            ///
            /// This has the same properties as [`String::shrink_to_fit`].
            ///
            /// # Examples
            ///
            /// ```
            /// # use typed_ident::presets::unicode::hybrid::*;
            #[doc = concat!("let mut buffer = Hybrid", stringify!($name), r#"::with_capacity(10);"#)]
            /// assert!(buffer.capacity() >= 10);
            /// buffer.shrink_to_fit();
            /// assert_eq!(buffer.capacity(), 0);
            /// buffer.push("example")?;
            /// assert!(buffer.capacity() >= 7);
            /// buffer.shrink_to_fit();
            /// assert_eq!(buffer.capacity(), 7);
            /// # Ok::<(), typed_ident::Error>(())
            /// ```
            #[inline]
            pub fn shrink_to_fit(&mut self) {
                self.inner.shrink_to_fit()
            }

            /// Attempts to reserve buffer space for `additional` more bytes.
            ///
            /// This has the same properties as [`String::try_reserve`].
            #[inline]
            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
                self.inner.try_reserve(additional)
            }

            /// Attempts to reserve exact buffer space for `additional` more bytes.
            ///
            /// This has the same properties as [`String::try_reserve_exact`].
            #[inline]
            pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
                self.inner.try_reserve_exact(additional)
            }
        }
    };
}
