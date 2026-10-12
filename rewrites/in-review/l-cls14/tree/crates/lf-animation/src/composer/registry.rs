//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit method of the composer classes this module
//! covers. `Proven` means the method is restated on its type and the
//! `lf-composerdiff` test crate ran it against its verified rewrite on the
//! same generated inputs, comparing results, every call in order and every
//! written word, with a deliberately wrong lift caught alongside. `Missing`
//! means the method is verified but not lifted here, with the reason. The
//! counts below come from this table.

/// Lift state of one verified method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its type and proven against its rewrite.
    Proven,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified method's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// Class, e.g. `"CombiningFilter"`.
    pub class: &'static str,
    /// Slot or method name in the 32-bit form, e.g. `"vf3"`.
    pub method: &'static str,
    /// The lifted name, or the reason it is not lifted.
    pub lifted_as: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away. Empty means the proof compares everything
    /// the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified method of the composer classes, in class order.
pub const ROWS: &[Row] = &[
    Row {
        class: "CombiningFilter",
        method: "vf0",
        lifted_as: "CombiningFilter::initialise",
        state: State::Proven,
        narrows: &[
            "the vtable word the original installs is not held by the lift; the harness checks it is the class's vtable",
            "the child count word is the length of the child list; the lift has no separate count",
            "the owner is found by the caller (thread-local block read); the lift takes it as an argument",
            "registration without an owner is outside the domain (panics); a null filter faults in the original, so the proof never passes one",
        ],
    },
    Row {
        class: "CombiningFilter",
        method: "vf3",
        lifted_as: "CombiningFilter::poll",
        state: State::Proven,
        narrows: &[
            "a child count of zero or less is an empty child list; the original's signed test is not represented separately",
            "children are polled through a boxed trait object; the child's own memory is outside the lift",
        ],
    },
    Row {
        class: "CombiningFilter",
        method: "vf4",
        lifted_as: "CombiningFilter::fold",
        state: State::Proven,
        narrows: &[
            "the base-class callee is given no access to the filter: the lift reads the mode word after the call and the proof scripts the callee to write nothing",
            "the rewrite stores nothing; the lift returns the fold and changes nothing",
        ],
    },
    Row {
        class: "ComposerOptimizedData",
        method: "vf0",
        lifted_as: "ComposerOptimizedData::scalar_deleting_destructor",
        state: State::Proven,
        narrows: &[
            "the thread's manager is found by the caller (thread-local block read); the lift takes it as an argument",
            "the free of the block is a call to the manager; the lift has no allocation to release, so the owner drops the block after the call",
            "a block to be freed without a manager is outside the domain (panics)",
            "the proof scripts both callees to write no block memory; their own effects are the trait implementers' responsibility",
        ],
    },
    Row {
        class: "ComposerOptimizedData",
        method: "vf2",
        lifted_as: "ComposerOptimizedData::reset_tables",
        state: State::Proven,
        narrows: &[
            "a value entry's two words are modelled; the other bytes of each entry's stride are not held and the harness checks they are unchanged",
            "a pointer-table entry holds a block-relative offset; the harness adds the block's address on the original's side",
            "the header callee is scripted to write no block memory; its effects are the trait implementers' responsibility",
            "the returned offset is the block-relative form of the address the original returns",
        ],
    },
];

/// Verified methods proven against their rewrites.
#[must_use]
pub fn proven_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Proven).count()
}

/// Verified methods not lifted here.
#[must_use]
pub fn missing_count() -> usize {
    ROWS.iter().filter(|r| r.state == State::Missing).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_proven_row_names_what_it_narrows() {
        for row in ROWS.iter().filter(|r| r.state == State::Proven) {
            assert!(!row.narrows.is_empty(), "{} {} narrows nothing listed", row.class, row.method);
        }
    }
}
