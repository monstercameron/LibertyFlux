//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified member of the ratio family, in bank order.
//! `Proven` means the instance is restated as [`RatioCell::refresh`](crate::veh_ratio::RatioCell::refresh)
//! and the differential test crate ran it against its verified rewrite on
//! the same generated inputs, comparing the stored quotient bit for bit
//! with a deliberately wrong lift caught alongside. Counts below come
//! from this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated as [`RatioCell::refresh`](crate::veh_ratio::RatioCell::refresh) and proven against its rewrite.
    Proven,
    /// Restated but not proven (no row should stay here: everything
    /// lifted in this module is proven).
    Lifted,
    /// Not lifted, for the stated reason.
    Missing,
}

/// One verified routine's row.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    /// The verified routine's name, e.g. `"veh_ratio_82c0"`.
    pub name: &'static str,
    /// The routine's cell index in [`RatioBank`](crate::veh_ratio::RatioBank).
    pub index: usize,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away. Empty means the proof compares
    /// everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// The dead integer return narrows to `()`.
pub const NARROW_RET: &str = "the dead integer return (0 where the rewrite has one) narrows to ()";
/// The three globals narrow to owned fields; the triple's identity is the index.
pub const NARROW_ADDR: &str =
    "the three globals travel as owned f32 fields; the triple's identity is the bank index";

/// Every verified member of the ratio family, in bank order.
pub const ROWS: &[Row] = &[
    Row {
        name: "veh_ratio_82c0",
        index: 0,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8340",
        index: 1,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8360",
        index: 2,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8380",
        index: 3,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_83a0",
        index: 4,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_83c0",
        index: 5,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_83f0",
        index: 6,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8430",
        index: 7,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8450",
        index: 8,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8470",
        index: 9,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8490",
        index: 10,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8510",
        index: 11,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8530",
        index: 12,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8570",
        index: 13,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_85b0",
        index: 14,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8630",
        index: 15,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_86b0",
        index: 16,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_86d0",
        index: 17,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8700",
        index: 18,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_8740",
        index: 19,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 20,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 21,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 22,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 23,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 24,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 25,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 26,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 27,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 28,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 29,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 30,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 31,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_recompute",
        index: 32,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69350",
        index: 33,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e693e0",
        index: 34,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69410",
        index: 35,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69440",
        index: 36,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69470",
        index: 37,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69490",
        index: 38,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e694b0",
        index: 39,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e694f0",
        index: 40,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69510",
        index: 41,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69530",
        index: 42,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69550",
        index: 43,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69570",
        index: 44,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e695e0",
        index: 45,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69650",
        index: 46,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69690",
        index: 47,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e696d0",
        index: 48,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e69750",
        index: 49,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e697a0",
        index: 50,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e697c0",
        index: 51,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_01",
        index: 52,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_02",
        index: 53,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_03",
        index: 54,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_04",
        index: 55,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_05",
        index: 56,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_06",
        index: 57,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_07",
        index: 58,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_08",
        index: 59,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_09",
        index: 60,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_10",
        index: 61,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_11",
        index: 62,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_12",
        index: 63,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_13",
        index: 64,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_14",
        index: 65,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_15",
        index: 66,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_store_16",
        index: 67,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_30",
        index: 68,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_50",
        index: 69,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_70",
        index: 70,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_d0",
        index: 71,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_f0",
        index: 72,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_10",
        index: 73,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_30",
        index: 74,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_70",
        index: 75,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_90",
        index: 76,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_b0",
        index: 77,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_e0",
        index: 78,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_00",
        index: 79,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_20",
        index: 80,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_40",
        index: 81,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_b0",
        index: 82,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_d0",
        index: 83,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_f0",
        index: 84,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_10",
        index: 85,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_80",
        index: 86,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_init_a0",
        index: 87,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_00",
        index: 88,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_01",
        index: 89,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_02",
        index: 90,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_03",
        index: 91,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_04",
        index: 92,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_05",
        index: 93,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_06",
        index: 94,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_07",
        index: 95,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_08",
        index: 96,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_09",
        index: 97,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_10",
        index: 98,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_11",
        index: 99,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_12",
        index: 100,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_13",
        index: 101,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_14",
        index: 102,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_15",
        index: 103,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_16",
        index: 104,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_17",
        index: 105,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_18",
        index: 106,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_19",
        index: 107,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_20",
        index: 108,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_21",
        index: 109,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_22",
        index: 110,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_23",
        index: 111,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
    Row {
        name: "veh_ratio_update_24",
        index: 112,
        state: State::Proven,
        narrows: &[NARROW_RET, NARROW_ADDR],
    },
];
