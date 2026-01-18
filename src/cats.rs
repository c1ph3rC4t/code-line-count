// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

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
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum CategoryID {
            $($variant),+
        }

        impl CategoryID {
            pub const fn all_ids() -> &'static [Self] {
                &[$(Self::$variant),+]
            }

            pub const fn all_names() -> &'static [&'static str] {
                &[$($($name),+),+]
            }

            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($($name)|+ => Some(Self::$variant),)+
                    _ => None,
                }
            }

            pub const fn names(self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant => &[$($name),+]),+
                }
            }

            pub const fn extensions(self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant => &[$($ext),*]),+
                }
            }
        }
    };
}
