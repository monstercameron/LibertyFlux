//! Host tests of the layout frame's call-out methods, on the 64-bit host.
//!
//! The 32-bit differential crate proves each method against its rewrite. These
//! tests cover the edge cases a reader of the code would ask about, with small
//! hosts that record what they are asked: the empty branch of each method, the
//! ordering of calls, and the fallbacks.

use super::blend::BlendHost;
use super::follow_up::{FollowUpHost, SharedFlag, Target, TargetStep};
use super::indexed::{IndexedListHost, ListAnswer};
use super::list::ListHost;
use super::parameter_block::{BLOCK_BYTES, ParameterBlockHost};
use super::records::RecordHost;
use super::{SPAN, UILayoutFrame, field};

/// A follow-up host that records each call and answers zero.
#[derive(Default)]
struct Calls {
    log: Vec<&'static str>,
    token_seen: Option<u32>,
    flags: [u8; 3],
}

impl FollowUpHost for Calls {
    fn resolve_token(&mut self, token: u32) -> Target {
        self.log.push("resolve");
        self.token_seen = Some(token);
        Target(0x100)
    }
    fn frame_step(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        self.log.push("frame_step");
        7
    }
    fn target_step(&mut self, _target: Target, _step: TargetStep, _arg: u32) -> u32 {
        self.log.push("target_step");
        0
    }
    fn frame_first_step(&mut self, _frame: &mut UILayoutFrame, _arg: u32) -> u32 {
        self.log.push("first");
        0
    }
    fn frame_second_step(&mut self, _frame: &mut UILayoutFrame, _arg: u32) -> u32 {
        self.log.push("second");
        0
    }
    fn frame_token(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        self.log.push("frame_token");
        0
    }
    fn target_follow_up(&mut self, _target: Target) -> u32 {
        self.log.push("target_follow_up");
        0
    }
    fn read_flag(&mut self, flag: SharedFlag) -> u8 {
        self.flags[flag as usize]
    }
    fn write_flag(&mut self, flag: SharedFlag, value: u8) {
        self.flags[flag as usize] = value;
    }
    fn notify(&mut self, _arg: u32) -> u32 {
        self.log.push("notify");
        0
    }
    fn shared_object(&mut self) -> u32 {
        0x200
    }
    fn call_sink(&mut self, _sink: u32, _stamp: u32) -> u32 {
        self.log.push("sink");
        0
    }
    fn handoff(&mut self, _object: u32) -> u32 {
        self.log.push("handoff");
        0
    }
}

#[test]
fn a_zero_token_makes_no_follow_up_call() {
    let mut frame = UILayoutFrame::new();
    let mut host = Calls::default();
    assert_eq!(frame.token_follow_up_when_nonzero(&mut host), 0);
    assert!(host.log.is_empty());
    assert_eq!(frame.token_follow_up_then_handoff(&mut host), 0);
    assert!(host.log.is_empty());
}

#[test]
fn a_token_runs_resolve_then_step_then_target_in_order() {
    let mut frame = UILayoutFrame::new();
    frame.set_word_d8(0x55);
    let mut host = Calls::default();
    frame.token_follow_up_when_nonzero(&mut host);
    assert_eq!(host.log, ["resolve", "frame_step", "target_step"]);
    assert_eq!(host.token_seen, Some(0x55));
}

#[test]
fn two_enables_run_after_the_follow_up_and_answer_the_second() {
    let mut frame = UILayoutFrame::new();
    frame.set_word_d4(3);
    let mut host = Calls::default();
    frame.token_follow_up_then_two_enables(&mut host);
    assert_eq!(
        host.log,
        ["resolve", "frame_step", "target_step", "first", "second"]
    );
}

#[test]
fn a_clear_pending_flag_takes_the_notifier_and_is_cleared() {
    let mut frame = UILayoutFrame::new();
    frame.set_word_e0(9);
    let mut host = Calls::default();
    host.flags[SharedFlag::PendingFor113 as usize] = 0;
    frame.token_follow_up_then_clear_pending(&mut host);
    assert!(host.log.contains(&"notify"), "a clear flag runs the notifier");
    assert_eq!(host.flags[SharedFlag::PendingFor113 as usize], 0);

    let mut host = Calls::default();
    host.flags[SharedFlag::PendingFor113 as usize] = 1;
    frame.token_follow_up_then_clear_pending(&mut host);
    assert!(!host.log.contains(&"notify"), "a set flag skips the notifier");
    assert_eq!(host.flags[SharedFlag::PendingFor113 as usize], 0, "and is cleared");
}

