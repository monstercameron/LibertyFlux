//! The montage container object and its lifted state dispatcher.
//!
//! The object owns its words: six collaborator handles (the clip list, the
//! auxiliary, configuration, output, view and source objects), the wanted,
//! dropped, pending and cursor index words, and the mode and phase bytes.
//! Its collaborators are opaque handles, and every call to them goes through
//! [`MontageWorld`]. The editor's state flag the dispatcher reads is passed
//! in as [`MontageGlobals`], not reached through any state.

use lf_core::Handle32;

use super::world::{
    AuxTag, CfgTag, ListTag, MontageWorld, OutTag, OwnTag, SrcTag, Tag, ViewTag,
};

/// The freshness value at and above which the refresh region tears down.
/// The comparison is signed.
const FRESHNESS_LIMIT: i32 = 100;

/// The phase byte while the reconcile region runs.
const PHASE_RECONCILE: u8 = 3;
/// The phase byte once the reconcile region has re-armed the container.
const PHASE_IDLE: u8 = 0;

/// The shared globals the dispatcher reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MontageGlobals {
    /// The editor's state flag: zero clears the cursor and pending words
    /// and returns at once, nonzero opens a session.
    pub state_flag: u32,
}

/// The montage container object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MontageContainer {
    /// The handle the collaborators receive as the container itself.
    pub own: Handle32<OwnTag>,
    /// The clip list object. Always present: the original faults when it
    /// is null on any path that reaches it.
    pub list: Handle32<ListTag>,
    /// The auxiliary object the refresh region toggles. Always present.
    pub aux: Handle32<AuxTag>,
    /// The configuration object. Always present.
    pub cfg: Handle32<CfgTag>,
    /// The output object the refresh region signals. Always present.
    pub out: Handle32<OutTag>,
    /// The view object the reconcile region drives. Always present.
    pub view: Handle32<ViewTag>,
    /// The source object the regions notify through. Always present.
    pub src: Handle32<SrcTag>,
    /// The wanted index word.
    pub want: u32,
    /// The dropped pending index, written by the reconcile region.
    pub dropped: u32,
    /// The pending index word.
    pub pending: u32,
    /// The cursor index word.
    pub cursor: u32,
    /// The mode byte the refresh region toggles off.
    pub mode: u8,
    /// The phase byte the reconcile region sets and clears.
    pub phase: u8,
}

impl MontageContainer {
    /// The state dispatcher: opens a session for the state flag and walks
    /// three identity gates, each comparing the session's tag word against
    /// a wanted tag.
    ///
    /// The first gate taken decides the region: the list walk, the
    /// threshold-gated refresh, or the pending-index reconciliation. When
    /// no gate matches, the last wanted tag word is the result. A zero
    /// state flag clears the cursor and pending words and returns zero.
    pub fn dispatch_state(
        &mut self,
        world: &mut impl MontageWorld,
        globals: &MontageGlobals,
    ) -> u32 {
        if globals.state_flag == 0 {
            self.cursor = 0;
            self.pending = 0;
            return 0;
        }
        let session = world.open_session(globals.state_flag);
        world.own_open(self.own);
        let gate1 = world.session_tag(session);
        let want1 = world.tag_id(Tag::A);
        if gate1 == want1 {
            return self.run_list(world);
        }
        let gate2 = world.session_tag(session);
        let want2 = world.tag_id(Tag::D);
        if gate2 == want2 {
            return self.run_refresh(world, session);
        }
        let gate3 = world.session_tag(session);
        let want3 = world.tag_id(Tag::J);
        if gate3 != want3 {
            return want3;
        }
        self.run_reconcile(world)
    }

    /// The list region: activates the helper, then notifies two helpers
    /// per clip element with the element's two words. The count is
    /// re-read after every element, and the final count is the result.
    fn run_list(&mut self, world: &mut impl MontageWorld) -> u32 {
        world.activate(Tag::B);
        world.lookup(Tag::C);
        let mut count = world.list_count(self.list);
        if count != 0 {
            let mut index = 0u32;
            loop {
                let elem = world.list_at(self.list, index);
                let word_a = world.elem_word_a(elem);
                world.emit_a(self.src, index, word_a);
                let word_b = world.elem_word_b(elem);
                world.emit_b(self.src, index, word_b);
                index = index.wrapping_add(1);
                count = world.list_count(self.list);
                if index >= count {
                    break;
                }
            }
        }
        count
    }

