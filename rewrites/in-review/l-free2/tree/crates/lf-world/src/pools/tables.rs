//! Small lookup tables built on the pools: tag search and bounded search.
//!
//! Lifted from the two verified pure search routines. Both take every
//! address as the value behind it and answer indexes instead of -1.

/// One tag pool: rows of (key, payload, payload).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagPool {
    /// Row keys, in row order.
    pub tags: Vec<u32>,
    /// First payload word per row.
    pub w0: Vec<u32>,
    /// Second payload word per row.
    pub w1: Vec<u32>,
}

impl TagPool {
    /// A pool over parallel row vectors.
    ///
    /// # Panics
    ///
    /// When the vectors are not equally long.
    #[must_use]
    pub fn from_rows(tags: Vec<u32>, w0: Vec<u32>, w1: Vec<u32>) -> Self {
        assert!(
            tags.len() == w0.len() && tags.len() == w1.len(),
            "ragged tag pool: {} tags, {} + {} payloads",
            tags.len(),
            w0.len(),
            w1.len()
        );
        Self { tags, w0, w1 }
    }
}

/// Six tag pools searched as one object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagPools {
    /// The six pools, selected by kind 0..6.
    pub pools: [TagPool; 6],
}

impl TagPools {
    /// Finds the first row at or after `start` in pool `kind` whose key
    /// equals `key`, writing its payloads through `out0` (masked to 24
    /// bits, as the 32-bit form zeroes the top byte) and `out1`.
    ///
    /// Answers `None` when the kind is unknown, the pool is empty, or no
    /// tag matches. The 32-bit form builds each row's tag as an address
    /// (`object + base + key * 16`); the lift compares the keys, which is
    /// what the addresses distinguish.
    ///
    /// # Panics
    ///
    /// When `start` is at or above 2^31 (the 32-bit form reads the pools
    /// at negative row indexes there).
    pub fn find(
        &self,
        kind: u32,
        key: u32,
        start: u32,
        out0: &mut u32,
        out1: &mut u32,
    ) -> Option<usize> {
        if kind > 5 {
            return None;
        }
        assert!(
            start < 0x8000_0000,
            "search start {start:#x} is negative as a row index"
        );
        let pool = &self.pools[kind as usize];
        let mut i = start as usize;
        while i < pool.tags.len() {
            if pool.tags[i] == key {
                *out0 = pool.w0[i] & 0x00FF_FFFF;
                *out1 = pool.w1[i];
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

/// A word array searched over a bounded slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordTable {
    /// The words.
    pub words: Vec<u32>,
}

impl WordTable {
    /// Searches `words[start .. start + count]` for `want`, answering the
    /// first matching index. Every bound is signed exactly as in the
    /// 32-bit form: a negative `start` or `count`, a `start` at or past
    /// the word count, a wrapped or over-long end above it, and an empty
    /// range all answer `None` without reading the array.
    ///
    /// # Panics
    ///
    /// When the table holds more than `i32::MAX` words.
    #[must_use]
    pub fn search(&self, want: u32, start: i32, count: i32) -> Option<usize> {
        let limit = i32::try_from(self.words.len())
            .expect("tables larger than i32::MAX words are out of domain");
        if start < 0 || start >= limit {
            return None;
        }
        if count < 0 {
            return None;
        }
        let end = start.wrapping_add(count);
        if end > limit || start >= end {
            return None;
        }
        let from = usize::try_from(start).expect("start checked in range");
        let to = usize::try_from(end).expect("end checked in range");
        for (j, word) in self.words[from..to].iter().enumerate() {
            if *word == want {
                return Some(from + j);
            }
        }
        None
    }
}