#[test]
fn a_set_handoff_flag_is_cleared_and_no_handoff_runs() {
    let mut frame = UILayoutFrame::new();
    frame.set_word_e4(4);
    let mut host = Calls::default();
    host.flags[SharedFlag::Handoff as usize] = 1;
    frame.token_follow_up_then_handoff(&mut host);
    assert!(!host.log.contains(&"handoff"));
    assert_eq!(host.flags[SharedFlag::Handoff as usize], 0);
}

/// A plus-one host whose target answers a fixed value.
struct PlusOne;

impl FollowUpHost for PlusOne {
    fn resolve_token(&mut self, _token: u32) -> Target {
        Target(1)
    }
    fn frame_step(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        0
    }
    fn target_step(&mut self, _target: Target, _step: TargetStep, _arg: u32) -> u32 {
        0
    }
    fn frame_first_step(&mut self, _frame: &mut UILayoutFrame, _arg: u32) -> u32 {
        0
    }
    fn frame_second_step(&mut self, _frame: &mut UILayoutFrame, _arg: u32) -> u32 {
        0
    }
    fn frame_token(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        5
    }
    fn target_follow_up(&mut self, _target: Target) -> u32 {
        u32::MAX
    }
    fn read_flag(&mut self, _flag: SharedFlag) -> u8 {
        0
    }
    fn write_flag(&mut self, _flag: SharedFlag, _value: u8) {}
    fn notify(&mut self, _arg: u32) -> u32 {
        0
    }
    fn shared_object(&mut self) -> u32 {
        0
    }
    fn call_sink(&mut self, _sink: u32, _stamp: u32) -> u32 {
        0
    }
    fn handoff(&mut self, _object: u32) -> u32 {
        0
    }
}

#[test]
fn the_plus_one_answer_wraps_at_the_top_of_the_range() {
    let mut frame = UILayoutFrame::new();
    assert_eq!(frame.resolve_token_and_follow_up_plus_one(&mut PlusOne), 0);
}

/// A blend host that records the handler's calls.
struct Blend {
    handler_calls: Vec<(u32, u32)>,
}

impl BlendHost for Blend {
    fn clamp_high(&mut self) -> f32 {
        1.0
    }
    fn half_factor(&mut self) -> f32 {
        0.5
    }
    fn blend_handler(&mut self, _frame: &mut UILayoutFrame, mode: u32, blended_bits: u32) -> u32 {
        self.handler_calls.push((mode, blended_bits));
        0
    }
}

#[test]
fn mode_one_stores_the_mode_and_does_not_blend() {
    let mut frame = UILayoutFrame::new();
    let mut host = Blend { handler_calls: Vec::new() };
    assert_eq!(frame.store_mode_then_blend_and_forward(&mut host, 1, 0x3F80_0000), 0);
    assert!(host.handler_calls.is_empty());
    assert_eq!(frame.bytes()[field::SLOT_14], 1);
}

#[test]
fn an_amount_above_the_bound_clamps_to_it_before_the_blend() {
    let mut frame = UILayoutFrame::new();
    frame.set_float_8(2.0);
    frame.set_slot_and_fraction(0, 0);
    let mut host = Blend { handler_calls: Vec::new() };
    frame.store_mode_then_blend_and_forward(&mut host, 2, 5.0_f32.to_bits());
    // scaled = 2.0 * 0.5 = 1.0; weighted = 2.0 * 1.0 = 2.0; blended = (0 - 1) + 2 = 1.0
    assert_eq!(host.handler_calls, [(2, 1.0_f32.to_bits())]);
}

