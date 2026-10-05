//! Registry: which slots are lifted, and what each proof narrows.
//!
//! Counts of verified members come from a census of
//! `rewrites/verified/index.json` on 5 October 2026. `proven` records the
//! differential run made by the lane that wrote the lift, the same day; the
//! 32-bit test crate that makes that run is not in the workspace yet, so the
//! pipeline has not repeated it.

/// One lifted slot and the shape of its proof.
#[derive(Clone, Copy, Debug)]
pub struct SlotRecord {
    /// Virtual slot name (`vf6`, `vf7`, ...).
    pub slot: &'static str,
    /// Lifted function name.
    pub lifted: &'static str,
    /// Verified rewrites of this slot in the census.
    pub members: u32,
    /// Whether the differential proof covers this slot yet.
    pub proven: bool,
    /// What the proof narrows or leaves out (empty when the proof is full
    /// over the stated domain).
    pub narrowings: &'static [&'static str],
}

/// The family's slots.
pub const SLOTS: &[SlotRecord] = &[
    SlotRecord {
        slot: "vf6",
        lifted: "tables::find_index",
        members: 457,
        proven: true,
        narrowings: &[
            "scan bound counts tested 1..=300 plus non-positive signed counts; larger positive counts untested (same code path)",
            "tables back at least `count` words whenever the count is positive (stated domain; the lift panics otherwise)",
        ],
    },
    SlotRecord {
        slot: "vf7",
        lifted: "tables::fetch_id",
        members: 455,
        proven: true,
        narrowings: &[
            "index inside the table (stated domain; the original reads unchecked and the lift panics outside)",
        ],
    },
    SlotRecord {
        slot: "vf8",
        lifted: "tables::class_tag",
        members: 456,
        proven: true,
        narrowings: &[
            "index inside the table (stated domain, as vf7)",
            "classifier answers cover 0..=6, u32::MAX and random values",
        ],
    },
    SlotRecord {
        slot: "vf9",
        lifted: "tables::class_rank",
        members: 57,
        proven: true,
        narrowings: &[
            "index inside the table (stated domain, as vf7)",
            "classifier answers cover 0..=6, u32::MAX and random values",
        ],
    },
    SlotRecord {
        slot: "vf12",
        lifted: "tables::reverse_lookup",
        members: 456,
        proven: true,
        narrowings: &[
            "key index inside the primary table (stated domain, as vf7)",
            "unsigned scan counts tested 1..=300; larger counts untested (same code path)",
        ],
    },
    SlotRecord {
        slot: "vf13",
        lifted: "tables::joined_fetch",
        members: 457,
        proven: true,
        narrowings: &[
            "scan bound counts tested 1..=300 plus non-positive signed counts; larger positive counts untested (same code path)",
            "tables back at least `count` words whenever the count is positive (stated domain)",
        ],
    },
    SlotRecord {
        slot: "vf2",
        lifted: "probe::query_tag",
        members: 444,
        proven: true,
        narrowings: &[
            "result narrowed from the out-address to Option<Tag>; the 32-bit return-address shape and the no-store cases are pinned by the test",
            "six rewrites hardcode the tag instead of relocating it (checker v4 legacy): proven under identity mapping only",
        ],
    },
    SlotRecord {
        slot: "vf14",
        lifted: "rows::collect",
        members: 413,
        proven: true,
        narrowings: &[
            "row keys inside the key table (stated domain; the original reads unchecked)",
            "length check is signed, as the original's is; the run recorded here was made while 144 members still compared unsigned and covered those on lengths where the two agree, and they have since been re-proven signed, so the run is to be repeated",
            "out buffers valid (the original faults on null; the lift takes &mut)",
        ],
    },
    SlotRecord {
        slot: "ctor",
        lifted: "(designed, not proven)",
        members: 440,
        proven: false,
        narrowings: &[
            "not lifted yet: base call, two per-board vtables, member init, flag bit; one one-off small-object shape",
        ],
    },
];
