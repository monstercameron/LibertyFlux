//! The word copies the verified rewrites were checked against: one entry per rewrite instance.
//!
//! Each entry gives the symbol name of a routine and the two words it copies between, as word
//! numbers of a [`StaticWords`](crate::static_list::StaticWords) table of [`WORD_COUNT`] words. The
//! test crate `lf-copydiff` binds each word number to the address the rewrite uses, and runs every
//! entry against its verified rewrite.

use super::WordCopy;
use crate::static_list::WordId;

/// The number of words the instance table uses.
pub const WORD_COUNT: usize = 15;

/// One routine's word copy, with its symbol name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedCopy {
    /// The routine's symbol name.
    pub name: &'static str,
    /// The copy the routine performs.
    pub copy: WordCopy,
}

/// The instances, in the order the test crate calls their rewrites.
pub const COPIES: [NamedCopy; 11] = [
    NamedCopy {
        name: "store_float_to_1b4b330",
        copy: WordCopy {
            source: WordId(0),
            destination: WordId(1),
        },
    },
    NamedCopy {
        name: "store_float_to_1b4b2f4",
        copy: WordCopy {
            source: WordId(0),
            destination: WordId(2),
        },
    },
    NamedCopy {
        name: "store_float_to_1b4b2c8",
        copy: WordCopy {
            source: WordId(0),
            destination: WordId(3),
        },
    },
    NamedCopy {
        name: "store_float_to_1b4b2ec",
        copy: WordCopy {
            source: WordId(0),
            destination: WordId(4),
        },
    },
    NamedCopy {
        name: "copy_timing_factor_1",
        copy: WordCopy {
            source: WordId(0),
            destination: WordId(5),
        },
    },
    NamedCopy {
        name: "task_float_copy_00E66FC0",
        copy: WordCopy {
            source: WordId(6),
            destination: WordId(7),
        },
    },
    NamedCopy {
        name: "task_float_copy_00E67010",
        copy: WordCopy {
            source: WordId(8),
            destination: WordId(6),
        },
    },
    NamedCopy {
        name: "task_float_copy_00E670C0",
        copy: WordCopy {
            source: WordId(8),
            destination: WordId(9),
        },
    },
    NamedCopy {
        name: "float_const_copy",
        copy: WordCopy {
            source: WordId(8),
            destination: WordId(10),
        },
    },
    NamedCopy {
        name: "veh_float_copy_50",
        copy: WordCopy {
            source: WordId(11),
            destination: WordId(12),
        },
    },
    NamedCopy {
        name: "veh_float_copy_90",
        copy: WordCopy {
            source: WordId(13),
            destination: WordId(14),
        },
    },
];
