//! The seek-cover composite task: the class that walks a ped to a cover
//! point and then holds its position there.
//!
//! [`SeekCoverTask`] owns the words of the object that the verified
//! constructors, the destructor and the subtask router read and write:
//! the position vector and its scalars, the mode and flag words, the
//! cover-slot words, the tail words and the owned-object word. The base
//! task's storage and the two embedded sub-objects are not owned here:
//! the callees that initialise and release them act on the object through
//! [`SeekCoverWorld`], which the caller supplies. Every callee receives
//! the object, so an ordering that the callee could observe (a callee
//! reading a flag word the constructor has not written yet) stays exactly
//! as the verified rewrites order it.
//!
//! Each method below is one verified rewrite, restated as a method:
//!
//! | Method | Verified rewrite |
//! |---|---|
//! | [`SeekCoverTask::construct_full`] | the full constructor |
//! | [`SeekCoverTask::construct_cover_row`] | the constructor that takes a cover-table row |
//! | [`SeekCoverTask::construct_short`] | the shorter constructor variant |
//! | [`SeekCoverTask::destruct`] | the non-deleting destructor |
//! | [`SeekCoverTask::route_to_sub_task`] | the subtask router |
//! | [`SeekCoverTask::allocate_node_for_arg`] | the task-node allocator |
//!
//! The collaborators are opaque: the pool that hands out workers and
//! nodes, the subtasks, the cover table, and the sub-objects are reached
//! through [`SeekCoverWorld`]. No addresses, numbered slots or global
//! state appear in this module.

use lf_core::Handle32;

/// Bit three of the flag byte: the synchronised bit of a subtask latch.
pub const LATCH_SYNC: u8 = 8;

/// Low three bits of the flag word: the subtask route.
const ROUTE_MASK: u32 = 7;

/// Bits three to five of the flag word hold the mode, stored as a signed
/// lane index by the router.
const MODE_SHIFT: u32 = 3;

/// Bits of the flag word that the full constructor keeps: bit zero and
/// bits six and up, the mode bits and bit one and two being rewritten.
const FULL_KEPT_FLAG_BITS: u32 = 0xFFFF_FFC1;

/// Bits of the flag word that the second constructor keeps: all bits
/// except bit zero and the mode bits three to five and bit one.
const ROW_KEPT_FLAG_BITS: u32 = 0xFFFF_FFC2;

/// Bit one of the flag word, set by the second constructor.
const ROW_FLAG_SET: u32 = 2;

/// Bits of the flag word the empty-row path keeps: the low three bits.
const EMPTY_KEPT_FLAG_BITS: u32 = 0xFFFF_FFF8;

/// Bits of the flag word that the short constructor keeps: bits six and up.
const SHORT_KEPT_FLAG_BITS: u32 = 0xFFFF_FFC0;

/// Bit zero of the flag word, set by the full constructor.
const FULL_FLAG_SET: u32 = 1;

/// The "no index" sentinel stored by the constructors.
const NO_INDEX: u32 = 0xFFFF_FFFF;

/// The third tail word, the mark, that every constructor sets to the
/// sentinel.
const NO_MARK: u32 = 0xFFFF_FFFF;

/// Which stamp the object carries, written first by the constructors and
/// the destructor. The stamp is the object's class identity; the port
/// keeps it in the collaborator, not in the owned fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stamp {
    /// The seek-cover class stamp.
    SeekCover,
}

/// The owned-object word that the destructor releases: the position
/// word, or the word at the far end of the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSlot {
    /// The word that the constructors store the position in.
    Pos,
    /// The owned word the constructors never store.
    Link,
}

/// The subtask route, from the low three bits of the flag word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverRoute {
    /// Route zero: the subtask takes the speed and the lane only.
    Zero,
    /// Route one: the subtask also takes the address of the cover slot.
    One,
    /// Route two: the subtask also takes the index word.
    Two,
}

impl CoverRoute {
    /// The route for a flag word, or `None` for the three routes the
    /// original does not run (it faults reading the result).
    #[must_use]
    pub const fn from_flags(flags: u32) -> Option<Self> {
        match flags & ROUTE_MASK {
            0 => Some(Self::Zero),
            1 => Some(Self::One),
            2 => Some(Self::Two),
            _ => None,
        }
    }
}

/// The fourth argument a route passes, after the position and the mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteExtra {
    /// Route two: the index word of the task.
    Index(u32),
    /// Route one: the cover-slot words of the task, by reference.
    SlotRef,
}

/// The arguments a subtask route passes to its subtask, in call order
/// after the worker. The direction vector is always passed by reference
/// and is not carried here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteArgs {
    /// The position word of the task.
    pub pos: u32,
    /// The mode byte of the task.
    pub mode: u8,
    /// Bit zero of the latch byte of the task.
    pub latch_bit: u8,
    /// The route's fourth argument, absent on route zero.
    pub extra: Option<RouteExtra>,
    /// The speed, as the bits of the float word of the task.
    pub speed: u32,
    /// The lane: the signed mode bits, widened to a full word.
    pub lane: u32,
}

