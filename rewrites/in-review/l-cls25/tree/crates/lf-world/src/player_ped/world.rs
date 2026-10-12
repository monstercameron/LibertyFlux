//! The collaborators of the player pedestrian class.
//!
//! Every call a lifted pedestrian method makes that is not lifted itself is
//! one method here, numbered as the verified rewrite numbers its callee.
//! The virtual dispatch to a sub-object is one method too: the lift does not
//! read the sub-object's table. Global reads are methods, because a lifted
//! method owns no global state.
//!
//! A `ped` argument is the pedestrian's own state, which the collaborator
//! may change; the lifted method reads its fields again after such a call.
//! A `this` argument is the pedestrian's identity as the original passes it.

use lf_core::Handle32;

use super::ped::{CPlayerPed, ObjRef};

/// Tag: the opaque identity of a player pedestrian object.
pub struct PedTag;
/// Tag: the opaque identity of an object the pedestrian holds a pointer to
/// (a sub-object, a link target, a model entry's owner).
pub struct ObjTag;

/// Everything a lifted pedestrian method reaches outside its own fields.
pub trait PlayerPedWorld {
    /// Callee 1 of the ready-gated job pair: the ready check. Only its low
    /// byte is tested.
    fn ped_ready_check(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// Callee 2 of the ready-gated job pair: the first job, called with the
    /// job word biased by a fixed amount, then the job word itself.
    fn ped_job_a(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32;

    /// Callee 3 of the ready-gated job pair: the second job, with the same
    /// argument shape as [`PlayerPedWorld::ped_job_a`].
    fn ped_job_b(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        biased: u32,
        job: u32,
    ) -> u32;

    /// Callee 2 of the linked notification: the trigger entry, called with
    /// the three constant words 0, 1 and 0 the original passes.
    fn trigger_entry(
        &mut self,
        ped: &mut CPlayerPed,
        this: Handle32<PedTag>,
        first: u32,
        second: u32,
        third: u32,
    ) -> u32;

    /// Callee 3 of the linked notification: the base update the unlinked
    /// case tail-forwards to.
    fn base_update(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// Callee 1 of the motion value addition: the helper that receives the
    /// new motion value as its bit pattern.
    fn motion_helper(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, bits: u32) -> u32;

    /// Callee 1 of the reset-and-apply step: the pre-pass, run only for the
    /// listed mode nibbles when the argument's bit 0x10 is clear.
    fn pre_step(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32;

    /// Callee 2 of the reset-and-apply step: the apply pass.
    fn apply_step(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, arg: u32) -> u32;

    /// Callee 3 of the reset-and-apply step: the notify entry of the first
    /// sub-object, called with the object as its `this` (zero when absent)
    /// and the two constant words 8 and 0.
    fn notify_object(
        &mut self,
        ped: &mut CPlayerPed,
        obj: ObjRef,
        first: u32,
        second: u32,
    ) -> u32;

    /// Virtual slot 8 of a sub-object, called with the argument. The lift
    /// does not read the sub-object's table: a host dispatches here.
    fn dispatch_slot8(&mut self, ped: &mut CPlayerPed, obj: Handle32<ObjTag>, arg: u32) -> u32;

    /// The player context, read from a shared global. Raw bits: zero is a
    /// value the original passes on.
    fn player_context(&mut self) -> u32;

    /// Callee 1 of the setup-and-store step: asks the context for a lookup
    /// object. Raw bits, zero included.
    fn lookup_context(&mut self, context: u32) -> u32;

    /// Callee 2 of the setup-and-store step: the setup helper, called on the
    /// lookup object with the pedestrian as its argument, the constant zero
    /// the original passes, then the three words and the tail word.
    #[allow(clippy::too_many_arguments)]
    fn setup(
        &mut self,
        ped: &mut CPlayerPed,
        lookup: u32,
        this: Handle32<PedTag>,
        flag: u32,
        first: u32,
        second: u32,
        third: u32,
        tail: u32,
    ) -> u32;

    /// Callee 3 of the setup-and-store step: the follow-up helper, called on
    /// the stored result with the final argument.
    fn finalize(&mut self, ped: &mut CPlayerPed, target: u32, arg: u32) -> u32;

    /// The state byte of a model entry, read from the shared model table
    /// and the entry's fixed offset. The lift does not read the table.
    fn model_state(&mut self, model_id: u32) -> u8;

    /// Callee 2 of the relink: the base entry, called with the flag word.
    fn base_entry(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, flag: u32) -> u32;

    /// One word of a record the pedestrian points at (a point's component),
    /// read from memory. Only the lifted method's reads use it.
    fn read_word(&mut self, address: u32) -> u32;

    /// The squared-distance threshold of the relink, read from a shared
    /// global, as the bits of a float.
    fn distance_threshold_bits(&mut self) -> u32;

    /// Callee 3 of the relink: the relink entry, called with the pedestrian
    /// and the constant one.
    fn relink(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, one: u32) -> u32;

    /// Callee 1 of the aim acquisition: the targeting component's context
    /// lookup, called with the component's identity. Zero when there is no
    /// context.
    fn get_context(&mut self, targeting: u32) -> u32;

    /// The property argument the context holds (a word of the context).
    fn context_property_argument(&mut self, context: u32) -> u32;

    /// Callee 2 of the aim acquisition: the property lookup for an argument.
    fn get_property(&mut self, argument: u32) -> u32;

    /// The threshold of a property, as the bits of a single-precision float.
    fn property_threshold_bits(&mut self, property: u32) -> u32;

    /// The aim state word, read from a shared global.
    fn aim_state(&mut self) -> u32;

    /// Callee 3 of the aim acquisition: the state gate, which takes no
    /// argument. Only its low byte is tested.
    fn state_gate(&mut self) -> u32;

    /// The aim factor the aim context holds, as the bits of a float.
    fn aim_factor_bits(&mut self, context: u32) -> u32;

    /// Callee 4 of the aim acquisition: creates the target with the helper
    /// code, the threshold bits, the property key and two constant zero
    /// words the original passes after it. Zero when no target is made.
    #[allow(clippy::too_many_arguments)]
    fn create_target(
        &mut self,
        component: u32,
        helper: u32,
        threshold: u32,
        key: u32,
        zero_first: u32,
        zero_second: u32,
    ) -> u32;

    /// Callee 5 of the aim acquisition: attaches the target to the targeting
    /// component. The answer is not used.
    fn attach_target(&mut self, component: u32, target: u32);

    /// Writes the selected marker byte of the aim context.
    fn mark_selected(&mut self, context: u32);

    /// Callee 2 of the entry chain: the reset entry of the sub-object at
    /// `sub`, an address the chain computed (raw, since the chain works on
    /// the address and never dereferences it itself).
    fn chain_reset(&mut self, sub: u32);

    /// Callee 3 of the entry chain: the flag entry of the sub-object, called
    /// with the flag word the chain passes (one).
    fn chain_flag(&mut self, sub: u32, flag: u32);

    /// Callee 4 of the entry chain: the main entry of the sub-object; its
    /// answer is the chain's answer.
    fn chain_main(&mut self, sub: u32) -> u32;

    /// Callee 1 of the model change: the model lookup, called with the
    /// pedestrian and the model number.
    fn lookup_model(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, model_id: u32) -> u32;

    /// Callee 2 of the model change: the preload of the model, called with
    /// the model number alone.
    fn preload_model(&mut self, model_id: u32) -> u32;

    /// Callee 3 of the model change: the revive, called with the pedestrian
    /// and a one-word flag saying whether the current slot is taken.
    fn revive(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, flag: u32) -> u32;

    /// Callee 4 of the model change: loads the bundle named by the job word.
    fn load_bundle(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>, bundle: u32) -> u32;
}

/// A word-sized shared global the gate block reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordGlobal {
    /// The state word: the gate refuses when it reads one.
    State,
    /// The first of the pair words, compared with the second.
    PairA,
    /// The second of the pair words.
    PairB,
    /// The mode word: the gate refuses when it reads its refusal value.
    Mode,
    /// The float constant the gate compares the float word with, as bits.
    FloatConst,
}

/// A byte-sized shared global the gate block reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteGlobal {
    /// The first flag byte.
    FlagA,
    /// The second flag byte.
    FlagB,
    /// The first of the two or-bytes.
    OrFirst,
    /// The second of the two or-bytes.
    OrSecond,
    /// The guard byte.
    Guard,
}

/// The collaborators of the gate block: the hook through a global slot, the
/// pose table, the virtual slots of the pedestrian, the inner and sub-objects,
/// and the probes and deep refresh.
pub trait GateWorld: PlayerPedWorld {
    /// The hook's argument, read from a shared global.
    fn hook_argument(&mut self) -> u32;

    /// The hook: the stdcall target a shared global slot holds. Answers the
    /// gate's first test.
    fn hook_call(&mut self, argument: u32) -> u32;

    /// A shared word global the gate reads.
    fn global_word(&mut self, which: WordGlobal) -> u32;

    /// A shared byte global the gate reads.
    fn global_byte(&mut self, which: ByteGlobal) -> u8;

    /// The pose table entry at a signed index.
    fn pose_entry(&mut self, index: i32) -> u32;

    /// The flag byte of a pose entry.
    fn entry_flag(&mut self, entry: u32) -> u8;

    /// The pedestrian's refresh slot (its table's slot, with the pedestrian
    /// and the pedestrian as arguments). Answers the object refreshed.
    fn refresh_virtual(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// Callee 3 of the gate: the follow-up on the object the refresh answered.
    fn refresh_follow(&mut self, refreshed: u32);

    /// The attach slot of the inner object's table, with the pedestrian as
    /// its argument.
    fn attach_virtual(&mut self, inner: u32, this: Handle32<PedTag>);

    /// Callee 5 of the gate: the first probe on the chain base.
    fn probe_first(&mut self, obj: u32) -> u32;

    /// Callee 6 of the gate: the second probe on the chain base.
    fn probe_second(&mut self, obj: u32) -> u32;

    /// Callee 7 of the gate: the deep refresh's gate, which takes no argument.
    fn deep_refresh_gate(&mut self) -> u32;

    /// Callee 8 of the gate: the deep refresh of the sub-object.
    fn deep_refresh_entry(&mut self, sub: u32);

    /// The sub-object's slot 4, called with the sub-object.
    fn sub_virtual(&mut self, sub: u32);

    /// Callee 10 of the gate: the readiness probe on the pedestrian.
    fn ready_probe(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// The pedestrian's tail slot, called with the pedestrian.
    fn tail_virtual(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>);
}

/// The collaborators of the task-budget step: the base step, the item lookup,
/// the task classifier, the budget task query, the award, the frame cost
/// globals and the heat limit, and one byte read from the item.
pub trait TaskWorld: PlayerPedWorld {
    /// Callee 0 of the budget step: the base class's step, which may change
    /// the pedestrian.
    fn base_step(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// Callee 1 of the budget step: the pedestrian's current item. Zero when
    /// there is none.
    fn current_item(&mut self, ped: &mut CPlayerPed, this: Handle32<PedTag>) -> u32;

    /// Callee 2 of the budget step: classifies the task record. Returns the
    /// answer word and the output word the callee writes; the tag byte the
    /// callee also writes is not read by the step.
    fn classify_task(&mut self, ped: &mut CPlayerPed, task: u32) -> (u32, u32);

    /// Callee 3 of the budget step: whether a task of the given kind is in the
    /// list (called with the list, the kind and the constant zero).
    fn task_present(&mut self, ped: &mut CPlayerPed, list: u32, task: u32, zero: u32) -> u32;

    /// Callee 4 of the budget step: the award added to the budget when no
    /// task is present and the hold is clear.
    fn award(&mut self, ped: &mut CPlayerPed) -> u32;

    /// The frame delta time, as the bits of a float, read from a shared
    /// global.
    fn frame_delta_bits(&mut self) -> u32;

    /// The cost scale constant, as the bits of a float, read from a shared
    /// global.
    fn cost_scale_bits(&mut self) -> u32;

    /// The heat limit byte, read from a shared global.
    fn heat_limit(&mut self) -> u8;

    /// One byte of the item at an address (a read only).
    fn read_byte(&mut self, address: u32) -> u8;
}
