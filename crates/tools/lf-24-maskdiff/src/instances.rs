//! The byte-flag instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::byte_flag_test::ByteFlagTest;

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<ByteFlagTest>; 4] = [
    Named {
        name: "CVehicle::vf74",
        spec: ByteFlagTest {
            offset: 0xf1d,
            mask: 0x10,
        },
    },
    Named {
        name: "CVehicle::vf81",
        spec: ByteFlagTest {
            offset: 0xf1d,
            mask: 0x40,
        },
    },
    Named {
        name: "stream_is_flagged",
        spec: ByteFlagTest {
            offset: 0x360,
            mask: 0x3,
        },
    },
    Named {
        name: "read_bit6_byte_at_f0",
        spec: ByteFlagTest {
            offset: 0xf0,
            mask: 0x40,
        },
    },
];
