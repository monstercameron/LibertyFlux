//! The row-address instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The
//! differential test runs the rewrite at index `i` and the lifted routine on the same generated inputs. `base_va`
//! is the global the rewrite names: the table's base, or the pointer stored there.

use lf_core::row_address::{BaseSource, IndexSource, RowAddress};

/// One instance: the symbol name of its rewrite, the family parameters it was extracted to, and the global.
#[derive(Clone, Copy, Debug)]
pub struct Named {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: RowAddress,
    /// The address of the global the rewrite names.
    pub base_va: u32,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named; 4] = [
    Named {
        name: "stride_table_entry",
        spec: RowAddress {
            base: BaseSource::Global,
            index: IndexSource::Field(0x32a4),
            stride: 0xbc,
            offset: 0x0,
        },
        base_va: 0x118d470,
    },
    Named {
        name: "NativeImpl_GET_ANIM_GROUP_FROM_CHAR_2",
        spec: RowAddress {
            base: BaseSource::PointerInGlobal,
            index: IndexSource::Parameter,
            stride: 0x58,
            offset: 0x0,
        },
        base_va: 0x16dd640,
    },
    Named {
        name: "anim_group_name",
        spec: RowAddress {
            base: BaseSource::PointerInGlobal,
            index: IndexSource::Parameter,
            stride: 0x58,
            offset: 0x20,
        },
        base_va: 0x16dd640,
    },
    Named {
        name: "animation_record_at_index",
        spec: RowAddress {
            base: BaseSource::Global,
            index: IndexSource::Parameter,
            stride: 0x110,
            offset: 0x0,
        },
        base_va: 0x16dd6a0,
    },
];
