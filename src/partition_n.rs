// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

pub trait PartitionN: Iterator {
    fn partition_n<B, const N: usize>(self, f: impl FnMut(&Self::Item) -> usize) -> [B; N]
    where
        B: Default + Extend<Self::Item>;
}

impl<I: Iterator> PartitionN for I {
    fn partition_n<B, const N: usize>(self, mut f: impl FnMut(&Self::Item) -> usize) -> [B; N]
    where
        B: Default + Extend<Self::Item>,
    {
        let mut result: [B; N] = std::array::from_fn(|_| B::default());
        for item in self {
            result[f(&item) % N].extend(std::iter::once(item));
        }
        result
    }
}
