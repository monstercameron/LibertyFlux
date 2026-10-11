//! What is lifted, what is proven, and what each proof leaves out.
//!
//! One row per verified 32-bit routine of the covered structures. `Proven`
//! means the routine is restated on its owning type and the differential
//! test crate ran it against its verified rewrite on the same generated
//! inputs, comparing results and every effect, with a deliberately wrong
//! lift caught alongside. Counts below come from this table.

/// Lift state of one verified routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Restated on the owning type and proven against its rewrite.
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
    /// Owning structure short name, e.g. `"PoseVolume"`.
    pub class: &'static str,
    /// Routine name in the 32-bit form, e.g. `"pose_transform"`.
    pub method: &'static str,
    /// Lift state.
    pub state: State,
    /// What the lift narrows away, or why the routine is missing. Empty
    /// means the proof compares everything the rewrite does.
    pub narrows: &'static [&'static str],
}

/// Every verified routine of the covered structures, grouped by structure.
pub const ROWS: &[Row] = &[
    // PoseVolume: the pose trio. All three verified routines of this
    // object in the group are lifted; the shape is complete.
    Row {
        class: "PoseVolume",
        method: "pose_transform",
        state: State::Proven,
        narrows: &[
            "the w slots are zero: the original copied an uninitialised scratch word there and the rewrite pins it to zero, which is what the lift matches",
            "the link build/fetch pair travels as the LinkMatrix trait: the proof scripts the stubs, compares call order and count, and asserts the stub addresses against the planted link and matrix",
            "the second build site is unreachable (nothing runs between the two reads, and a null after the first build faults at the first vector): mirrored in the lift, not exercised",
            "a build that leaves no matrix faults in the original and panics in the lift (host-proven)",
        ],
    },
    Row {
        class: "PoseVolume",
        method: "pose_blend",
        state: State::Proven,
        narrows: &[
            "the fill slot (thiscall on the object, inferred to be the transform above from its identical signature, object and lane) travels as the PoseFill trait: the proof scripts both sides and compares the call",
            "the out2 address answer narrows to the computed values: the proof compares the written words",
            "the one-half factor is a caller-supplied value for the relocated tuning word",
        ],
    },
    Row {
        class: "PoseVolume",
        method: "cone_test",
        state: State::Proven,
        narrows: &[
            "the fill slot travels as the PoseFill trait, as in the blend",
            "the angle, cosine-like and sine-like callees travel as ConeSolvers methods: the proof scripts answers on both sides and compares arguments bit for bit",
            "the four normaliser callees share one trait method, told apart by NormSlot: the proof pins the slot order and the count immediate",
            "the six tuning words (half, one, half-pi, tau, two bit masks) are caller-supplied values for the relocated words",
            "angle-wrap loops are exercised only over sane angles and a positive tau; an infinite angle or a non-positive tau would not terminate, as in the original",
        ],
    },
    // BuildManagers: the kind-0x11 builder pair. One generic method over
    // the trailing words, proven once per arity.
    Row {
        class: "BuildManagers",
        method: "build4",
        state: State::Proven,
        narrows: &[
            "the ped slot read (task pointer, flag byte) travels as the ped_task context call: the proof plants the ped image and scripts the same outcome",
            "a null ped lookup faults in the original and panics in the lift (host-proven)",
            "the trailing kind word is passed explicitly so the proof compares it; the four-word cdecl shape is pinned by the stub",
            "task answers travel as opaque handles; the proof compares their raw words with the rewrite's answers",
        ],
    },
    Row {
        class: "BuildManagers",
        method: "build5",
        state: State::Proven,
        narrows: &["as build4, with the five-word cdecl shape pinned by the stub"],
    },
    // ChainCloner: the entry-chain clone pair. One generic method over
    // the done-flag write, proven once per instance.
    Row {
        class: "ChainCloner",
        method: "clone_chain_a",
        state: State::Proven,
        narrows: &[
            "the chain lookup, makers, setters and direct stores travel as ChainClone methods: the proof scripts entries and products, compares every call in order with its arguments, and compares the product image words the rewrite writes",
            "entry and product addresses travel as opaque handles; the proof compares their raw words with the stub answers",
            "the constant 0 answer narrows to ()",
        ],
    },
    Row {
        class: "ChainCloner",
        method: "clone_chain_b",
        state: State::Proven,
        narrows: &[
            "as clone_chain_a, with no done-flag write: the proof asserts mark_done is never called and the flag image word keeps its planted value",
        ],
    },
    // MoverPose: the mover trio. The pose and the blend state block are
    // the owned data; the family's callees and virtual methods travel as
    // MoverCallees.
    Row {
        class: "MoverPose",
        method: "lerp_position_heading",
        state: State::Proven,
        narrows: &[
            "the height setter (the state block's virtual slot) travels as MoverCallees::set_height; the proof checks that the rewrite passes its trailing padding word as zero, which the lift does not model",
            "the target pose and the state block are planted as images; the callees are assumed not to write the pose or the block, which the lift cannot observe through a shared reference",
            "the height passes through the slot as bits: a signalling NaN height would be quieted by the x87 return convention of a getter, so the proof's heights are quiet",
        ],
    },
    Row {
        class: "MoverPose",
        method: "drive_blend",
        state: State::Proven,
        narrows: &[
            "the blend into the state block (callee id 4 of the rewrite, same argument shape as the lerp routine) is an opaque MoverCallees::blend_into: the proof compares its call, arguments and count, not its effect; that it is the lerp routine is inferred from the shape and not established by a callee map",
            "the setup, readiness, encode callees and the height setter travel as MoverCallees methods; the proof compares every call in order with its arguments",
            "the target is a handle, compared as a raw word (null is None)",
            "the callees are assumed not to write the pose or the state block",
        ],
    },
    Row {
        class: "MoverPose",
        method: "commit_blend",
        state: State::Proven,
        narrows: &[
            "the flag and state notifications, the three decoders and the height getter travel as MoverCallees methods; the proof compares every call in order with its arguments and answers",
            "the state block's height getter is read after the state notification, as in the original; the callees are assumed not to write the pose or the block",
            "the flag byte keeps its other bits, as the rewrite does; only bit 0 is written by the family",
        ],
    },
];

/// Counts of (proven, lifted-but-unproven, missing) rows.
#[must_use]
pub fn counts() -> (usize, usize, usize) {
    let mut proven = 0;
    let mut lifted = 0;
    let mut missing = 0;
    for row in ROWS {
        match row.state {
            State::Proven => proven += 1,
            State::Lifted => lifted += 1,
            State::Missing => missing += 1,
        }
    }
    (proven, lifted, missing)
}
