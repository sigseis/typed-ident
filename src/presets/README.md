# Format Table

| **Delimiter**             | **Mixed**       | **Cased**            | **Lower** or **Upper**                      |
|---------------------------|-----------------|----------------------|---------------------------------------------|
| **Low Line (`_`)**        | [`CamelIdent`]  | [`CasedCamelIdent`]  | [`LowerCamelIdent`]  / [`UpperCamelIdent`]  |
| **Flat Line (`_` / `-`)** | [`HybridIdent`] | [`CasedHybridIdent`] | [`LowerHybridIdent`] / [`UpperHybridIdent`] |
| **Hyphen-Minus (`-`)**    | [`KebabIdent`]  | [`CasedKebabIdent`]  | [`LowerKebabIdent`]  / [`UpperKebabIdent`]  |
| **Low Line (`_`)**        | [`SnakeIdent`]  | [`CasedSnakeIdent`]  | [`LowerSnakeIdent`]  / [`UpperSnakeIdent`]  |

All formats are organized from less-restrictive (left) to more-restrictive (right), with **Lower** and **Upper** both providing different restrictions rather than one being more restrictive than the other.
