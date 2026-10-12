//! The word-test instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::word_test::{Compare, Predicate, Rhs, Source};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<Predicate>; 9] = [
    Named {
        name: "html_text_format_is_empty",
        spec: Predicate {
            source: Source::Field(0x238),
            compare: Compare::Equal,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "CPedDataCop::vf6",
        spec: Predicate {
            source: Source::Field(0x54),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(0xffffffff),
        },
    },
    Named {
        name: "stream_mgr_has_pending",
        spec: Predicate {
            source: Source::Field(0x9a8),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "audio_flag_134_is_set",
        spec: Predicate {
            source: Source::Field(0x134),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "audio_flag_130_is_set",
        spec: Predicate {
            source: Source::Field(0x130),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "vehicle_head_equals_arg",
        spec: Predicate {
            source: Source::Field(0x0),
            compare: Compare::Equal,
            rhs: Rhs::Parameter,
        },
    },
    Named {
        name: "scrolling_menu_value_is_positive",
        spec: Predicate {
            source: Source::Field(0x208),
            compare: Compare::SignedGreater,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "scrolling_row_has_item",
        spec: Predicate {
            source: Source::Field(0x1e4),
            compare: Compare::NotEqual,
            rhs: Rhs::Constant(0x0),
        },
    },
    Named {
        name: "scrolling_row_has_next",
        spec: Predicate {
            source: Source::Field(0x204),
            compare: Compare::SignedGreater,
            rhs: Rhs::Constant(0x0),
        },
    },
];
