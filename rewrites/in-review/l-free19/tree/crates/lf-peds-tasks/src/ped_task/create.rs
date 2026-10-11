//! Task creation: the kind-`0x11` builder and the entry-chain cloner.
//!
//! Two small structures share this module, one with two routines each.
//!
//! The builder serves the two build routines, which differ only in how
//! many words they forward to the cdecl builder: four
//! (`handle, found, arg2, kind`) or five (`handle, found, arg2, arg3,
//! kind`). [`BuildManagers::build_task`] is one generic method over the
//! trailing words, proven once per arity. The two manager words the
//! 32-bit form reads from globals are fields of [`BuildManagers`]; a
//! null handle skips the ped lookup, a live flagged task is returned
//! directly, and everything else builds through the [`TaskBuildCtx`]
//! collaborator.
//!
//! The cloner serves the two chain routines, which differ only in
//! whether the product's flag word is marked done afterwards.
//! [`ChainCloner::clone_chain`] is one generic method over that write,
//! proven once per instance. Entries are found through the [`ChainClone`]
//! collaborator, made through the owner, and finished through setters
//! and direct stores.

#![forbid(unsafe_code)]

use lf_core::Handle32;

/// The task kind the builder constructs.
pub const KIND_11: u32 = 0x11;
/// Flag bits or-ed into each entry's flag word before the maker runs.
pub const FLAG_EXTRA: u32 = 0x4000;
/// Flag bits or-ed into the product's flag word (done-marking instance).
pub const FLAG_DONE: u32 = 0x0040_0000;
/// Priority word passed to both makers.
pub const PRIORITY: u32 = 0xC100_0000;
/// Selector sentinel: unknown or absent.
pub const NONE: u32 = 0xFFFF_FFFF;

/// The ped manager behind the ped-handle lookup (opaque identity).
#[derive(Debug)]
pub struct PedMgr;

/// The argument lookup behind the build (opaque identity).
#[derive(Debug)]
pub struct ArgLookup;

/// A kind-`0x11` task (opaque identity).
#[derive(Debug)]
pub struct Kind11Task;

/// The chain owner's manager (opaque identity).
#[derive(Debug)]
pub struct ChainOwner;

/// A chain entry node (opaque identity).
#[derive(Debug)]
pub struct ChainNode;

/// A freshly made chain product (opaque identity).
#[derive(Debug)]
pub struct CloneProduct;

/// The two manager words the build routines read.
///
/// In the 32-bit form these are global words holding manager object
/// addresses; here they are handles the context calls back through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildManagers {
    /// The ped manager (`G_PEDMGR` word).
    pub ped_mgr: Option<Handle32<PedMgr>>,
    /// The argument lookup (`G_LOOKUP` word).
    pub lookup: Option<Handle32<ArgLookup>>,
}

/// What the ped's current task slot holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedTaskOutcome {
    /// The ped lookup answered null: the 32-bit form reads the task
    /// slot through the null ped and faults. The lift panics instead.
    NullPed,
    /// A null task, or a task with a clear flag: fall through to the build.
    Build,
    /// A task with a set flag: returned directly.
    Keep(Handle32<Kind11Task>),
}

/// The builder's collaborators: ped lookup, argument lookup, construction.
pub trait TaskBuildCtx {
    /// Reads the ped's current task slot through the ped manager.
    fn ped_task(&mut self, mgr: Option<Handle32<PedMgr>>, handle: u32) -> PedTaskOutcome;
    /// Resolves the second argument through the argument lookup.
    fn resolve_arg(&mut self, lookup: Option<Handle32<ArgLookup>>, arg1: u32) -> u32;
    /// Builds the kind-`0x11` task, forwarding the trailing words and kind.
    fn build_kind11(
        &mut self,
        handle: u32,
        found: u32,
        words: &[u32],
        kind: u32,
    ) -> Option<Handle32<Kind11Task>>;
}

impl BuildManagers {
    /// Builds the managers from their words.
    #[must_use]
    pub const fn new(
        ped_mgr: Option<Handle32<PedMgr>>,
        lookup: Option<Handle32<ArgLookup>>,
    ) -> Self {
        Self { ped_mgr, lookup }
    }

    /// Builds a kind-`0x11` task for a ped handle, forwarding `N` words.
    ///
    /// `N = 1` is the four-word build, `N = 2` the five-word build. A
    /// null handle skips the ped lookup; a live flagged task is kept;
    /// otherwise the second argument resolves and the builder runs.
    ///
    /// # Panics
    ///
    /// When the ped lookup answers null: the original faults there.
    pub fn build_task<C: TaskBuildCtx, const N: usize>(
        &self,
        ctx: &mut C,
        handle: u32,
        arg1: u32,
        words: [u32; N],
    ) -> Option<Handle32<Kind11Task>> {
        if handle != 0 {
            match ctx.ped_task(self.ped_mgr, handle) {
                PedTaskOutcome::NullPed => panic!("null ped lookup faults in the original"),
                PedTaskOutcome::Build => {}
                PedTaskOutcome::Keep(task) => return Some(task),
            }
        }
        let found = ctx.resolve_arg(self.lookup, arg1);
        ctx.build_kind11(handle, found, &words, KIND_11)
    }
}