    /// The refresh region: a freshness threshold that tears down at and
    /// above the limit (compared signed), otherwise a refresh sequence of
    /// keyed lookups, property propagation and one float blend value. The
    /// mode byte selects one of two toggle paths before the sequence. The
    /// released blend target's answer is the result.
    fn run_refresh(
        &mut self,
        world: &mut impl MontageWorld,
        session: Option<Handle32<super::world::SessionTag>>,
    ) -> u32 {
        let freshness = world.freshness(self.src);
        if (freshness as i32) >= FRESHNESS_LIMIT {
            world.note_teardown_mark();
            return world.state_teardown();
        }
        world.activate(Tag::E);
        world.state_refresh();
        if self.mode != 0 {
            self.mode = 0;
            world.aux_set(self.aux, 0);
            let tail = world.list_tail(self.list);
            world.tail_set(tail, 1);
            world.signal_out(self.out, 0);
        } else {
            let inner = world.list_inner(self.list);
            let probe = world.probe_open(inner);
            world.probe_prime(probe);
            let inner_again = world.list_inner(self.list);
            let probe_again = world.probe_open(inner_again);
            world.probe_set(probe_again, 0);
        }
        world.lookup(Tag::F);
        world.prepare(Tag::G);
        let handle = world.tag_id(Tag::H);
        let worker = world.open_worker(self.own, session, handle);
        let first = world.session_first(worker);
        let key = world.list_rank(self.list, first);
        world.note_rank(self.own, key);
        world.note_session(self.own, worker);
        let first_again = world.session_first(worker);
        let key_again = world.list_rank(self.list, first_again);
        world.list_touch(self.list, key_again, 1);
        world.session_mode(worker, 1);
        world.list_mode(self.list, 1);
        world.note_sync(self.own);
        let inner_a = world.list_inner(self.list);
        let prop = world.prop_read(inner_a);
        world.cfg_set(self.cfg, prop);
        let inner_b = world.list_inner(self.list);
        let prop_again = world.prop_read(inner_b);
        let blend = world.blend_level(self.src, prop_again);
        let target = world.resolve_output(Tag::I, blend);
        world.release_output(target)
    }

    /// The reconcile region: sets the phase byte, then reconciles the
    /// pending index against the list count. Either the pending entry is
    /// dropped (copied to the dropped word, cursor and pending cleared,
    /// the dropped entry returned) or the container is re-armed through
    /// its own entry (phase, cursor and pending cleared, that answer
    /// returned).
    ///
    /// The count is read twice and both reads take part: the cursor is
    /// compared against one less than the first, the wanted index against
    /// the second. All index arithmetic wraps.
    fn run_reconcile(&mut self, world: &mut impl MontageWorld) -> u32 {
        self.phase = PHASE_RECONCILE;
        let count_a = world.list_count(self.list);
        let cursor_seen = self.cursor == count_a.wrapping_sub(1);
        let count_b = world.list_count(self.list);
        let cursor = self.cursor;
        let want_seen = self.want == count_b;
        let adjacent = cursor == self.want || cursor.wrapping_add(1) == self.want;
        if self.pending != 0 && (!cursor_seen || !want_seen) && !adjacent {
            world.drop_pending(self.own, self.pending);
            let dropped = self.pending;
            self.dropped = dropped;
            self.cursor = 0;
            self.pending = 0;
            return dropped;
        }
        world.list_touch(self.list, cursor, 1);
        world.view_set(self.view, 0);
        let sub = world.cfg_sub(self.cfg);
        world.cfg_sub_set(sub, 0);
        let answer = world.own_rearm(self.own, 1);
        self.phase = PHASE_IDLE;
        self.cursor = 0;
        self.pending = 0;
        answer
    }
}
