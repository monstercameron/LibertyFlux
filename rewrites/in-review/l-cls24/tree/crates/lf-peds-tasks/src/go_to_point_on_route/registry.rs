//! Registry of the go-to-point-on-route family: one row per method of the
//! class, with its state and what each proof narrows.
//!
//! State values:
//! - `Proven`: lifted here and proven against its verified 32-bit rewrite
//!   by the differential crate `lf-gotopoint-diff`.
//! - `Unlifted`: verified, not lifted yet; the reason is in `note`.
//! - `Unverified`: not verified by any checker run; not lifted and not
//!   written from its name.
//!
//! Method names are the class's own names. Two virtual tables carry the
//! class: the primary table and the table of the second base. A slot number
//! names a method only together with its table, so the rows say which.

/// How far a method has been lifted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Lifted here and proven against its verified rewrite.
    Proven,
    /// Verified, but not lifted yet.
    Unlifted,
    /// Not verified; not lifted.
    Unverified,
}

/// One method of the family.
#[derive(Clone, Copy, Debug)]
pub struct Method {
    /// The method's name in the class, with its table and slot.
    pub name: &'static str,
    /// Where it stands.
    pub state: State,
    /// What the proof narrows, or why the method is not lifted.
    pub note: &'static str,
}

/// Every method of the family that has a verified rewrite, and the
/// unverified ones the lift leaves out.
pub const METHODS: &[Method] = &[
    Method {
        name: "primary table slot 10 (progress is positive)",
        state: State::Proven,
        note: "Returns the answer as a bool; the rewrite returns it as 1 or 0. NaN and zeros are not positive. Proven on generated progress words that include NaN payloads, signed zeros and the infinities.",
    },
    Method {
        name: "second table slot 1 (speed clamp)",
        state: State::Proven,
        note: "The rewrite returns nothing the lift reproduces; the lift returns nothing. The upper bound 3.0 is a literal in the rewrite and in the lift; the original reads the same value from the executable's data, so the proof cannot see a change to that data word (rule 6 of the brief: the rewrite should load it from the image).",
    },
    Method {
        name: "second table slot 7 (position point copy)",
        state: State::Proven,
        note: "The rewrite copies three words into a caller's output block and returns that block's address. The lift returns the three words; the proof writes them into an output block and compares the block, and does not compare the returned address, which the caller already holds.",
    },
    Method {
        name: "second table slot 9 (position set)",
        state: State::Proven,
        note: "Only the low byte of the force word is tested; the proof includes force words whose low byte is zero but whose other bytes are not. The first stack word is never read and is not a parameter. The source is read by value; the rewrite's reads of the source are not observable.",
    },
    Method {
        name: "second table slot 10 (value 0x8C set)",
        state: State::Proven,
        note: "The equal-means-untouched rule is proven with zero and negative zero pairs. The method returns nothing.",
    },
    Method {
        name: "primary table slot 5 (refresh and mode)",
        state: State::Proven,
        note: "The route refresh is a host call with the task and one word; the proof scripts writes into the flag word from the refresh, so the flag read after the call is checked, which the verified contract's script (no writes) does not exercise. The first stack argument is passed to the refresh; the third is never read.",
    },
    Method {
        name: "primary table slot 1 (clone)",
        state: State::Proven,
        note: "The allocator is a host call; the manager word the allocator receives is read by the host from the image's data, and the proof compares the manager word the host passes, not the way the host reads it. The constructor is a host call with seven arguments; the two block addresses are compared as offsets from the task. The constructor itself is not lifted and not verified: the proof treats it as a callee with a scripted answer. A null allocation returns 0 without the constructor call.",
    },
    Method {
        name: "primary table slot 22 (advance progress)",
        state: State::Proven,
        note: "The shared cache (a flag word and three cached words, filled on first use) is an owned value the caller passes in; the lift holds no global state. The ped's link flags and position are host reads, each made only where the verified method makes it. The scale helper is one host call with the bits of its double argument; the proof compares the two argument words and the answer it narrows to a float. The target point is passed by value and read by the caller; the rewrite reads it only on its working path. The return register is not compared: the rewrite's own notes say its value differs by path. The constants 1.5, 2.0, 3.0, 4.0, 1.1 and the double of one eighth of pi are literals in the rewrite and in the lift; the original loads them from the executable's data, so the proof cannot see a change to those words (rule 6). The generated cache flags reach both the first-use and the filled path of each cached word.",
    },
    Method {
        name: "primary table slot 20 (route follow step)",
        state: State::Unlifted,
        note: "Verified (2018 bytes, about 440 instructions, eleven callees and several data reads). Not lifted in this lane: its host trait would need one method per callee with its argument order, and its proof a script for each. Not reached in the time available.",
    },
    Method {
        name: "constructor",
        state: State::Unverified,
        note: "No verified rewrite. The clone calls it as a host call; it is not lifted.",
    },
    Method {
        name: "destructor thunk",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "update heading",
        state: State::Unverified,
        note: "No verified rewrite. It forwards to a sibling class's method.",
    },
    Method {
        name: "arrival radius getter",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "task id getter",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "field 0x14 pointer getter (vehicle subsystem entry)",
        state: State::Unverified,
        note: "No verified rewrite; the dossier lists it under the vehicle subsystem, not this family.",
    },
];
