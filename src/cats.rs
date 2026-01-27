// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

/// A macro for creating a `CategoryID` enum
///
/// Prioritizes speed over memory footprint.
/// The generated enum derives [`Clone`], [`Copy`], [`Debug`], [`PartialEq`], and [`Eq`].
/// `from_name` is case-sensitive.
///
/// # Example
///
/// ```
/// define_categories! {
///     Rust => {
///         names: ["rust", "rs"],
///         extensions: ["rs", "rlib"],
///     },
///     Haskell => {
///         names: ["haskell", "hs"],
///         extensions: ["hs", "lhs"],
///     },
/// }
///
/// // Lookup by name
/// assert_eq!(CategoryID::from_name("rs"), Some(CategoryID::Rust));
/// assert_eq!(CategoryID::from_name("unknown"), None);
///
/// // Get extensions for a category
/// assert_eq!(CategoryID::Haskell.extensions(), &["hs", "lhs"]);
/// ```
#[macro_export]
macro_rules! define_categories {
    (
        $(
            $variant:ident => {
                names: [$($name:literal),+ $(,)?],
                extensions: [$($ext:literal),* $(,)?],
            }
        ),+ $(,)?
    ) => {
        /// A category identifier.
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum CategoryID {
            $($variant),+
        }

        impl CategoryID {
            /// Gets a list of all IDs.
            pub const fn all_ids() -> &'static [Self] {
                &[$(Self::$variant),+]
            }

            /// Gets a list of all names (order unspecified).
            pub const fn all_names() -> &'static [&'static str] {
                &[$($($name),+),+]
            }

            /// Gets an ID from a name if there is one.
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($($name)|+ => Some(Self::$variant),)+
                    _ => None,
                }
            }

            /// Gets a list of all names associated with an ID.
            pub const fn names(self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant => &[$($name),+]),+
                }
            }

            /// Gets a list of all extensions associated with an ID.
            pub const fn extensions(self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant => &[$($ext),*]),+
                }
            }
        }
    };
}