#[test]
fn a_not_a_number_amount_passes_the_clamp_unchanged() {
    let mut frame = UILayoutFrame::new();
    let mut host = Blend { handler_calls: Vec::new() };
    let nan = f32::from_bits(0x7FC0_0001);
    frame.store_mode_then_blend_and_forward(&mut host, 2, nan.to_bits());
    let (_, bits) = host.handler_calls[0];
    assert!(f32::from_bits(bits).is_nan(), "a not-a-number stays not-a-number");
}

/// A record host whose container holds the given flags.
struct Records<'a> {
    flags: &'a [u8],
}

impl RecordHost for Records<'_> {
    fn query_records(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        0xABCD_EF12
    }
    fn record_count(&mut self, _container: u32) -> u16 {
        self.flags.len() as u16
    }
    fn record_flags(&mut self, _container: u32, index: usize) -> u8 {
        self.flags[index]
    }
}

#[test]
fn one_record_never_answers_true_and_keeps_the_container_high_bits() {
    let mut frame = UILayoutFrame::new();
    let answer = frame.two_flags_set_among_records(&mut Records { flags: &[0x0A] });
    assert_eq!(answer, 0xABCD_EF00);
}

#[test]
fn two_records_with_both_flags_answer_true() {
    let mut frame = UILayoutFrame::new();
    let answer = frame.two_flags_set_among_records(&mut Records { flags: &[0x02, 0x08] });
    assert_eq!(answer, 0xABCD_EF01);
}

#[test]
fn a_record_with_the_first_flag_counts_for_it_only() {
    let mut frame = UILayoutFrame::new();
    let answer = frame.two_flags_set_among_records(&mut Records { flags: &[0x0A, 0x0A] });
    assert_eq!(answer, 0xABCD_EF00, "both records carry 0x02, so the second bit never appears and the answer is false");
}

/// A list host over a list in plain Rust values.
struct List {
    entries: Vec<u32>,
    released: Vec<u32>,
    freed: Vec<u32>,
    owner: Option<(u32, u32)>,
}

impl ListHost for List {
    fn list_entry(&mut self, _list: u32, index: usize) -> u32 {
        self.entries[index]
    }
    fn release_entry(&mut self, entry: u32) {
        self.released.push(entry);
    }
    fn free_entry(&mut self, entry: u32) {
        self.freed.push(entry);
    }
    fn list_owner_released(&mut self, _frame: &mut UILayoutFrame, base: u32, end: u32) {
        self.owner = Some((base, end));
    }
}

#[test]
fn an_empty_list_reports_to_the_owner_and_releases_nothing() {
    let mut frame = UILayoutFrame::new();
    let mut host = List { entries: vec![], released: vec![], freed: vec![], owner: None };
    assert_eq!(frame.release_pointer_list_entries(&mut host), 0);
    assert!(host.released.is_empty() && host.freed.is_empty());
    assert_eq!(host.owner, Some((0, 0)));
}

#[test]
fn null_entries_are_skipped_and_the_others_are_released_then_freed() {
    let mut frame = UILayoutFrame::new();
    frame.set_word_at_for_list(0x1000, 3);
    let mut host = List {
        entries: vec![0x11, 0, 0x33],
        released: vec![],
        freed: vec![],
        owner: None,
    };
    frame.release_pointer_list_entries(&mut host);
    assert_eq!(host.released, [0x11, 0x33]);
    assert_eq!(host.freed, [0x11, 0x33]);
    assert_eq!(host.owner, Some((0x1000, 0x1000 + 12)));
}

impl UILayoutFrame {
    /// Test helper: sets the list base and the count the release reads.
    fn set_word_at_for_list(&mut self, base: u32, count: u16) {
        self.set_word_at(field::LIST_BASE, base);
        self.set_word_at(field::LIST_COUNT, u32::from(count));
    }
}

/// An indexed-list host over scripted answers.
struct Index {
    first: u32,
    record: u32,
    list_len: u16,
    resolves: Vec<u32>,
}

