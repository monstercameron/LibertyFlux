//! The machine-readable record of what has been lifted.
//!
//! One [`LiftRecord`] per verified rewrite that has a lifted form, one
//! [`DeferredRecord`] per rewrite looked at and not lifted. Counts for the
//! progress file come from here, and the differential harness asserts that
//! every record has a differential case, so a lift cannot be added without
//! its proof.

/// Which kind of lift a function got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// Arguments in, value out; no memory, globals or callees.
    Pure,
    /// Behaviour is the calls made through callee slots.
    Forwarder,
    /// Reads or writes globals (carried in a state struct).
    Globals,
}

/// One lifted function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiftRecord {
    /// The original's address (the file VA its verified rewrite names).
    pub original: u32,
    /// The verified rewrite's export name in `rewrites/verified/`.
    pub rewrite: &'static str,
    /// Path of the lifted item, relative to this crate.
    pub lifted: &'static str,
    /// Kind of lift.
    pub family: Family,
    /// What the lifted form does not carry or narrows; empty when the lift
    /// is total over the original's inputs.
    pub narrowing: &'static str,
}

/// One rewrite that was examined and deliberately not lifted yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeferredRecord {
    /// The original's address.
    pub original: u32,
    /// The verified rewrite's export name.
    pub rewrite: &'static str,
    /// Why it waits.
    pub reason: &'static str,
}

const fn pure(
    original: u32,
    rewrite: &'static str,
    lifted: &'static str,
    narrowing: &'static str,
) -> LiftRecord {
    LiftRecord {
        original,
        rewrite,
        lifted,
        family: Family::Pure,
        narrowing,
    }
}

const fn fwd(
    original: u32,
    rewrite: &'static str,
    lifted: &'static str,
    narrowing: &'static str,
) -> LiftRecord {
    LiftRecord {
        original,
        rewrite,
        lifted,
        family: Family::Forwarder,
        narrowing,
    }
}

const fn glob(
    original: u32,
    rewrite: &'static str,
    lifted: &'static str,
    narrowing: &'static str,
) -> LiftRecord {
    LiftRecord {
        original,
        rewrite,
        lifted,
        family: Family::Globals,
        narrowing,
    }
}

const NAN_PAYLOAD: &str = "NaN results are NaN; payload not part of the lifted contract";
const LOW_BYTE: &str = "returns bool; original answers in the low byte";
const DTOR: &str = "flag word narrowed to bit 0";
const SORT: &str = "range in elements (valid ranges only); placeholder and comparator arguments live in the trait implementation; callee return value dropped";

