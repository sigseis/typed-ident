Syntax definitions for segmentation strategies.

These are additional syntax rules that are meant to be set on a [`Profile`].

This module defines two segmentation strategies:

* [`Char`] - segment the in-profile characters by individual character code points.
* [`Grapheme`] - segment the in-profile characters by graphemes.

# Which Should I Use?

Technically you could use either regardless of the profile, but it just changes how boundary segmentation works.

For instance, using [`Char`] on the [`Unicode`] profile would still *work*, it just may fail to identify certain chunk boundaries, because the fact that we process everything on individual character code points would lead to slightly different boundaries being identified (mainly by *removing* boundaries).

If you care really badly about efficiency, *and* you don't mind losing some of these boundaries, then you can use [`Char`], just be aware of what that means.

**I do not recommend this!**

I think you should just use what makes sense for the profile.

* If you have a profile consisting purely of ASCII characters, you can use [`Char`] segmentation.
* Otherwise, you probably should just use [`Grapheme`] segmentation.

[`Profile`]: crate::syntax::profile::Profile
[`Unicode`]: crate::syntax::profile::chars::Unicode
