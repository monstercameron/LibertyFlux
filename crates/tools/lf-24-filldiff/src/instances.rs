//! The table-fill instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// What a fill returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Returns {
    /// Nothing: the routine returns no value.
    Unit,
    /// Zero.
    Zero,
    /// The constant the table is filled with.
    Fill,
}

/// One table fill: the global address of the table's first word, the number of words, the constant, and what
/// the routine returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fill {
    /// The global address of the first word.
    pub base: u32,
    /// The number of consecutive words filled.
    pub count: usize,
    /// The constant written to each word.
    pub value: u32,
    /// What the routine returns.
    pub returns: Returns,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<Fill>; 6] = [
    Named {
        name: "veh_table_fill_neg1_8320 #0",
        spec: Fill {
            base: 0x12fae88,
            count: 0xc4,
            value: 0xffffffff,
            returns: Returns::Unit,
        },
    },
    Named {
        name: "veh_id_table_reset #1",
        spec: Fill {
            base: 0x158e660,
            count: 0x80,
            value: 0xffffffff,
            returns: Returns::Zero,
        },
    },
    Named {
        name: "veh_id_table_reset #2",
        spec: Fill {
            base: 0x15928c0,
            count: 0x80,
            value: 0xffffffff,
            returns: Returns::Zero,
        },
    },
    Named {
        name: "veh_id_table_reset #3",
        spec: Fill {
            base: 0x1600320,
            count: 0x320,
            value: 0xffffffff,
            returns: Returns::Zero,
        },
    },
    Named {
        name: "veh_table_fill_80 #4",
        spec: Fill {
            base: 0x178e7c8,
            count: 0x71c,
            value: 0xffffffff,
            returns: Returns::Zero,
        },
    },
    Named {
        name: "initialize_vehicle_index_table_sentinels #5",
        spec: Fill {
            base: 0x179d128,
            count: 0x200,
            value: 0xffffffff,
            returns: Returns::Fill,
        },
    },
];