/// The fields of a small task node that the allocator fills in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmallNode {
    /// The argument word of the node.
    pub arg: u32,
    /// The flag byte of the node.
    pub flag: u8,
    /// The tail word of the node.
    pub tail: u32,
}

/// Marker for a seek-cover worker object taken from the task pool.
#[derive(Debug)]
pub struct CoverWorker;

/// Marker for a subtask object that a route returns.
#[derive(Debug)]
pub struct CoverSubTask;

/// Marker for a task node taken from the task pool.
#[derive(Debug)]
pub struct TaskNode;

/// The words of the seek-cover object that the lifted methods own.
///
/// Field names follow what the verified rewrites do with them. Offsets
/// from the start of the object are documented per field; they are for
/// the reader and the differential proof, and the lifted code never uses
/// them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SeekCoverTask {
    /// The direction vector, three words, copied in by the constructors.
    pub dir: [u32; 3],
    /// The position word; the destructor releases it when it is not zero.
    pub pos: u32,
    /// The mode byte, the low byte of the constructor's first flag word.
    pub mode_byte: u8,
    /// The index word: the cover-table row, or the no-index sentinel.
    pub index: u32,
    /// The four cover-slot words the router and the second constructor read.
    pub slot: [u32; 4],
    /// The flag word: the route in the low three bits, the mode in bits
    /// three to five, the set bit in bit zero or one.
    pub flags: u32,
    /// The latch byte; bit zero is set by the constructors, bit three is
    /// synchronised with the subtask by the router.
    pub latch: u8,
    /// The speed word: the float bits copied in by the constructors.
    pub weight: u32,
    /// The three tail words the constructors zero.
    pub tail: [u32; 3],
    /// The owned word the destructor releases when it is not zero.
    pub link: u32,
    /// The mark word, set to the sentinel by every constructor.
    pub mark: u32,
    /// The two last tail words, zeroed by every constructor.
    pub tail_end: [u32; 2],
}

/// The collaborators of [`SeekCoverTask`]: the callees and virtual slots
/// the verified rewrites call, one method per callee, and the cover table
/// and the task pool as queries.
///
/// The object is passed to every method that the original calls with the
/// object as its `this`, so a callee sees exactly the fields the original
/// has written by then.
pub trait SeekCoverWorld {
    /// Writes the class stamp into the object's first word.
    fn set_stamp(&mut self, this: &mut SeekCoverTask, stamp: Stamp);
    /// Runs the base task's constructor on the object.
    fn base_construct(&mut self, this: &mut SeekCoverTask);
    /// Initialises the embedded sub-object at the first member offset.
    fn init_member_60(&mut self, this: &mut SeekCoverTask);
    /// Initialises the embedded sub-object at the second member offset.
    fn init_member_6c(&mut self, this: &mut SeekCoverTask);
    /// The shared re-initialiser that finishes every constructor.
    fn reinit_shared(&mut self, this: &mut SeekCoverTask);
    /// The flag byte of the cover-table row for an index.
    fn cover_row_flags(&mut self, index: u32) -> u8;
    /// Queries the cover-table row for an index: four words.
    fn cover_row_query(&mut self, this: &mut SeekCoverTask, index: u32) -> [u32; 4];
    /// Releases one owned word, given the word's value.
    fn release_owned(&mut self, this: &mut SeekCoverTask, slot: OwnedSlot, owned: u32);
    /// Releases the embedded sub-object at the second member offset.
    fn release_member_6c(&mut self, this: &mut SeekCoverTask);
    /// Releases the embedded sub-object at the first member offset.
    fn release_member_60(&mut self, this: &mut SeekCoverTask);
    /// Runs the base task's destructor, returning its answer.
    fn base_destruct(&mut self, this: &mut SeekCoverTask) -> u32;
    /// Takes a worker from the task pool, or `None` when there is none.
    fn fetch_worker(&mut self, this: &mut SeekCoverTask) -> Option<Handle32<CoverWorker>>;
    /// Runs a route on a worker, returning the subtask it produces.
    fn run_route(
        &mut self,
        this: &mut SeekCoverTask,
        route: CoverRoute,
        worker: Handle32<CoverWorker>,
        args: &RouteArgs,
    ) -> Option<Handle32<CoverSubTask>>;
    /// The latch byte of a subtask.
    fn latch_of(&mut self, sub: Handle32<CoverSubTask>) -> u8;
    /// Stores the latch byte of a subtask.
    fn set_latch_of(&mut self, sub: Handle32<CoverSubTask>, latch: u8);
    /// Takes a task node from the task pool, or `None` when there is none.
    fn allocate_node(&mut self) -> Option<Handle32<TaskNode>>;
    /// Runs the base constructor of a task node.
    fn construct_node(&mut self, node: Handle32<TaskNode>);
    /// Stamps a task node as a small node and stores its fields.
    fn store_small_node(&mut self, node: Handle32<TaskNode>, fields: &SmallNode);
}

