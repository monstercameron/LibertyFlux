//! The shooting variant of the seek-cover task (class
//! `CTaskComplexSeekCoverShooting`).
//!
//! [`ShootingTask`] is a seek-cover task with one extra word: the subject
//! the shooting task works against, at the object's offset eight. Its
//! methods reach the base task's destructor, the pool, the subject, the
//! targets and the virtual slots through [`ShootingWorld`].
//!
//! Each method below is one verified rewrite, restated as a method:
//!
//! | Method | Verified rewrite |
//! |---|---|
//! | [`ShootingTask::scalar_delete`] | the scalar deleting destructor |
//! | [`ShootingTask::pursuit_action`] | the follow-up action for a target |
//! | [`ShootingTask::shot_node`] | the firing-solution node builder |
//! | [`ShootingTask::initialise_against_subject`] | the subject initialiser |
//! | [`ShootingTask::has_full_chain`] | the three-level type-tag check |
//!
//! The method for the last verified slot, the one the rewrites give the
//! largest body, is not lifted here; see the registry.

use lf_core::Handle32;

use super::task::{SeekCoverTask, TaskNode};

/// Flag byte bit of a target: set when the target wants a follow-up node.
const TARGET_NODE_BIT: u8 = 4;

/// Flag word bit of a subject: set once the subject has accepted the task.
const SUBJECT_READY_BIT: u32 = 1;

/// Flag word bit of a subject: set by the initialiser after acceptance.
const SUBJECT_PREPARED_BIT: u32 = 2;

/// The first argument of the pursuit solver's dispatch: the argument the
/// virtual slot receives.
pub const PURSUIT_DISPATCH_ARG: u32 = 0x3E8;

/// The kind word the firing node is built with.
pub const SHOT_KIND: u32 = 0xF;

/// The tail word the firing node is built with.
pub const SHOT_TAIL: u32 = 0xFFFF_FFFF;

/// The type tags of the three-level chain, in order.
pub const CHAIN_TAGS: [u32; 3] = [0x11D, 0x41A, 0x76D];

/// The mode of the pursuit node: the two flag fields the node takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PursuitNode {
    /// The argument word of the node (always zero from this method).
    pub arg: u32,
    /// The two-byte flag field (always one from this method).
    pub flag16: u16,
    /// The one-byte flag field (always one from this method).
    pub flag8: u8,
}

/// Marker for the subject object of a shooting task.
#[derive(Debug)]
pub struct ShotSubject;

/// Marker for a target object a pursuit action is taken against.
#[derive(Debug)]
pub struct PursuitTarget;

/// Marker for an argument object passed to the subject's initialiser.
#[derive(Debug)]
pub struct InitArg;

/// The words of a shooting task that the lifted methods own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ShootingTask {
    /// The subject the shooting task works against, or `None` for the
    /// null word. The original faults when a method reads through a null
    /// subject; the port returns zero instead (listed in the registry).
    pub subject: Option<Handle32<ShotSubject>>,
    /// The seek-cover words the shooting methods also read and write.
    pub base: SeekCoverTask,
}

/// The collaborators of [`ShootingTask`].
pub trait ShootingWorld {
    /// Runs the seek-cover base destructor on the object.
    fn base_destruct(&mut self, this: &mut ShootingTask) -> u32;
    /// Returns the object to the task pool.
    fn free_to_pool(&mut self, this: &mut ShootingTask);
    /// Takes a task node from the pool, or `None` when there is none.
    fn fetch_node(&mut self) -> Option<Handle32<TaskNode>>;
    /// Runs the constructor of a pursuit node.
    fn construct_node(&mut self, node: Handle32<TaskNode>);
    /// Stamps a pursuit node and stores its fields.
    fn store_pursuit_node(&mut self, node: Handle32<TaskNode>, fields: &PursuitNode);
    /// Runs the constructor of a firing node with the firing solution.
    fn construct_shot_node(&mut self, node: Handle32<TaskNode>, args: &ShotNodeArgs) -> u32;
    /// The flag byte of a target.
    fn target_flags(&mut self, target: Handle32<PursuitTarget>) -> u8;
    /// Runs the pursuit solver on a target, into scratch the caller does not read.
    fn solve(&mut self, this: &mut ShootingTask, target: Handle32<PursuitTarget>);
    /// Calls the object's virtual slot that takes one argument.
    fn dispatch_virtual(&mut self, this: &mut ShootingTask, arg: u32) -> u32;
    /// The scale factor the firing solution multiplies by, as float bits.
    fn scale_factor(&mut self) -> u32;
    /// The flag word of a subject.
    fn subject_flags(&mut self, subject: Handle32<ShotSubject>) -> u32;
    /// Stores the flag word of a subject.
    fn set_subject_flags(&mut self, subject: Handle32<ShotSubject>, flags: u32);
    /// Calls the subject's virtual slot that takes three arguments.
    fn ask_subject(
        &mut self,
        subject: Handle32<ShotSubject>,
        a0: Handle32<InitArg>,
        a1: u32,
        a2: u32,
    ) -> u32;
    /// Runs the subject initialiser's first member initialiser on the object.
    fn init_member_60(&mut self, this: &mut ShootingTask);
    /// Runs the subject initialiser's second member initialiser on the object.
    fn init_member_6c(&mut self, this: &mut ShootingTask);
    /// Finishes the subject initialiser on its argument.
    fn finish_arg(&mut self, a0: Handle32<InitArg>);
    /// The type tag of a chain object, through its virtual slot.
    fn chain_tag(&mut self, link: Handle32<ShotSubject>) -> u32;
    /// The next object of the chain, or `None` for the null word.
    fn chain_next(&mut self, link: Handle32<ShotSubject>) -> Option<Handle32<ShotSubject>>;
}

