//! Host edge tests for the lifted composer classes.
//!
//! One case per question a reader of the code would ask: an empty filter,
//! the first decisive child, the mode bit past the thirty-second child, the
//! low byte of an answer, the registration flag, the fold's rotation, the
//! table layout after a reset, and the destructor's guards. Bit-exactness
//! against the verified rewrites is proven by the 32-bit differential
//! crate, not here.

use std::cell::Cell;
use std::rc::Rc;

use lf_animation::composer::combining_filter::MODE_BITS;
use lf_animation::composer::optimized_data::{
    COUNT_OFFSET, FREE_FLAG, TABLE_COUNT, TABLE_OFFSET, TABLE_STRIDE, TABLE_TARGET_SKEW,
    VALUE_COUNT,
};
use lf_animation::composer::registry::{self, State};
use lf_animation::composer::{
    BaseFilterCallees, BlockId, CombiningFilter, ComposerCallees, ComposerDataCallees, ComposerId,
    ComposerOptimized, ComposerOptimizedData,
    FilterOwner, HandlerId, InnerId, ItemStack, NodeId, ObjectManager, OutId, PollChild, TableEntry,
    ValueEntry,
};
use lf_core::boundary::Handle32;

/// A child that answers a fixed word and counts the polls it receives.
struct Fixed {
    answer: u32,
    polls: Rc<Cell<u32>>,
}

impl PollChild for Fixed {
    fn poll(&mut self, _a0: u32, _a1: u32, _a2: u32) -> u32 {
        self.polls.set(self.polls.get() + 1);
        self.answer
    }
}

fn fixed(answer: u32, polls: &Rc<Cell<u32>>) -> Box<dyn PollChild> {
    Box::new(Fixed {
        answer,
        polls: polls.clone(),
    })
}

fn handle(raw: u32) -> Option<Handle32<CombiningFilter>> {
    Handle32::new(raw)
}

#[test]
fn empty_filter_accepts() {
    let mut f = CombiningFilter::new(Vec::new(), 0);
    assert!(f.poll(1, 2, 3));
    let mut f = CombiningFilter::new(Vec::new(), u32::MAX);
    assert!(f.poll(0, 0, 0));
}

#[test]
fn and_term_rejects_on_zero_verdict() {
    let polls = Rc::new(Cell::new(0));
    // Mode bit clear: child 0 is an AND-term. A zero answer rejects.
    let mut f = CombiningFilter::new(vec![fixed(0, &polls), fixed(1, &polls)], 0);
    assert!(!f.poll(0, 0, 0));
    // The poll stops at the first decisive child.
    assert_eq!(polls.get(), 1);
}

#[test]
fn or_term_accepts_on_first_accepting_child() {
    let polls = Rc::new(Cell::new(0));
    // Mode bit 0 set: child 0 is an OR-term. Child 1 is never asked.
    let mut f = CombiningFilter::new(vec![fixed(1, &polls), fixed(0, &polls)], 1);
    assert!(f.poll(0, 0, 0));
    assert_eq!(polls.get(), 1);
}

#[test]
fn only_the_low_byte_of_an_answer_is_a_verdict() {
    let polls = Rc::new(Cell::new(0));
    // A nonzero word whose low byte is zero is a rejecting answer.
    let mut f = CombiningFilter::new(vec![fixed(0x0100, &polls)], 0);
    assert!(!f.poll(0, 0, 0));
    // A word whose low byte is nonzero accepts, whatever its high bits.
    let mut f = CombiningFilter::new(vec![fixed(0x0101_0001, &polls)], 0);
    assert!(f.poll(0, 0, 0));
}

#[test]
fn mode_bits_past_the_thirty_second_child_do_not_exist() {
    let polls = Rc::new(Cell::new(0));
    // Thirty-three children, every answer a rejection. An all-ones mode word
    // gives the first thirty-two children OR-terms, which pass over a
    // rejection; the thirty-third child has no mode bit, so it is an
    // AND-term and rejects the poll. Had its bit wrapped round to the first
    // one, it would pass over the rejection and the poll would accept.
    let children: Vec<Box<dyn PollChild>> = (0..MODE_BITS + 1).map(|_| fixed(0, &polls)).collect();
    let mut f = CombiningFilter::new(children, u32::MAX);
    assert!(!f.poll(0, 0, 0));
    assert_eq!(polls.get(), MODE_BITS as u32 + 1);
}