impl IndexedListHost for Index {
    fn first_query(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        self.first
    }
    fn second_query(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        0
    }
    fn registry_record(&mut self, _key: u32) -> u32 {
        self.record
    }
    fn registry_list(&mut self, _record: u32) -> u32 {
        0x77
    }
    fn list_count(&mut self, _list: u32) -> u16 {
        self.list_len
    }
    fn list_entry(&mut self, _list: u32, index: usize) -> u32 {
        index as u32
    }
    fn entry_key(&mut self, entry: u32) -> u32 {
        entry
    }
    fn key_resolves(&mut self, key: u32) -> u32 {
        self.resolves[key as usize]
    }
}

#[test]
fn an_empty_list_passes_and_answers_the_list() {
    let mut frame = UILayoutFrame::new();
    let mut host = Index { first: 1, record: 1, list_len: 0, resolves: vec![] };
    assert_eq!(frame.every_list_entry_resolves(&mut host), ListAnswer::List(0x77));
}

#[test]
fn one_refused_entry_falls_back_to_the_frame_field() {
    let mut frame = UILayoutFrame::new();
    let mut host = Index { first: 1, record: 1, list_len: 3, resolves: vec![1, 0, 1] };
    assert_eq!(frame.every_list_entry_resolves(&mut host), ListAnswer::FieldFallback);
}

#[test]
fn a_zero_first_query_falls_back_without_a_lookup() {
    let mut frame = UILayoutFrame::new();
    let mut host = Index { first: 0x100, record: 1, list_len: 3, resolves: vec![1, 1, 1] };
    assert_eq!(frame.every_list_entry_resolves(&mut host), ListAnswer::FieldFallback, "low byte zero");
}

/// A parameter-block host with a stamp that it answers in order.
struct Block {
    stamps: Vec<u32>,
    cursor: usize,
    words: [u32; 5],
    allocations: usize,
}

impl ParameterBlockHost for Block {
    fn frame_stamp(&mut self, _frame: &mut UILayoutFrame) -> u32 {
        let value = self.stamps[self.cursor];
        self.cursor += 1;
        value
    }
    fn allocate_block(&mut self, size: u32) -> u32 {
        assert_eq!(size, BLOCK_BYTES);
        self.allocations += 1;
        0x500
    }
    fn write_block_word(&mut self, _block: u32, index: usize, value: u32) {
        self.words[index] = value;
    }
    fn read_block_word(&mut self, _block: u32, index: usize) -> u32 {
        self.words[index]
    }
}

#[test]
fn a_first_refresh_allocates_and_writes_the_block_with_a_fresh_stamp() {
    let mut frame = UILayoutFrame::new();
    let mut host = Block { stamps: vec![0x42], cursor: 0, words: [9; 5], allocations: 0 };
    let answer = frame.refresh_parameter_block(&mut host, 0x3F00_0000, 6);
    assert_eq!(answer, 0x42);
    assert_eq!(host.allocations, 1);
    assert_eq!(host.words, [0x3F00_0000, 6, 0x42, 0, 0]);
    assert_eq!(frame.word_at_for_test(field::CACHE_EC), 0x500);
}

#[test]
fn a_matching_stamp_refreshes_and_a_different_one_leaves_the_block() {
    let mut frame = UILayoutFrame::from_bytes([0; SPAN]);
    frame.set_cache_for_test(0x500);

    let mut matching = Block { stamps: vec![0x42, 0x43], cursor: 0, words: [0, 0, 0x42, 0, 0], allocations: 0 };
    assert_eq!(frame.refresh_parameter_block(&mut matching, 1, 2), 0x43);
    assert_eq!(matching.words[2], 0x43, "a match writes the fresh stamp");

    let mut other = Block { stamps: vec![0x44], cursor: 0, words: [0, 0, 0x99, 0, 0], allocations: 0 };
    assert_eq!(frame.refresh_parameter_block(&mut other, 1, 2), 0x44);
    assert_eq!(other.words, [0, 0, 0x99, 0, 0], "a different stamp leaves the block alone");
}

impl UILayoutFrame {
    /// Test helper: reads a word of the frame.
    fn word_at_for_test(&self, at: usize) -> u32 {
        self.word_at(at)
    }

    /// Test helper: sets the cache word.
    fn set_cache_for_test(&mut self, value: u32) {
        self.set_word_at(field::CACHE_EC, value);
    }
}