/// The arguments of the firing node's constructor, after the node: the kind,
/// the position word, the direction vector (passed by reference, so not
/// carried here), the scaled distance and the tail word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShotNodeArgs {
    /// The kind word.
    pub kind: u32,
    /// The position word of the shooting task.
    pub pos: u32,
    /// The scaled firing distance, as float bits.
    pub scaled_bits: u32,
    /// The tail word.
    pub tail: u32,
}

impl ShootingTask {
    /// The scalar deleting destructor. Runs the base destructor, then
    /// returns the object to the pool when the free flag's low bit is set.
    pub fn scalar_delete<W: ShootingWorld>(&mut self, world: &mut W, free_flag: u32) {
        world.base_destruct(self);
        if free_flag & 1 != 0 {
            world.free_to_pool(self);
        }
    }

    /// The follow-up action for a target. When the target's flag byte has
    /// its node bit set, takes a pursuit node from the pool, constructs it
    /// and returns it (zero when the pool is empty). Otherwise runs the
    /// pursuit solver and dispatches through the object's virtual slot.
    pub fn pursuit_action<W: ShootingWorld>(
        &mut self,
        world: &mut W,
        target: Handle32<PursuitTarget>,
    ) -> u32 {
        if world.target_flags(target) & TARGET_NODE_BIT != 0 {
            let Some(node) = world.fetch_node() else {
                return 0;
            };
            world.construct_node(node);
            world.store_pursuit_node(
                node,
                &PursuitNode {
                    arg: 0,
                    flag16: 1,
                    flag8: 1,
                },
            );
            return node.get();
        }
        world.solve(self, target);
        world.dispatch_virtual(self, PURSUIT_DISPATCH_ARG)
    }

    /// Builds a firing node from the firing solution: the argument scaled
    /// by the global factor, with this object's position word and direction
    /// vector. Returns the constructor's answer, or zero when the pool is
    /// empty.
    pub fn shot_node<W: ShootingWorld>(&self, world: &mut W, arg: u32) -> u32 {
        let Some(node) = world.fetch_node() else {
            return 0;
        };
        // The argument is a signed count, converted exactly; the factor is
        // the only operand that can be not-a-number, so the operand order
        // cannot change its payload.
        let scaled = (arg as i32 as f32) * f32::from_bits(world.scale_factor());
        world.construct_shot_node(
            node,
            &ShotNodeArgs {
                kind: SHOT_KIND,
                pos: self.base.pos,
                scaled_bits: scaled.to_bits(),
                tail: SHOT_TAIL,
            },
        )
    }

    /// Initialises the shooting task against its subject. Unless the
    /// subject is already ready, asks it through its virtual slot and
    /// fails (returning zero) when it declines; a subject that accepts is
    /// marked prepared. Then runs the two member initialisers, finishes on
    /// the argument and returns one.
    pub fn initialise_against_subject<W: ShootingWorld>(
        &mut self,
        world: &mut W,
        a0: Handle32<InitArg>,
        a1: u32,
        a2: u32,
    ) -> u32 {
        let Some(subject) = self.subject else {
            return 0;
        };
        if world.subject_flags(subject) & SUBJECT_READY_BIT == 0 {
            if world.ask_subject(subject, a0, a1, a2) & 0xFF == 0 {
                return 0;
            }
            let flags = world.subject_flags(subject);
            world.set_subject_flags(subject, flags | SUBJECT_PREPARED_BIT);
        }
        world.init_member_60(self);
        world.init_member_6c(self);
        world.finish_arg(a0);
        1
    }

    /// Walks three nested links through their virtual type tags, expecting
    /// the tags in order, and reports whether the base task's owned word is
    /// set. Returns one only when all three tags match and that word is
    /// set, zero otherwise.
    pub fn has_full_chain<W: ShootingWorld>(&self, world: &mut W) -> u32 {
        let Some(o1) = self.subject else {
            return 0;
        };
        if world.chain_tag(o1) != CHAIN_TAGS[0] {
            return 0;
        }
        let Some(o2) = world.chain_next(o1) else {
            return 0;
        };
        if world.chain_tag(o2) != CHAIN_TAGS[1] {
            return 0;
        }
        let Some(o3) = world.chain_next(o2) else {
            return 0;
        };
        if world.chain_tag(o3) != CHAIN_TAGS[2] {
            return 0;
        }
        u32::from(self.base.link != 0)
    }
}