/// Every lifted function.
pub const LIFTED: &[LiftRecord] = &[
    pure(0x0088_BCD0, "rw_0088bcd0", "pure::scaled_product_floor", ""),
    pure(0x008A_6A00, "rw_008a6a00", "pure::round_half_away", ""),
    pure(
        0x008A_ADB0,
        "rw_008aadb0",
        "pure::lerp_clamped",
        NAN_PAYLOAD,
    ),
    pure(0x008D_5180, "rw_008D5180", "pure::code_for_selector", ""),
    pure(0x008D_70E0, "rw_008d70e0", "pure::clamp_map", NAN_PAYLOAD),
    pure(
        0x0091_B3C0,
        "rw_0091b3c0",
        "pure::map_char_byte",
        "byte in, byte out; residue in bits 8-31 of the original's result pinned by test",
    ),
    pure(0x0092_5E50, "rw_00925E50", "pure::input_buffer_size", ""),
    pure(0x0092_53F0, "rw_009253F0", "pure::input_flag_class", ""),
    pure(
        0x0094_B8C0,
        "rw_0094b8c0",
        "pure::bank_slot_index",
        "returns the slot index; the boundary applies the 0xC8 stride; bank flag narrowed to its low byte",
    ),
    pure(
        0x0095_2630,
        "rw_00952630",
        "pure::size_class",
        "count passed as i32",
    ),
    pure(0x0095_29E0, "rw_009529e0", "pure::kind_flag_bit", ""),
    pure(
        0x0095_32A0,
        "rw_009532a0",
        "pure::record_size_for_tag",
        "tag narrowed to its low byte",
    ),
    pure(0x0095_35D0, "rw_009535d0", "pure::band_index", ""),
    pure(0x0095_3640, "rw_00953640", "pure::bucket_limit", ""),
    pure(0x0097_B490, "rw_0097b490", "pure::audio_channel_bucket", ""),
    pure(
        0x009B_7600,
        "rw_009b7600",
        "lf_core::boundary::element_addr with pure::RECORD_0X84_STRIDE",
        "lifts to plain indexing; the address form exists only at the boundary",
    ),
    pure(0x009F_62C0, "rw_009f62c0", "pure::code_to_float", ""),
    pure(0x00A7_1CF0, "rw_00a71cf0", "pure::is_kind_4_to_6", LOW_BYTE),
    pure(0x00AB_6F50, "rw_00ab6f50", "pure::ids_match_or_null", ""),
    pure(0x00B3_1650, "rw_00b31650", "pure::task_rate", ""),
    pure(0x00B7_9210, "rw_00b79210", "pure::float_band", ""),
    pure(0x00B7_9250, "rw_00b79250", "pure::index_to_float", ""),
    pure(0x00BE_81D0, "rw_00BE81D0", "pure::pair_rejected", ""),
    pure(0x00D3_8C10, "rw_00d38c10", "pure::selector_code", ""),
    pure(
        0x00D7_40A0,
        "rw_00d740a0",
        "pure::render_mode_is_active",
        LOW_BYTE,
    ),
    pure(
        0x00D7_40E0,
        "rw_00d740e0",
        "pure::render_mode_is_shadow",
        LOW_BYTE,
    ),
    fwd(
        0x0098_15F0,
        "rw_009815f0",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x0098_56A0,
        "rw_009856a0",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00AD_D1A0,
        "rw_00add1a0",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00AD_D1C0,
        "rw_00add1c0",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00C6_E1C0,
        "rw_00c6e1c0",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00CA_4E40,
        "rw_00ca4e40",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00D8_C690,
        "rw_00d8c690",
        "forward::deleting_destructor",
        DTOR,
    ),
    fwd(
        0x00AB_BDA0,
        "rw_00abbda0",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00AD_E7A0,
        "rw_00ade7a0",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00B0_5100,
        "rw_00b05100",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00B3_3D00,
        "rw_00b33d00",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00B3_3D70,
        "rw_00b33d70",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00B3_3DE0,
        "rw_00b33de0",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00C6_DBB0,
        "rw_00c6dbb0",
        "forward::final_insertion_sort",
        SORT,
    ),
    fwd(
        0x00AD_EBD0,
        "rw_00adebd0",
        "forward::sort",
        "range in elements; byte spans of 1-3 (which never terminate in the original) are unrepresentable",
    ),
    fwd(
        0x00A7_2820,
        "rw_00a72820",
        "forward::any_gate_open",
        "gate answers narrowed to their low byte",
    ),
    fwd(
        0x00AB_A180,
        "rw_00aba180",
        "forward::dual_rank_test",
        "rank answers narrowed to their low byte",
    ),
    fwd(0x009A_3EA0, "rw_009a3ea0", "forward::probe_nonzero", ""),
    fwd(
        0x008A_C690,
        "rw_008ac690",
        "forward::effect_process",
        "returns bool; residue in bits 8-31 of the original's result pinned by test",
    ),
    glob(
        0x0095_2DB0,
        "rw_00952db0",
        "slot_table::bump_serial_a",
        "returns the low half only",
    ),
    glob(
        0x0095_2E60,
        "rw_00952e60",
        "slot_table::bump_serial_b",
        "returns the low half only",
    ),
    glob(
        0x0095_2DE0,
        "rw_00952de0",
        "slot_table::allocate",
        "cursor domain 0..=1500; used bytes narrowed to bool",
    ),
    glob(
        0x0095_3110,
        "rw_00953110",
        "slot_table::entry_value",
        "index narrowed to its low 16 bits",
    ),
    glob(
        0x0095_3210,
        "rw_00953210",
        "slot_table::entry_row",
        "returns the row index; the boundary applies the table address",
    ),
    glob(
        0x00E6_3C90,
        "rw_00e63c90",
        "slot_table::clear_entries",
        "end-of-table address result not carried",
    ),
    glob(0x0095_3900, "rw_00953900", "slot_table::clock_span", ""),
    glob(
        0x0095_3910,
        "rw_00953910",
        "slot_table::clock_span_seconds",
        "",
    ),
    glob(0x0095_2700, "rw_00952700", "slot_table::stamp_sum", ""),
    glob(
        0x0095_26D0,
        "rw_009526d0",
        "slot_table::scaled_ticks",
        NAN_PAYLOAD,
    ),
    glob(
        0x0095_3160,
        "rw_00953160",
        "slot_table::take_pending_scale",
        NAN_PAYLOAD,
    ),
    glob(0x0095_26B0, "rw_009526b0", "slot_table::stamped_base", ""),
    glob(0x0095_2660, "rw_00952660", "slot_table::key_value", ""),
];

/// Rewrites examined and not lifted yet.
pub const DEFERRED: &[DeferredRecord] = &[
    DeferredRecord {
        original: 0x0095_2C70,
        rewrite: "rw_00952c70",
        reason: "address-embedding: adds a buffer address from a table to an offset and stores the pointer; lifts with the buffers",
    },
    DeferredRecord {
        original: 0x0095_3950,
        rewrite: "rw_00953950",
        reason: "address-embedding: selects and clears ring buffers through stored pointers; lifts with the buffers",
    },
    DeferredRecord {
        original: 0x008F_00C0,
        rewrite: "rw_008f00c0",
        reason: "returns addresses of members inside a large object whose layout is not documented yet (one of six siblings)",
    },
];

#[cfg(test)]
mod tests {
    use super::{DEFERRED, Family, LIFTED};
    use std::collections::BTreeSet;

    #[test]
    fn records_are_unique_and_counted() {
        let originals: BTreeSet<u32> = LIFTED.iter().map(|r| r.original).collect();
        assert_eq!(originals.len(), LIFTED.len(), "one record per original");
        let names: BTreeSet<&str> = LIFTED.iter().map(|r| r.rewrite).collect();
        assert_eq!(names.len(), LIFTED.len());
        assert!(DEFERRED.iter().all(|d| !originals.contains(&d.original)));
        let count = |f| LIFTED.iter().filter(|r| r.family == f).count();
        assert_eq!(
            (
                count(Family::Pure),
                count(Family::Forwarder),
                count(Family::Globals)
            ),
            (26, 19, 13)
        );
    }

    #[test]
    fn rewrite_names_end_with_their_address() {
        for r in LIFTED {
            let hex = r.rewrite.trim_start_matches("rw_");
            assert_eq!(
                u32::from_str_radix(hex, 16).ok(),
                Some(r.original),
                "{}",
                r.rewrite
            );
        }
    }
}
