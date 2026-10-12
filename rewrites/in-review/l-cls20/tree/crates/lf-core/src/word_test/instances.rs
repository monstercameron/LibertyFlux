//! The word tests the verified rewrites were checked against: one entry per rewrite instance.
//!
//! Each entry gives the symbol name of a routine and its test. A global source is a number in a static
//! word table; the test crate `lf-flagdiff` binds each number to the address its rewrite reads and runs
//! every entry against its verified rewrite.

use super::{Compare, Predicate, Rhs, Source};
use crate::static_list::WordId;

/// The number of static words the global instances use.
pub const GLOBAL_WORDS: usize = 2;

/// One routine's word test, with its symbol name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedPredicate {
    /// The routine's symbol name.
    pub name: &'static str,
    /// The test the routine makes.
    pub predicate: Predicate,
}

/// The instances, in the order the test crate calls their rewrites.
pub const PREDICATES: [NamedPredicate; 6] = [
    NamedPredicate {
        name: "audPoliceScanner::is_scanning",
        predicate: Predicate {
            source: Source::Field(0xB0C),
            compare: Compare::Equal,
            rhs: Rhs::Constant(2),
        },
    },
    NamedPredicate {
        name: "check_state_equals_one",
        predicate: Predicate {
            source: Source::Field(0x114),
            compare: Compare::Equal,
            rhs: Rhs::Constant(1),
        },
    },
    NamedPredicate {
        name: "audStaticRadioEmitter::vf5",
        predicate: Predicate {
            source: Source::Field(0x30),
            compare: Compare::Equal,
            rhs: Rhs::Parameter,
        },
    },
    NamedPredicate {
        name: "object_field_10_not_200",
        predicate: Predicate {
            source: Source::Field(0x10),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(200),
        },
    },
    NamedPredicate {
        name: "report_whether_the_streaming_state_word",
        predicate: Predicate {
            source: Source::Global(WordId(0)),
            compare: Compare::Equal,
            rhs: Rhs::Constant(1),
        },
    },
    NamedPredicate {
        name: "NativeImpl_GET_PLAYER_HAS_TRACKS",
        predicate: Predicate {
            source: Source::Global(WordId(1)),
            compare: Compare::SignedGreater,
            rhs: Rhs::Constant(2),
        },
    },
];
