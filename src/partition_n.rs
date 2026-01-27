// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

/// Extension trait for partitioning an iterator into N buckets.
pub trait PartitionN: Iterator {
    /// Partitions items into `N` collections based on a classifier function.
    ///
    /// # Example
    ///
    /// ```
    /// use your_crate::PartitionN;
    ///
    /// let [evens, odds]: [Vec<_>; 2] = (0..10).partition_n(|n| n % 2);
    /// assert_eq!(evens, [0, 2, 4, 6, 8]);
    /// assert_eq!(odds, [1, 3, 5, 7, 9]);
    /// ```
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
