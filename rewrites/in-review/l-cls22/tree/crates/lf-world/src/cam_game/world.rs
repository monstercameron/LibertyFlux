//! The collaborators of the game camera class.
//!
//! Every call a lifted camera method makes that is not lifted itself is one
//! method here. Numbered callees keep the order the verified rewrites call
//! them in; the virtual calls are the object's own dispatched slot and the
//! worker's release slot. Global reads and writes are methods too, because a
//! lifted method owns no global state. Reads of objects the camera does not
//! own (gate status objects, the tail object) are methods on the world,
//! reached through an opaque handle.

use lf_core::Handle32;

use super::game::CCamGame;

/// Tag: the opaque identity of a game camera object.
pub struct CamTag;
/// Tag: the opaque identity of the owner a camera is attached to.
pub struct CamOwnerTag;
/// Tag: the opaque identity of a camera worker object.
pub struct WorkerTag;
/// Tag: the opaque identity of the object pool manager.
pub struct PoolTag;
/// Tag: the opaque identity of a gate status object (read, never written).
pub struct GateTag;
/// Tag: the opaque identity of a camera target found by a gate check.
pub struct TargetTag;
/// Tag: the opaque identity of the tail object (read and written).
pub struct TailTag;
/// Tag: the opaque identity of the global singleton object the blend step
/// reads.
pub struct SingletonTag;

/// A global the camera's blend step reads or writes. Values cross the
/// boundary as raw bits: floats as their bit patterns, bytes zero-extended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CamGlobal {
    /// Upper bound of the blend and of the first accumulator (a float).
    Ceiling,
    /// First inhibit byte.
    InhibitA,
    /// Second inhibit byte.
    InhibitB,
    /// Step added to the first blend channel (a float).
    Step1,
    /// Step added to the first accumulator (a float).
    Accumulator1Step,
    /// Step added to the second blend channel (a float).
    Step2,
    /// Step added to the second accumulator (a float).
    Accumulator2Step,
    /// Floor the blend channels are raised to (a float).
    Cap,
    /// First accumulator (a float).
    Accumulator1,
    /// Packed counter advanced when the first accumulator is zero.
    Count,
    /// Done flag of the first channel (a byte).
    Done1,
    /// Second accumulator (a float).
    Accumulator2,
    /// Done flag of the second channel (a byte).
    Done2,
    /// Fallback byte checked when the singleton is inactive.
    Fallback,
}

/// A field of the singleton object the blend step reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingletonField {
    /// The active byte: nonzero when the singleton is active.
    ActiveByte,
    /// The mode word: the second channel runs only when it reads exactly 1.
    ModeWord,
}

/// Which of the two gate blocks of the camera's event step runs. The two
/// blocks have the same shape; they differ in the callees they call and in
/// the global word they pass on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateBlock {
    /// The first block: callees 1 to 3 and the first global word.
    First,
    /// The second block: callees 13 to 15 and the second global word.
    Second,
}

