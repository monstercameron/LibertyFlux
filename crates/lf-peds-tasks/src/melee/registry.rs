//! Registry of the complex melee family: one row per method of the class,
//! with its state and what each proof narrows.
//!
//! State values:
//! - `Proven`: lifted here and proven against its verified 32-bit rewrite
//!   by the differential crate `lf-melee-diff`.
//! - `Unlifted`: verified, not lifted yet; the reason is in `note`.
//! - `Unverified`: not verified by any checker run; not lifted and not
//!   written from its name.
//!
//! Method names are the class's own names with the virtual slot in
//! brackets where the method is virtual.

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
    /// The method's name in the class, with its virtual slot.
    pub name: &'static str,
    /// Where it stands.
    pub state: State,
    /// What the proof narrows, or why the method is not lifted.
    pub note: &'static str,
}

/// Every method of the family: the verified ones the lift proves or leaves
/// out, and the unverified ones the lift leaves out.
pub const METHODS: &[Method] = &[
    Method {
        name: "constructor",
        state: State::Proven,
        note: "The vtable word the constructor installs (the class's table address, relocated) is not compared: the lifted type carries no vtable, and the proof asserts the rewrite's word equals the relocated table instead. The object answers its own address; the lift answers the owned task. The base constructor and the target-node setter are host calls whose verified proof observes no write, and the proof's stubs write nothing either; the real routines run natively in the game, so any write they make outside the ten slots is not modelled. Each slot reset is a host call with the slot's offset; the proof scripts a slot-distinguishing pattern into the twelve bytes on both sides. Only the low bit of each flag word reaches the flag byte; the proof feeds words with other bits set. The high nibble of the incoming byte at 0xF1 is preserved, which the lift takes as a parameter because a constructor cannot read uninitialised storage.",
    },
    Method {
        name: "deleting destructor (vf0)",
        state: State::Proven,
        note: "The destructor is a host call that always runs; the free helper is a host call with the manager word that runs only when the low bit of the flags word is set, and the manager word is read only then. The lift returns nothing; the proof asserts the rewrite answers the object's address. The flags word arrives whole so the low-bit test stays behaviour of the lift; the proof feeds words with other bits set.",
    },
    Method {
        name: "start through the task manager (vf1)",
        state: State::Proven,
        note: "A null handle ends the start after the manager and handle calls; the proof reaches both paths. The flag byte is read before the mode calls and the mode word after them; the proof scripts a write of the mode word from the flag helper, which the verified contract does not observe, so the read order is checked. Only the low byte of the mode answer reaches the flag helper. The lift returns nothing; the proof asserts the rewrite always answers 0. The four flag bits keep the original's order (bit 1, bit 2, bit 0, bit 3).",
    },
    Method {
        name: "per-tick advance (vf20)",
        state: State::Unlifted,
        note: "Verified as fn_00cd0860.rs by a-G34 with 1200 original-completing trials and all 40 callee IDs reached. Not lifted: it has 36 scripted calls and four indirect virtual calls across several state paths, plus a scratch event buffer and float countdown; this lane did not complete an end-to-end portable call model and differential proof for it. The accepted contract fills two stack-derived output pointers with zero, skips call 3 argument 1, call 23 arguments 2 and 3, call 30 argument 0 and call 38 argument 2, and compares only the low byte of call 30 argument 4. These are proof narrowings, not lifted behavior.",
    },
    Method {
        name: "initialise and register (vf19)",
        state: State::Proven,
        note: "The two timing words come from the host; the proof feeds them from test memory through the same globals the rewrite reads, including equal and distinct pairs. The writes run before the two calls, in order. The lift returns nothing; the proof asserts the rewrite always answers 0. The kind tag and the constant 1 are passed to the registration call and compared.",
    },
    Method {
        name: "follow-up request (vf18)",
        state: State::Proven,
        note: "The reference cleanup runs first and may rewrite the flag byte; the proof scripts that write, which the verified contract does not observe, so the flag is read after the call on both sides. The sub-task query is a host call with the sub-task handle; the proof answers it through a stub behind a real virtual slot. Only the two accepted answers reach the sink, whose answer is returned. The proof runs a null sub-task only with the disable bit set: the original dereferences the sub-task unconditionally on the other path, so there is nothing to compare it against.",
    },
    Method {
        name: "set_has_target",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "is_action_sub_task_valid",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "test_flag_f0_04",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "has_target",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "make_abortable",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "load_melee_anim_groups",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "ensure_move_sub_task",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "release_melee_anim_groups",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "set_target_node",
        state: State::Unverified,
        note: "No verified rewrite. The lifted constructor calls it as a host call; it is not lifted.",
    },
    Method {
        name: "update_engage_flag",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
    Method {
        name: "update_target_node",
        state: State::Unverified,
        note: "No verified rewrite.",
    },
];
