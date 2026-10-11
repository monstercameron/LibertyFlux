//! Lifted free functions of the ped task system: pose volumes and task creation.
//!
//! The original drives each pedestrian through task objects. Most of the
//! task code belongs to no named class: these are the free functions the
//! naming lanes grouped by behaviour under `ped_task_*`. This module owns
//! the data of the three simplest shapes found among them as ordinary
//! Rust (no addresses, no virtual tables, no global state, no dependence
//! on pointer width) and restates each verified 32-bit routine with
//! behaviour in it as a method on its owning type.
//!
//! - [`pose`]: the task pose volume (two local-space vectors, a tag word,
//!   an optional parent link and a flag byte) with its world-space
//!   transform, its pose blend and its point-in-volume test.
//! - [`create`]: task creation. The kind-`0x11` task builder shared by the
//!   four-word and five-word build routines (one generic method, proven
//!   once per arity), and the entry-chain cloner shared by the two clone
//!   routines (one generic method over the done-flag write, proven once
//!   per instance).
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-pedtaskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! - [`mover`]: the mover pose of the task (position, heading, packed mode,
//!   flag and level), the blend toward a target pose into a state block,
//!   the drive step that feeds the blend, and the commit back into the
//!   pose. Its unlifted callees and virtual methods travel as one trait.
//!
//! - [`seek`]: the seek-target record a seek task reads: the mover's
//!   position, a goal, and the step toward the goal in the plane.
//!
//! - [`event_table`]: the 32-record task event table the scans and
//!   replacements read and write: deadline refresh and selection, state
//!   and object lookups, flag writes, and the release and owner-object
//!   routines. Its helpers travel as one trait.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per routine in [`registry`].

#![forbid(unsafe_code)]

mod audio_release;
mod create;
mod event_table;
mod flag_update;
mod gate;
mod id_lookup;
mod mover;
mod pose;
mod seek;
mod aim_record;
mod record_scan;
mod task_block;
mod task_owner;
mod task_predicates;
mod task_slots;
mod task_state;
mod task_turn;

pub mod registry;
pub mod shape_registry;

pub use aim_record::{
    AimCallees, AimOwner, AimTask, RecordWords, normalised_direction,
};
pub use record_scan::{
    ENTRY_CONSTANTS, Anchor, FetchCallee, RecordTable, ScanEntry, ScratchSlot, TABLE_SLOTS, TAG_SLOT_FIRST, TAG_SLOT_SECOND, TAG_SLOT_THIRD, TableRecord, slot_for_tag,
};
pub use task_block::{
    BLOCK_BYTES, BlockCallees, COUNTDOWN_BASE, PARAM_WORDS, SELECTOR_NONE, SUB_HEAD_OFFSET, SUB_TAIL_OFFSET, ScaleFactors, TaskBlock, truncate_to_i32,
};
pub use task_predicates::{
    COUNT_OFFSET, ENABLED_OFFSET, FLAG_SATISFIED, FLAGS_OFFSET, KIND_MINIMUM, KIND_OFFSET,
    KIND_TYPE_MASK, PredicateTask, RESOURCE_OFFSET, RESOURCE_TAG_OFFSET, ResourceTag, TAG_EXPECTED,
    TAG_MASK,
};
pub use task_turn::{
    DIR_SCALE, FLAG_RESET, RATE_GATE, RATE_ONE, RATE_STEP, TurnParams, TurnTask, TurnTrig, Vec3,
};
pub use task_state::{
    BLOCK_WORDS, INVALID_HANDLE, STATE_RECORD_COUNT, STATE_RECORD_WORDS, StateRecord, TaskStateBlock,
};

pub use audio_release::{
    AudioObjects, AudioSlot, AudioTask, CANDIDATE_KIND_MASK, CANDIDATE_KIND_SPECIAL,
    CANDIDATE_STATE_READY, CLEANUP_ENABLED, CLEANUP_MODE_SENTINEL, Deadline, PROGRESS_LIMIT,
    PROGRESS_SCALE, PROGRESS_STEP, ReleaseGlobals, SKIP_MODE, SubtaskCallees, WINDOW_TICKS,
};
pub use create::{
    ArgLookup, BuildManagers, ChainClone, ChainCloner, ChainEntry, ChainNode, ChainOwner,
    CloneProduct, FLAG_DONE, FLAG_EXTRA, FoundEntry, KIND_11, Kind11Task, NONE, PRIORITY, PedMgr,
    PedTaskOutcome, TaskBuildCtx,
};
pub use event_table::{
    DELAY_GAP, EventCallees, EventObjects, EventRecord, EventTable, NEARBY_DISTANCE_SQUARED,
    QUEUED_KIND, RECORD_COUNT, REDUCED_THRESHOLD, STANDARD_THRESHOLD,
};
pub use flag_update::{
    COMPONENT_OFFSET, DRAW_THRESHOLD, FlagCallees, FlagOwner, RANDOM_FLAG, SKIP_SUBJECT_FLAG, SubjectFlag,
};
pub use gate::{GateCandidate, GateStatus, GateWords, admits};
pub use id_lookup::{DEFAULT_ID, NO_ID, TaskIdTable, TaskKey};
pub use mover::{BlendState, COMMIT_FLAG_ARG, MoverCallees, MoverPose, NOT_READY, SNAP_FLAG};
pub use pose::{
    AngleTuning, Blended, ConeSolvers, FLAG_ACTIVE, FLAG_COPY, FLAG_SPHERE, FLAG_TRANSFORM, Link,
    LinkMatrix, Matrix, NORM_COUNT, NormSlot, PoseFill, PoseSample, PoseVolume, Transformed,
};
pub use seek::{SEEK_MODE, STEP_SCALE, SeekGoal, SeekMover, SeekRecord, TUNING};
pub use task_owner::{OwnerCallees, TaskOwner};
pub use task_slots::{
    FIRST_BOMB_KIND, FIRST_BOMB_SLOT, ManagerCallees, PRIMARY_COUNT, SECOND_BOMB_KIND,
    SECOND_BOMB_SLOT, SECONDARY_COUNT, TaskSlots,
};