/// Inputs of the full constructor, in the order the original takes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullArgs<'a> {
    /// The position word, stored in the object's position word.
    pub pos: u32,
    /// The first vector, copied into the direction vector.
    pub dir_src: &'a [u32; 3],
    /// The byte stored as the mode byte.
    pub mode_byte: u8,
    /// The byte whose bit zero is folded into the latch byte.
    pub latch_src: u8,
    /// The second vector, copied into the first three cover-slot words.
    pub slot_src: &'a [u32; 3],
    /// The speed word, the float bits copied in.
    pub weight: u32,
    /// The mode word; its low three bits go to the flag word.
    pub mode: u32,
}

/// Inputs of the second constructor, in the order the original takes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoverRowArgs<'a> {
    /// The position word.
    pub pos: u32,
    /// The direction vector, copied in.
    pub dir_src: &'a [u32; 3],
    /// The first flag word; its low byte is the mode byte.
    pub flags0: u32,
    /// The second flag word; its bit zero is folded into the latch byte.
    pub flags1: u32,
    /// The cover-table row index.
    pub index: u32,
    /// The speed word, the float bits copied in.
    pub weight: u32,
    /// The mode word; its low three bits go to the flag word.
    pub mode: u32,
}

/// Inputs of the shorter constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShortArgs<'a> {
    /// The position word.
    pub pos: u32,
    /// The direction vector, copied in.
    pub dir_src: &'a [u32; 3],
    /// The byte stored as the mode byte.
    pub mode_byte: u8,
    /// The byte whose bit zero is folded into the latch byte.
    pub latch_src: u8,
    /// The speed word, the float bits copied in.
    pub weight: u32,
    /// The mode word; its low three bits go to the flag word.
    pub mode: u32,
}

impl SeekCoverTask {
    /// Reads the lane: the three mode bits of the flag word, sign
    /// extended to a full word.
    #[must_use]
    pub const fn lane(flags: u32) -> u32 {
        (((flags << 26) as i32) >> 29) as u32
    }

    /// The full constructor. Runs the base constructor, stamps the class,
    /// copies the vectors and scalars in, runs the two sub-object
    /// initialisers, folds the mode into the flag word, folds bit zero of
    /// the second byte into the latch, zeroes the tail and finishes
    /// through the shared re-initialiser.
    pub fn construct_full<W: SeekCoverWorld>(&mut self, world: &mut W, args: &FullArgs<'_>) {
        world.base_construct(self);
        world.set_stamp(self, Stamp::SeekCover);
        self.dir = *args.dir_src;
        self.pos = args.pos;
        self.mode_byte = args.mode_byte;
        world.init_member_60(self);
        world.init_member_6c(self);
        self.index = NO_INDEX;
        self.slot[0] = args.slot_src[0];
        self.slot[1] = args.slot_src[1];
        self.slot[2] = args.slot_src[2];
        self.flags = ((args.mode & 7) << MODE_SHIFT) | (self.flags & FULL_KEPT_FLAG_BITS) | FULL_FLAG_SET;
        self.latch = fold_latch_bit(self.latch, args.latch_src);
        self.weight = args.weight;
        self.tail = [0, 0, 0];
        self.mark = NO_MARK;
        self.tail_end = [0, 0];
        world.reinit_shared(self);
    }

    /// The constructor that takes a cover-table row. Runs the base
    /// constructor and the sub-object initialisers, copies the vector and
    /// scalars, and then either takes the row's query answer (when the
    /// row's flag byte has a low bit set) or clears the slot and the
    /// index. Finishes through the shared re-initialiser either way.
    pub fn construct_cover_row<W: SeekCoverWorld>(&mut self, world: &mut W, args: &CoverRowArgs<'_>) {
        world.base_construct(self);
        world.set_stamp(self, Stamp::SeekCover);
        self.dir = *args.dir_src;
        self.pos = args.pos;
        self.mode_byte = args.flags0 as u8;
        world.init_member_60(self);
        world.init_member_6c(self);
        self.index = args.index;
        self.slot[0] = 0;
        self.slot[1] = 0;
        self.slot[2] = 0;
        self.flags = (self.flags & ROW_KEPT_FLAG_BITS) | ((args.mode & 7) << MODE_SHIFT);
        self.latch = fold_latch_bit(self.latch, args.flags1 as u8);
        self.flags |= ROW_FLAG_SET;
        self.weight = args.weight;
        self.tail = [0, 0, 0];
        self.mark = NO_MARK;
        self.tail_end = [0, 0];

        let row_flags = world.cover_row_flags(args.index);
        if row_flags & 7 != 0 {
            let answer = world.cover_row_query(self, args.index);
            self.slot = answer;
            world.reinit_shared(self);
        } else {
            self.slot[2] = 0;
            self.slot[1] = 0;
            self.slot[0] = 0;
            self.flags &= EMPTY_KEPT_FLAG_BITS;
            self.index = NO_INDEX;
            world.reinit_shared(self);
        }
    }

