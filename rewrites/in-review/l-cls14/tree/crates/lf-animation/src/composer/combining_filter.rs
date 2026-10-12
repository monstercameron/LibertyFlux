//! The combining filter: a node of the motion composer that answers a poll
//! by asking its child filters in order.
//!
//! A combining filter owns an ordered list of child filters and one mode
//! word. Polling it asks each child with the same three arguments. A child
//! whose mode bit is set is an OR-term: one accepting answer accepts the
//! whole poll at once. A child whose bit is clear is an AND-term: one
//! rejecting answer rejects the whole poll at once. A poll that reaches the
//! end of the list accepts, and so does a poll with no children. Only the
//! low byte of a child's answer is its verdict. The mode word has 32 bits,
//! so a child from the thirty-third on has no bit and is an AND-term.
//!
//! The same word is the accumulator that [`CombiningFilter::fold`] reads,
//! because the original keeps one word that both methods read.
//!
//! Operating-system and thread state do not enter this module. The thread's
//! filter owner is found by the original through its thread-local block;
//! here the caller passes the owner to [`CombiningFilter::initialise`].
//! The verified rewrites are proven against this module by the
//! `lf-composerdiff` test crate (three levels up to the repository root).

use lf_core::boundary::Handle32;

/// Number of child filters a mode word can mark as OR-terms.
pub const MODE_BITS: usize = 32;

/// Flag bit that asks [`CombiningFilter::initialise`] to register the
/// filter with its owner.
pub const REGISTER_FLAG: u32 = 1;

/// Rotation applied to the base answer before it is folded into the word.
pub const FOLD_ROTATE: u32 = 16;

/// A child filter, polled through its poll method.
pub trait PollChild {
    /// Polls the child with the three poll arguments and returns its
    /// answer. Only the low byte of the answer is a verdict.
    fn poll(&mut self, a0: u32, a1: u32, a2: u32) -> u32;
}

/// The thread's filter owner: the object a filter registers itself with
/// when it is initialised with the registration flag.
pub trait FilterOwner {
    /// Registers `filter` with the owner. `None` stands for a filter with
    /// no identity.
    fn register_filter(&mut self, filter: Option<Handle32<CombiningFilter>>);
}

/// Callees of [`CombiningFilter::fold`]. The base filter class answers for
/// the object; its meaning is not established by the verified rewrite.
pub trait BaseFilterCallees {
    /// Asks the base filter class for its answer on `filter`.
    fn base_answer(&mut self, filter: Option<Handle32<CombiningFilter>>) -> u32;
}

/// A combining filter: its children in polling order and its mode word.
#[derive(Default)]
pub struct CombiningFilter {
    children: Vec<Box<dyn PollChild>>,
    mode: u32,
}

impl CombiningFilter {
    /// A filter with the given children, in polling order, and mode word.
    #[must_use]
    pub fn new(children: Vec<Box<dyn PollChild>>, mode: u32) -> Self {
        Self { children, mode }
    }

    /// The mode word, which is also the fold accumulator.
    #[must_use]
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// Number of child filters.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Initialises the filter: it is left with no children, and when the
    /// flags carry the registration bit it registers with the owner. The
    /// mode word is not changed. Returns the filter's identity.
    ///
    /// The owner is consulted only when the registration bit is set.
    ///
    /// # Panics
    ///
    /// When the registration bit is set and no owner is given. The original
    /// would call through a missing owner, so this case is outside the
    /// domain.
    pub fn initialise<O: FilterOwner + ?Sized>(
        &mut self,
        identity: Option<Handle32<CombiningFilter>>,
        flags: u32,
        owner: Option<&mut O>,
    ) -> Option<Handle32<CombiningFilter>> {
        self.children.clear();
        if flags & REGISTER_FLAG != 0 {
            let owner = owner.expect("registration asked for without an owner");
            owner.register_filter(identity);
        }
        identity
    }

    /// Polls the children in order and reports whether the poll accepts.
    ///
    /// An OR-term that accepts decides the poll at once. An AND-term that
    /// rejects decides it at once. Surviving every child accepts, and an
    /// empty list accepts at once.
    pub fn poll(&mut self, a0: u32, a1: u32, a2: u32) -> bool {
        let mode = self.mode;
        for (index, child) in self.children.iter_mut().enumerate() {
            let or_term = index < MODE_BITS && (mode >> index) & 1 != 0;
            let verdict = (child.poll(a0, a1, a2) as u8) != 0;
            if or_term {
                if verdict {
                    return true;
                }
            } else if !verdict {
                return false;
            }
        }
        true
    }

    /// Folds the base filter's answer into the mode word and returns the
    /// result. The word itself is not changed: the original returns the
    /// fold and stores nothing.
    pub fn fold<B: BaseFilterCallees + ?Sized>(
        &self,
        identity: Option<Handle32<CombiningFilter>>,
        base: &mut B,
    ) -> u32 {
        let answer = base.base_answer(identity);
        self.mode ^ answer.rotate_left(FOLD_ROTATE)
    }
}