#[test]
fn initialise_clears_children_and_keeps_the_mode_word() {
    let polls = Rc::new(Cell::new(0));
    let mut f = CombiningFilter::new(vec![fixed(1, &polls)], 0xABCD);
    let mut owner = NoOwner;
    let id = f.initialise(handle(0x1000), 0, Some(&mut owner));
    assert_eq!(Handle32::raw_or_zero(id), 0x1000);
    assert_eq!(f.child_count(), 0);
    assert_eq!(f.mode(), 0xABCD);
}

/// An owner that records every registration.
#[derive(Default)]
struct RecordingOwner {
    registered: Vec<u32>,
}

impl FilterOwner for RecordingOwner {
    fn register_filter(&mut self, filter: Option<Handle32<CombiningFilter>>) {
        self.registered.push(Handle32::raw_or_zero(filter));
    }
}

/// Owner placeholder for runs that never register.
struct NoOwner;

impl FilterOwner for NoOwner {
    fn register_filter(&mut self, _filter: Option<Handle32<CombiningFilter>>) {
        panic!("this run must not register");
    }
}

#[test]
fn registration_follows_the_low_flag_bit_only() {
    let mut f = CombiningFilter::new(Vec::new(), 0);
    let mut owner = RecordingOwner::default();
    f.initialise(handle(0x20), 2, Some(&mut owner));
    assert!(owner.registered.is_empty());
    f.initialise(handle(0x20), 3, Some(&mut owner));
    assert_eq!(owner.registered, vec![0x20]);
}

#[test]
#[should_panic(expected = "registration asked for without an owner")]
fn registration_without_an_owner_is_outside_the_domain() {
    let mut f = CombiningFilter::new(Vec::new(), 0);
    f.initialise::<NoOwner>(handle(0x20), 1, None);
}

/// A base answer that records its argument.
struct Base(u32);

impl BaseFilterCallees for Base {
    fn base_answer(&mut self, _filter: Option<Handle32<CombiningFilter>>) -> u32 {
        self.0
    }
}

#[test]
fn fold_rotates_the_answer_and_keeps_the_word() {
    let f = CombiningFilter::new(Vec::new(), 0x0000_00FF);
    let folded = f.fold(handle(0x40), &mut Base(0x1234_5678));
    assert_eq!(folded, 0x0000_00FF ^ 0x5678_1234);
    assert_eq!(f.mode(), 0x0000_00FF);
}

/// Callees of the composer block that record what they were called with.
#[derive(Default)]
struct BlockCalls {
    reset: Vec<u32>,
    released: Vec<u32>,
}

impl ComposerDataCallees for BlockCalls {
    fn reset_optimized(&mut self, block: Option<BlockId>) {
        self.reset.push(Handle32::raw_or_zero(block));
    }

    fn release_child_frames(&mut self, block: Option<BlockId>, offset: u32) {
        self.released
            .push(Handle32::raw_or_zero(block).wrapping_add(offset));
    }
}

fn block(raw: u32) -> Option<BlockId> {
    Handle32::new(raw)
}

#[test]
fn reset_builds_the_pointer_table_and_returns_its_last_target() {
    let mut data = ComposerOptimizedData::new(
        [ValueEntry {
            first: 7,
            second: 9,
        }; VALUE_COUNT],
        [TableEntry { target: 0xDEAD }; TABLE_COUNT],
        5,
    );
    let mut calls = BlockCalls::default();
    let last = data.reset_tables(block(0x1000), &mut calls);

    assert_eq!(calls.released, vec![0x1004]);
    assert_eq!(data.count(), 0);
    assert!(data.values().iter().all(|v| *v == ValueEntry::default()));
    let first = TABLE_OFFSET + TABLE_TARGET_SKEW;
    assert_eq!(data.table()[0].target, first);
    assert_eq!(data.table()[1].target, first + TABLE_STRIDE);
    let last_expected = TABLE_OFFSET + (TABLE_COUNT as u32 - 1) * TABLE_STRIDE + TABLE_TARGET_SKEW;
    assert_eq!(last, last_expected);
    // The table base, 62 strides of 0x30 and the skew add up to 0xBEC.
    assert_eq!(last, 0xBEC);
    assert_eq!(COUNT_OFFSET, 0xBF0);
}

#[test]
fn reset_with_no_block_still_releases_the_header_at_offset_four() {
    let mut data = ComposerOptimizedData::default();
    let mut calls = BlockCalls::default();
    data.reset_tables(None, &mut calls);
    assert_eq!(calls.released, vec![4]);
}

/// A manager that records the blocks it frees.
#[derive(Default)]
struct FreeLog {
    freed: Vec<u32>,
}

impl ObjectManager for FreeLog {
    fn free_object(&mut self, block: BlockId) {
        self.freed.push(block.get());
    }
}