/// Everything a game camera method reaches outside its own fields.
///
/// A `this` argument is the camera's identity as the original passes it.
/// A `cam` argument is the camera's own state, which the callee may change;
/// the lifted method reads its fields again after such a call.
pub trait CamGameWorld {
    /// Callee 1 of the deleting destructor: the class's destructor body.
    /// Not lifted yet, so it is a call on the world.
    fn destroy_body(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// The pool manager, read from a shared global after the destructor
    /// body has run. `None` stands for the original's zero.
    fn pool_manager(&mut self) -> Option<Handle32<PoolTag>>;

    /// Callee 2 of the deleting destructor: returns the camera's storage to
    /// the pool held by `manager`.
    fn pool_release(&mut self, manager: Option<Handle32<PoolTag>>, this: Handle32<CamTag>);

    /// The object's own dispatched teardown step (its table's slot 6).
    /// The lift does not read the table: a host whose camera class is
    /// overridden in a derived class dispatches here.
    fn teardown(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 2 of the restart: builds a worker for `owner`. The original
    /// passes two constant words (mode 1, extra 0) between the owner and
    /// the camera; they are fixed by the restart and are passed in the
    /// same order here.
    fn create_worker(
        &mut self,
        owner: Option<Handle32<CamOwnerTag>>,
        mode: u32,
        extra: u32,
        this: Handle32<CamTag>,
    ) -> Option<Handle32<WorkerTag>>;

    /// Callee 3 of the restart: chains the worker's flags.
    fn chain_set_flags(&mut self, worker: Option<Handle32<WorkerTag>>);

    /// The worker's own release step (its table's slot 1). The restart
    /// never reads the worker's table itself.
    fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>);

    /// Callee 1 of the idle reset: the shared reset step, which may change
    /// any field of the camera.
    fn reset_state_block(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// The shared default word, read from a global and copied into the
    /// camera by the idle reset.
    fn shared_default_word(&mut self) -> u32;

    /// The inhibit byte, read from a global before the gate blocks run.
    fn inhibit_flag(&mut self) -> u8;

    /// Callee 1 (for the first block) or callee 13 (for the second): asks
    /// the provider for a gate status object. `None` for zero.
    fn gate_status(&mut self, block: GateBlock) -> Option<Handle32<GateTag>>;

    /// One byte of a gate status object, at `offset` within it. A read only.
    fn gate_byte(&mut self, gate: Handle32<GateTag>, offset: u32) -> u8;

    /// Callee 2 (first block) or callee 14 (second): asks the watcher for a
    /// target of the given type, with the camera as the subject.
    fn find_target(
        &mut self,
        cam: &mut CCamGame,
        this: Handle32<CamTag>,
        block: GateBlock,
    ) -> Option<Handle32<TargetTag>>;

    /// The global word a gate block passes on: the first word for the first
    /// block, the second for the second.
    fn gate_word(&mut self, block: GateBlock) -> u16;

    /// Callee 3 (first block) or callee 15 (second): notifies the sink with
    /// the camera's sub-object, the global word and the two constant
    /// flags, which the original passes as one. The sub-object is the
    /// camera's own address plus a fixed offset, computed by the host.
    fn notify_sink(
        &mut self,
        cam: &mut CCamGame,
        this: Handle32<CamTag>,
        word: u16,
        block: GateBlock,
    );

    /// Callee 4: the camera's step that syncs its target.
    fn sync_target(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 5: whether the global singleton exists.
    fn singleton_present(&mut self) -> bool;

    /// Callees 6 to 12, one per `step`: the camera's middle update chain.
    fn middle_update(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>, step: u32);

    /// Callee 16: the watcher's answer for the tail, a second selector.
    fn tail(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) -> Option<Handle32<TailTag>>;

    /// The state byte of the tail object (offset 0x22E).
    fn tail_state(&mut self, tail: Handle32<TailTag>) -> u8;

    /// The flag byte of the tail object (offset 0x18C).
    fn tail_flags(&mut self, tail: Handle32<TailTag>) -> u8;

    /// Writes the flag byte of the tail object.
    fn set_tail_flags(&mut self, tail: Handle32<TailTag>, flags: u8);

    /// Writes the state byte of the tail object.
    fn set_tail_state(&mut self, tail: Handle32<TailTag>, state: u8);

    /// Writes the matched byte of the global the tail step records into.
    fn set_matched(&mut self, matched: u8);

    /// Clears the global done flag at the end of the step.
    fn clear_done(&mut self);

    /// Callees 1 to 4 of the blend update: the camera's four update steps,
    /// run in order before the initialised marker is looked at.
    fn update_1(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);
    /// See [`CamGameWorld::update_1`].
    fn update_2(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);
    /// See [`CamGameWorld::update_1`].
    fn update_3(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);
    /// See [`CamGameWorld::update_1`].
    fn update_4(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 5 of the blend update: notifies a watcher when the camera was
    /// already initialised.
    fn watch_init(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 6 of the blend update: the global singleton, or `None` when it
    /// does not exist. Its fields are read through [`CamGameWorld::singleton_field`].
    fn singleton(&mut self) -> Option<Handle32<SingletonTag>>;

    /// One field of the singleton object.
    fn singleton_field(&mut self, singleton: Handle32<SingletonTag>, field: SingletonField) -> u32;

    /// Callee 7 of the blend update: the first gate answer. Only its low
    /// byte is tested.
    fn blend_gate_one(&mut self) -> u32;

    /// Callee 8 of the blend update: the second gate answer. Only its low
    /// byte is tested.
    fn blend_gate_two(&mut self) -> u32;

    /// Reads a global the blend step uses, as raw bits.
    fn read_global(&mut self, global: CamGlobal) -> u32;

    /// Writes a global the blend step sets, as raw bits. A byte global takes
    /// the low byte of the value.
    fn write_global(&mut self, global: CamGlobal, bits: u32);

    /// Callee 9 of the blend update: the camera's update step after the
    /// singleton's blend channels.
    fn update_9(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 10 of the blend update: the camera's update step after the
    /// singleton block.
    fn update_10(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);

    /// Callee 11 of the blend update: the camera's last update step.
    fn update_11(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>);
}