    /// The shorter constructor. Like the full constructor, but the second
    /// vector is zeroed rather than copied, the flag word keeps only its
    /// bits six and up, and the mode is folded in without a set bit.
    pub fn construct_short<W: SeekCoverWorld>(&mut self, world: &mut W, args: &ShortArgs<'_>) {
        world.base_construct(self);
        world.set_stamp(self, Stamp::SeekCover);
        self.dir = *args.dir_src;
        self.pos = args.pos;
        self.mode_byte = args.mode_byte;
        world.init_member_60(self);
        world.init_member_6c(self);
        self.index = NO_INDEX;
        self.slot[0] = 0;
        self.slot[1] = 0;
        self.slot[2] = 0;
        self.flags = ((args.mode & 7) << MODE_SHIFT) | (self.flags & SHORT_KEPT_FLAG_BITS);
        self.latch = fold_latch_bit(self.latch, args.latch_src);
        self.weight = args.weight;
        self.tail = [0, 0, 0];
        self.mark = NO_MARK;
        self.tail_end = [0, 0];
        world.reinit_shared(self);
    }

    /// The non-deleting destructor. Stamps the class, releases the
    /// position word and the owned word when each is set (clearing it),
    /// releases the two embedded sub-objects, then runs the base
    /// destructor and returns its answer.
    pub fn destruct<W: SeekCoverWorld>(&mut self, world: &mut W) -> u32 {
        world.set_stamp(self, Stamp::SeekCover);
        let owned_pos = self.pos;
        if owned_pos != 0 {
            world.release_owned(self, OwnedSlot::Pos, owned_pos);
            self.pos = 0;
        }
        let owned_link = self.link;
        if owned_link != 0 {
            world.release_owned(self, OwnedSlot::Link, owned_link);
            self.link = 0;
        }
        world.release_member_6c(self);
        world.release_member_60(self);
        world.base_destruct(self)
    }

    /// The subtask router. Picks a route from the low three bits of the
    /// flag word, takes a worker from the pool, runs the route on it, and
    /// synchronises bit three of the latch byte with the subtask's. Returns
    /// the subtask, or `None` when there is none to return.
    ///
    /// The original faults on a route it does not run and on a missing
    /// worker or subtask (it reads the subtask's latch byte at address
    /// zero plus its offset). This port returns `None` at those points
    /// instead of reading. That difference is outside every proof and is
    /// listed in the registry.
    pub fn route_to_sub_task<W: SeekCoverWorld>(&mut self, world: &mut W) -> Option<Handle32<CoverSubTask>> {
        let route = CoverRoute::from_flags(self.flags)?;
        let worker = world.fetch_worker(self)?;
        let extra = match route {
            CoverRoute::Zero => None,
            CoverRoute::One => Some(RouteExtra::SlotRef),
            CoverRoute::Two => Some(RouteExtra::Index(self.index)),
        };
        let args = RouteArgs {
            pos: self.pos,
            mode: self.mode_byte,
            latch_bit: self.latch & 1,
            extra,
            speed: self.weight,
            lane: Self::lane(self.flags),
        };
        let sub = world.run_route(self, route, worker, &args)?;
        let mine = self.latch;
        let theirs = world.latch_of(sub);
        let flip = (mine ^ theirs) & LATCH_SYNC;
        world.set_latch_of(sub, theirs ^ flip);
        Some(sub)
    }

    /// The task-node allocator. Takes a node from the pool, runs its
    /// constructor, and fills in the small-node fields with the argument.
    /// Returns the node, or `None` when the pool has none. The object is
    /// not read, so this takes no receiver.
    pub fn allocate_node_for_arg<W: SeekCoverWorld>(world: &mut W, arg: u32) -> Option<Handle32<TaskNode>> {
        let node = world.allocate_node()?;
        world.construct_node(node);
        world.store_small_node(node, &SmallNode { arg, flag: 0, tail: 0 });
        Some(node)
    }
}

/// Bit zero of the latch byte takes the bit zero of the source byte; the
/// other bits are kept. Used by the three constructors.
#[must_use]
pub const fn fold_latch_bit(latch: u8, source: u8) -> u8 {
    latch ^ ((latch ^ source) & 1)
}
