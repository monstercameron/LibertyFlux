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
pub const INSTANCES: [Named; 1] = [
    Named {
        name: "veh_zone_slot_ptr",
        spec: RowAddress {
            base: BaseSource::Global,
            index: IndexSource::Parameter,
            stride: 0x28,
            offset: 0x0,
        },
        base_va: 0x15fcce0,
    },
];
