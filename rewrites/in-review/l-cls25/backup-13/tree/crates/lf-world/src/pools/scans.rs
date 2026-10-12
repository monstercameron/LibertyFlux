//! Small single-routine pool structures: a pair, word blocks, keyed flags.
//!
//! Three layouts from the lane's list that appear once each: a
//! two-element pool built in place ([`PairPool`]), four blocks of four
//! scattered words ([`WordBlocks`]), and sixteen keyed flag bytes
//! ([`KeyedFlags`]). Each owns its bytes as ordinary Rust data.

/// Builds one pair element in place: the pair initialiser.
pub trait ElemInit<E> {
    /// Initialises `elem`, answering a status word.
    fn init_elem(&mut self, elem: &mut E) -> u32;
}

impl<E, F: FnMut(&mut E) -> u32> ElemInit<E> for F {
    fn init_elem(&mut self, elem: &mut E) -> u32 {
        self(elem)
    }
}

/// A two-element pool with a ready flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairPool<E> {
    /// The first element.
    pub first: E,
    /// The second element.
    pub second: E,
    /// Whether the pool finished initialising.
    pub ready: bool,
}

impl<E> PairPool<E> {
    /// Initialises both elements in order and raises the ready flag,
    /// answering the second initialisation's status word.
    #[must_use]
    pub fn init(mut first: E, mut second: E, init: &mut impl ElemInit<E>) -> (Self, u32) {
        init.init_elem(&mut first);
        let answer = init.init_elem(&mut second);
        (
            Self {
                first,
                second,
                ready: true,
            },
            answer,
        )
    }
}

/// Four blocks of four scattered words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordBlocks {
    /// The words, four per block.
    pub blocks: [[u32; 4]; 4],
}

impl WordBlocks {
    /// Whether any of the sixteen words equals `value`.
    #[must_use]
    pub fn contains(&self, value: u32) -> bool {
        self.blocks.iter().any(|block| block.contains(&value))
    }
}

/// Sixteen keyed flag bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyedFlags {
    /// One key per entry.
    pub keys: [u32; 16],
    /// One flag byte per entry.
    pub flags: [u8; 16],
}

impl KeyedFlags {
    /// Clears the flag byte of every entry whose key equals `value`.
    pub fn clear_matches(&mut self, value: u32) {
        for (key, flag) in self.keys.iter().zip(self.flags.iter_mut()) {
            if *key == value {
                *flag = 0;
            }
        }
    }
}