/// One chain entry's words, as the maker calls see them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainEntry {
    /// Flag word (`+0x04`).
    pub flags: u32,
    /// Auxiliary word (`+0x08`).
    pub aux: u32,
    /// First selector (`+0x0c`).
    pub x: u32,
    /// Second selector (`+0x10`).
    pub y: u32,
    /// Alternate first selector (`+0x14`).
    pub alt_a: u32,
    /// Alternate second selector (`+0x18`).
    pub alt_b: u32,
    /// First payload (`+0x4c`, float bits).
    pub f1: u32,
    /// Second payload (`+0x54`, float bits).
    pub f2: u32,
    /// Third payload (`+0x58`, float bits).
    pub f3: u32,
}

/// A found entry: its node identity and its words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundEntry {
    /// The answering node.
    pub node: Handle32<ChainNode>,
    /// The node's words.
    pub entry: ChainEntry,
}

/// The cloner's collaborators: chain lookup, makers, setters, stores.
pub trait ChainClone {
    /// Finds the first entry for `key`.
    fn find_first(&mut self, key: u32) -> Option<FoundEntry>;
    /// Finds the next entry for `key`.
    fn find_next(&mut self, key: u32) -> Option<FoundEntry>;
    /// Makes a product from the plain selectors.
    #[allow(clippy::too_many_arguments)]
    fn make_full(
        &mut self,
        owner: Option<Handle32<ChainOwner>>,
        y: u32,
        x: u32,
        flags: u32,
        aux: u32,
        priority: u32,
        none: u32,
    ) -> Option<Handle32<CloneProduct>>;
    /// Makes a product from the alternate selectors.
    fn make_alt(
        &mut self,
        owner: Option<Handle32<ChainOwner>>,
        a: u32,
        b: u32,
        flags: u32,
        aux: u32,
        priority: u32,
    ) -> Option<Handle32<CloneProduct>>;
    /// Sets the product's first payload through its setter.
    fn set_first(&mut self, product: Handle32<CloneProduct>, f1: u32);
    /// Stores the product's second payload directly.
    fn store_second(&mut self, product: Handle32<CloneProduct>, f2: u32);
    /// Sets the product's third payload through its setter.
    fn set_third(&mut self, product: Handle32<CloneProduct>, f3: u32);
    /// Marks the product's flag word done (done-marking instance only).
    fn mark_done(&mut self, product: Handle32<CloneProduct>);
}

/// The entry-chain cloner: an owner that clones each entry of a chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainCloner {
    /// The owner (manager) the maker calls run through (`this+0x78`).
    pub owner: Option<Handle32<ChainOwner>>,
}

impl ChainCloner {
    /// Builds a cloner for an owner.
    #[must_use]
    pub const fn new(owner: Option<Handle32<ChainOwner>>) -> Self {
        Self { owner }
    }

    /// Walks the entry chain for `key`, cloning each entry.
    ///
    /// Each entry's flags gain [`FLAG_EXTRA`], then one maker runs: the
    /// full maker when the auxiliary word is zero or neither selector
    /// is [`NONE`], else the alternate maker. A null product skips the
    /// entry; otherwise the three payloads land and, when `SET_DONE`,
    /// the product's flag word gains [`FLAG_DONE`].
    pub fn clone_chain<C: ChainClone, const SET_DONE: bool>(&self, ctx: &mut C, key: u32) {
        let mut node = ctx.find_first(key);
        while let Some(found) = node {
            let e = &found.entry;
            let flags = e.flags | FLAG_EXTRA;
            // The 32-bit form tests y then x with identical bodies;
            // one condition covers both.
            let product = if e.aux == 0 {
                ctx.make_full(self.owner, e.y, e.x, flags, e.aux, PRIORITY, NONE)
            } else if e.y == NONE || e.x == NONE {
                ctx.make_alt(self.owner, e.alt_a, e.alt_b, flags, e.aux, PRIORITY)
            } else {
                ctx.make_full(self.owner, e.y, e.x, flags, e.aux, PRIORITY, NONE)
            };
            if let Some(p) = product {
                ctx.set_first(p, e.f1);
                ctx.store_second(p, e.f2);
                ctx.set_third(p, e.f3);
                if SET_DONE {
                    ctx.mark_done(p);
                }
            }
            node = ctx.find_next(key);
        }
    }
}
