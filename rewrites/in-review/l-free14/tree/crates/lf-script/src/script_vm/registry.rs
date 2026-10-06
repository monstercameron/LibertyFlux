//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the `script_vm_*` free-function
//! group: the altitude-gated restart/area routines, the pack-and-skip
//! routines, the bullet/box/extent tests, the blip marker pair, the text
//! key pair and the two singles. `Proven` means the routine is restated
//! on its owning type and the differential test crate ran it against its
//! verified rewrite on the same generated inputs, comparing results and
//! every effect, with a deliberately wrong lift caught alongside. Counts
//! below come from this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on its owning type and proven against its rewrite.
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
    /// Naming-lane name of the verified routine.
    pub func: &'static str,
    /// Lifted method, e.g. `"AltitudeGate::register_restart"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the `script_vm_*` group.
pub const ROWS: &[Row] = &[
    Row {
        func: "script_vm_altitude_pack_a",
        method: "AltitudeGate::register_restart",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_b",
        method: "AltitudeGate::register_restart",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_c",
        method: "AltitudeGate::clear_area",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_d",
        method: "AltitudeGate::clear_area_cars",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_e",
        method: "AltitudeGate::register_point",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_f",
        method: "AltitudeGate::register_point",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_g",
        method: "AltitudeGate::clear_objects",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_altitude_pack_h",
        method: "AltitudeGate::register_point",
        state: State::Proven,
        narrows: &[
            "the threshold global becomes the gate's owned word",
            "the original writes its own z slot back; the lift keeps z and the proof compares the packed triple instead",
        ],
    },
    Row {
        func: "script_vm_pack_words_flag0",
        method: "PointSkip::emit",
        state: State::Proven,
        narrows: &[
            "the frame buffer becomes the point triple; the proof snapshots the forwarded words",
        ],
    },
    Row {
        func: "script_vm_pack_words_flag1",
        method: "PointSkip::emit",
        state: State::Proven,
        narrows: &[
            "the frame buffer becomes the point triple; the proof snapshots the forwarded words",
        ],
    },
    Row {
        func: "script_vm_pack_words_trailflag",
        method: "PointSkip::emit",
        state: State::Proven,
        narrows: &[
            "the frame buffer becomes the point triple; the proof snapshots the forwarded words",
        ],
    },
    Row {
        func: "script_vm_pack_point_3d",
        method: "AreaProbe::test_point",
        state: State::Proven,
        narrows: &[
            "the frame buffer becomes the point triple; the proof snapshots the forwarded words",
        ],
    },
    Row {
        func: "script_vm_normalize_box_corners",
        method: "AreaProbe::test_box",
        state: State::Proven,
        narrows: &[
            "the two frame triples become the corner arrays; the proof snapshots the forwarded words",
        ],
    },
    Row {
        func: "script_vm_test_expanded_bounds",
        method: "AreaProbe::test_expanded",
        state: State::Proven,
        narrows: &[
            "the two frame views become the row and padded arrays; the proof snapshots the forwarded words",
            "the relocated routine word travels as an opaque handle",
        ],
    },
    Row {
        func: "script_vm_entity_apply_offset",
        method: "BlipTable::apply_offset",
        state: State::Proven,
        narrows: &[
            "the row-pointer table becomes owned rows; indexes past the store panic while the original reads past it",
            "the fallback global becomes the table's owned index",
        ],
    },
    Row {
        func: "script_vm_entity_read_offset",
        method: "BlipTable::read_offset",
        state: State::Proven,
        narrows: &[
            "the row-pointer table becomes owned rows; indexes past the store panic while the original reads past it",
            "the output pointer becomes the returned words; the original answers the pointer itself",
            "the fourth word is zero: the original reads uninitialized scratch, pinned to zero by the verified contract",
            "miss answers other than 0xFFFF_FFFF (e.g. 0xFFFF_FFFE) are out of domain: the original wraps the table address",
        ],
    },
    Row {
        func: "script_vm_resolve_named_key",
        method: "(not lifted)",
        state: State::Missing,
        narrows: &[
            "not reached: needs the text-entry trait design (state probe, formatter, resolver, cookie check)",
        ],
    },
    Row {
        func: "script_vm_dual_key_dispatch",
        method: "(not lifted)",
        state: State::Missing,
        narrows: &[
            "not reached: the largest routine in the group (seven callees, four globals); needs the text-entry and draw-call designs",
        ],
    },
    Row {
        func: "script_vm_pack_color_verts",
        method: "LoadingClock::draw",
        state: State::Proven,
        narrows: &["the frame array becomes the frame words; the proof snapshots every window"],
    },
    Row {
        func: "script_vm_angled_area_pack",
        method: "AngledArea::clear",
        state: State::Proven,
        narrows: &[
            "the two frame triples become the triple arrays; the proof snapshots the forwarded words",
        ],
    },
];

/// Counts (proven, missing) over [`ROWS`].
#[must_use]
pub fn counts() -> (usize, usize) {
    let mut proven = 0;
    let mut missing = 0;
    for row in ROWS {
        match row.state {
            State::Proven => proven += 1,
            State::Missing => missing += 1,
            State::Lifted => {}
        }
    }
    (proven, missing)
}