#[test]
fn destructor_frees_only_with_the_flag_and_an_identity() {
    let mut calls = BlockCalls::default();
    let mut mgr = FreeLog::default();

    let got = ComposerOptimizedData::scalar_deleting_destructor(
        block(0x500),
        0,
        &mut calls,
        Some(&mut mgr),
    );
    assert_eq!(Handle32::raw_or_zero(got), 0x500);
    assert!(mgr.freed.is_empty());

    ComposerOptimizedData::scalar_deleting_destructor(
        block(0x500),
        FREE_FLAG,
        &mut calls,
        Some(&mut mgr),
    );
    assert_eq!(mgr.freed, vec![0x500]);
    assert_eq!(calls.reset, vec![0x500, 0x500]);
}

#[test]
fn destructor_with_no_identity_needs_no_manager() {
    let mut calls = BlockCalls::default();
    let got = ComposerOptimizedData::scalar_deleting_destructor::<_, FreeLog>(
        None,
        FREE_FLAG,
        &mut calls,
        None,
    );
    assert_eq!(got, None);
    assert_eq!(calls.reset, vec![0]);
}

#[test]
#[should_panic(expected = "free asked for without a manager")]
fn destructor_that_must_free_needs_a_manager() {
    let mut calls = BlockCalls::default();
    ComposerOptimizedData::scalar_deleting_destructor::<_, FreeLog>(
        block(0x500),
        FREE_FLAG,
        &mut calls,
        None,
    );
}

#[test]
fn sequence_wraps_at_the_sixteen_bit_limit() {
    let mut c = ComposerOptimized::new(true, false, u16::MAX - 1, 0x55, None, None);
    c.advance_sequence(None, &mut NoCallees);
    assert_eq!(c.sequence(), 0);
    let mut c = ComposerOptimized::new(true, false, u16::MAX, 0x55, None, None);
    assert_eq!(c.advance_and_read_word(None, &mut NoCallees), 0x55);
    assert_eq!(c.sequence(), 1);
}

/// Callees for runs that must not call anything (immediate mode, no drain).
struct NoCallees;

impl ComposerCallees for NoCallees {
    fn sync(&mut self, _composer: Option<ComposerId>) {
        panic!("this run must not sync");
    }
    fn handle_table_entry(&mut self, _e: Option<HandlerId>, _a: u32, _b: u32, _c: u32, _d: u32, _e2: u32) {
        panic!("this run must not dispatch");
    }
    fn check(&mut self, _composer: Option<ComposerId>, _token: Option<OutId>) -> u32 {
        panic!("this run must not gate");
    }
    fn handle_node(&mut self, _composer: Option<ComposerId>, _node: Option<NodeId>) {
        panic!("this run must not handle a node");
    }
    fn drain(&mut self, _composer: Option<ComposerId>) {
        panic!("this run must not drain");
    }
    fn forward_inner(&mut self, _inner: Option<InnerId>) -> u32 {
        panic!("this run must not forward");
    }
    fn apply_pose_direct(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        panic!("this run must not forward");
    }
    fn apply_static_tracks_to_pose(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        panic!("this run must not forward");
    }
    fn copy_dof_pair(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        panic!("this run must not forward");
    }
    fn setup_identity_frame(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        panic!("this run must not forward");
    }
}

/// Callees that record the order of their calls.
#[derive(Default)]
struct OrderLog {
    calls: Vec<&'static str>,
}

impl ComposerCallees for OrderLog {
    fn sync(&mut self, _composer: Option<ComposerId>) {
        self.calls.push("sync");
    }
    fn handle_table_entry(&mut self, _e: Option<HandlerId>, _a: u32, _b: u32, _c: u32, _d: u32, _e2: u32) {
        self.calls.push("dispatch");
    }
    fn check(&mut self, _composer: Option<ComposerId>, _token: Option<OutId>) -> u32 {
        self.calls.push("check");
        1
    }
    fn handle_node(&mut self, _composer: Option<ComposerId>, _node: Option<NodeId>) {
        self.calls.push("handle");
    }
    fn drain(&mut self, _composer: Option<ComposerId>) {
        self.calls.push("drain");
    }
    fn forward_inner(&mut self, _inner: Option<InnerId>) -> u32 {
        self.calls.push("forward");
        0x77
    }
    fn apply_pose_direct(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        self.calls.push("pose");
    }
    fn apply_static_tracks_to_pose(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        self.calls.push("tracks");
    }
    fn copy_dof_pair(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        self.calls.push("copy");
    }
    fn setup_identity_frame(&mut self, _c: Option<ComposerId>, _a0: u32, _a1: u32, _l: Option<ComposerId>) {
        self.calls.push("setup");
    }
}

#[test]
fn drain_and_forward_is_skipped_when_linked() {
    let inner = Handle32::new(0x900);
    let mut c = ComposerOptimized::new(false, true, 0, 0, Handle32::new(0x800), inner);
    let mut calls = OrderLog::default();
    assert_eq!(c.drain_and_forward(Handle32::new(0x100), &mut calls), 0);
    assert!(calls.calls.is_empty());
    assert!(c.free_node().is_some(), "a linked composer keeps its free node");
}

#[test]
fn drain_and_forward_syncs_drains_clears_the_cache_then_forwards() {
    let inner = Handle32::new(0x900);
    let mut c = ComposerOptimized::new(true, false, 0, 0, Handle32::new(0x800), inner);
    let mut calls = OrderLog::default();
    // Immediate mode does not stop this sync: the original always runs it.
    assert_eq!(c.drain_and_forward(Handle32::new(0x100), &mut calls), 0x77);
    assert_eq!(calls.calls, vec!["sync", "drain", "forward"]);
    assert!(c.free_node().is_none());
}

#[test]
#[should_panic(expected = "drain and forward needs an inner object")]
fn drain_and_forward_without_an_inner_object_is_outside_the_domain() {
    let mut c = ComposerOptimized::new(false, false, 0, 0, None, None);
    c.drain_and_forward(Handle32::new(0x100), &mut OrderLog::default());
}

#[test]
fn forwarders_sync_only_while_immediate_mode_is_off() {
    let c = ComposerOptimized::new(false, false, 0, 0, None, None);
    let mut calls = OrderLog::default();
    c.forward_two_words_pose(Handle32::new(0x100), 1, 2, &mut calls);
    assert_eq!(calls.calls, vec!["sync", "pose"]);
    let c = ComposerOptimized::new(true, false, 0, 0, None, None);
    let mut calls = OrderLog::default();
    c.forward_one_word_copy(Handle32::new(0x100), 1, &mut calls);
    assert_eq!(calls.calls, vec!["copy"]);
}

#[test]
fn stack_read_pops_after_reading_the_old_top() {
    let c = ComposerOptimized::new(true, false, 0, 0, None, None);
    let mut stack = ItemStack {
        table: vec![10, 20, 30],
        top: 2,
    };
    let got = c.peek_or_pop(None, 0, &mut stack, &mut NoCallees);
    assert_eq!(got, 30, "the item read is the one on top before the pop");
    assert_eq!(stack.top, 1);
    let got = c.peek_or_pop(None, 1, &mut stack, &mut NoCallees);
    assert_eq!(got, 20);
    assert_eq!(stack.top, 1, "a nonzero low byte keeps the top");
}

#[test]
fn stack_read_pops_from_zero_to_the_wrapped_index() {
    let c = ComposerOptimized::new(true, false, 0, 0, None, None);
    let mut stack = ItemStack {
        table: vec![5],
        top: 0,
    };
    assert_eq!(c.peek_or_pop(None, 0x100, &mut stack, &mut NoCallees), 5, "low byte zero pops");
    assert_eq!(stack.top, u32::MAX);
}

#[test]
fn registry_counts_match_the_rows() {
    assert_eq!(registry::proven_count(), 16);
    assert_eq!(
        registry::ROWS.iter().filter(|r| r.state == State::Proven).count(),
        registry::proven_count()
    );
    assert_eq!(
        registry::ROWS.len(),
        registry::proven_count() + registry::missing_count()
    );
}


#[test]
fn table_dispatch_reads_the_top_entry_and_advances_by_one() {
    let mut c = ComposerOptimized::new(true, false, 7, 0, None, None).with_word_70(0x1234);
    let stack = ItemStack {
        table: vec![0xAA, 0xBB, 0xCC],
        top: 1,
    };
    let mut calls = OrderLog::default();
    c.dispatch_table_entry(Handle32::new(0x100), 9, &stack, &mut calls);
    assert_eq!(calls.calls, vec!["dispatch"]);
    assert_eq!(c.sequence(), 8, "the dispatch advances the counter by one");
    assert_eq!(c.word_70(), 0x1234);
}

#[test]
fn table_dispatch_syncs_only_while_immediate_mode_is_off() {
    let mut c = ComposerOptimized::new(false, false, 0, 0, None, None);
    let stack = ItemStack {
        table: vec![1],
        top: 0,
    };
    let mut calls = OrderLog::default();
    c.dispatch_table_entry(Handle32::new(0x100), 0, &stack, &mut calls);
    assert_eq!(calls.calls, vec!["sync", "dispatch"]);
}
