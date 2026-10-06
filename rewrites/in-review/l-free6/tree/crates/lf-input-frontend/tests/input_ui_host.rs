//! Host tests for the lifted input-ui shapes: edge cases a reader of
//! the code would ask about. No rewrites here; these run on the 64-bit
//! host as well as on 32-bit.

use lf_input_frontend::input_ui::bounds::{
    BoundsSink, Measure, accumulate_bounds, fold_max, fold_min,
};
use lf_input_frontend::input_ui::holders::{
    Construct, ConstructedBody, CounterState, FreshBlock, HolderAlloc, HolderKind, HolderSink,
    PollSlot, UiHolder,
};
use lf_input_frontend::input_ui::records::{SubRecord, UiStateRecord};
use lf_input_frontend::input_ui::registry;
use std::collections::VecDeque;

// ---------------- shared fakes ----------------

struct FakeAlloc {
    queue: VecDeque<Option<FreshBlock>>,
}

impl HolderAlloc for FakeAlloc {
    fn alloc(&mut self, _size: u32) -> Option<FreshBlock> {
        self.queue.pop_front().flatten()
    }
}

struct FakeSink {
    seen: Vec<Option<UiHolder>>,
    answer: u32,
}

impl HolderSink for FakeSink {
    fn receive(&mut self, holder: Option<&UiHolder>) -> u32 {
        self.seen.push(holder.cloned());
        self.answer
    }
}

struct FakeCtor {
    kind: HolderKind,
    link: u32,
}

impl Construct for FakeCtor {
    fn construct(&mut self, slot: u32, _args: &[u32]) -> ConstructedBody {
        let _ = slot;
        ConstructedBody {
            kind: self.kind,
            link: self.link,
        }
    }
}

struct FakePoll {
    answers: VecDeque<u32>,
    calls: u32,
}

impl PollSlot for FakePoll {
    fn poll(&mut self, _holder: &UiHolder) -> u32 {
        self.calls += 1;
        self.answers.pop_front().unwrap()
    }
}

// ---------------- the stamp ----------------

#[test]
fn stamp_zero_stays_zero() {
    assert_eq!(UiHolder::stamp_link(0, 0), 0);
}

#[test]
fn stamp_counter_wins_inside_mask() {
    assert_eq!(UiHolder::stamp_link(0, 0x1234), 0x1234);
    assert_eq!(UiHolder::stamp_link(0xFFFF_FFFF, 0), 0xFFFF_C000);
    assert_eq!(UiHolder::stamp_link(0xABCD_1234, 0x5678), 0xABCD_1678);
}

#[test]
fn stamp_high_bits_survive() {
    // Only the low 14 bits change, whatever the counter holds above.
    assert_eq!(
        UiHolder::stamp_link(0xDEAD_BEEF, 0xFFFF_FFFF),
        0xDEAD_BEEF ^ ((0xDEAD_BEEF ^ 0xFFFF_FFFF) & 0x3FFF)
    );
    assert_eq!(UiHolder::stamp_link(0xDEAD_BEEF, 0xFFFF_FFFF), 0xDEAD_BFFF);
}

// ---------------- the poll-and-fold ----------------

fn fold(link: u32, v1: u32, v2: u32) -> (u32, u32) {
    let holder = UiHolder::from_parts(7, HolderKind::Ctor5, link, Vec::new());
    let mut poll = FakePoll {
        answers: VecDeque::from(vec![v1, v2]),
        calls: 0,
    };
    let out = UiHolder::poll_and_fold(link, &holder, &mut poll);
    assert_eq!(poll.calls, 2);
    out
}

#[test]
fn fold_zero_answers_fold_zero() {
    assert_eq!(fold(0, 0, 0), (0, 0));
}

#[test]
fn fold_unit_quotient_lands_at_bit_14() {
    assert_eq!(fold(0, 0, 16), (0x4000, 0x4000));
}

#[test]
fn fold_negative_dividend_truncates_toward_zero() {
    // v1 = -1: r1 = -1, t = 1. v2 = -1: q = 0. Answer 0.
    assert_eq!(fold(0, 0xFFFF_FFFF, 0xFFFF_FFFF), (0, 0));
    // v2 = -16, t = 0: q = -1, shifted and masked to all fold bits.
    assert_eq!(fold(0, 0, 0xFFFF_FFF0), (0x01FF_C000, 0x01FF_C000));
}

#[test]
fn fold_shifts_wrap_like_the_original() {
    // v2 = i32::MAX, t = 15: the sum wraps, the quotient is negative,
    // the shift wraps; every step matches release wrapping arithmetic.
    let q = 0x7FFF_FFFF_i32.wrapping_add(15) / 16;
    let m = (q.cast_unsigned().wrapping_shl(14) ^ 0x1234_5678) & 0x01FF_C000;
    assert_eq!(fold(0x1234_5678, 1, 0x7FFF_FFFF), (0x1234_5678 ^ m, m));
}

#[test]
fn fold_existing_bits_outside_mask_survive() {
    // Bits outside 14..24 pass through; inside, the quotient wins.
    let (link, m) = fold(0xFE00_3FFF, 0, 16);
    assert_eq!(m, 0x4000);
    assert_eq!(link, 0xFE00_3FFF ^ 0x4000);
}

// ---------------- the factories ----------------

fn block(slot: u32, link0: u32) -> FreshBlock {
    FreshBlock { slot, link0 }
}

#[test]
fn factory_null_alloc_hands_none_to_sink() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![None]),
    };
    let mut sink = FakeSink {
        seen: Vec::new(),
        answer: 0xA5,
    };
    let mut counter = CounterState::new(0xFFFF_FFFF);
    let got = UiHolder::create_callback(
        &mut alloc,
        &mut sink,
        &mut counter,
        HolderKind::Callback16,
        &[1, 2],
    );
    assert_eq!(got, 0xA5);
    assert_eq!(sink.seen, vec![None]);
    assert_eq!(counter.next, 0xFFFF_FFFF);
}

#[test]
fn factory_stamps_and_advances_wrapping_counter() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![Some(block(3, 0x1111_2222))]),
    };
    let mut sink = FakeSink {
        seen: Vec::new(),
        answer: 9,
    };
    let mut counter = CounterState::new(u32::MAX);
    let got = UiHolder::create_callback(
        &mut alloc,
        &mut sink,
        &mut counter,
        HolderKind::Callback12,
        &[0x77],
    );
    assert_eq!(got, 9);
    assert_eq!(counter.next, 0);
    let holder = sink.seen[0].as_ref().unwrap();
    assert_eq!(holder.slot(), 3);
    assert_eq!(holder.kind(), HolderKind::Callback12);
    assert_eq!(holder.link(), UiHolder::stamp_link(0x1111_2222, u32::MAX));
    assert_eq!(holder.words(), &[0x77]);
}

#[test]
#[should_panic(expected = "callback factory called for")]
fn factory_rejects_constructor_kind() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::new(),
    };
    let mut sink = FakeSink {
        seen: Vec::new(),
        answer: 0,
    };
    let mut counter = CounterState::new(0);
    let _ = UiHolder::create_callback(&mut alloc, &mut sink, &mut counter, HolderKind::Ctor5, &[]);
}

#[test]
#[should_panic(expected = "payload length")]
fn factory_rejects_wrong_payload() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::new(),
    };
    let mut sink = FakeSink {
        seen: Vec::new(),
        answer: 0,
    };
    let mut counter = CounterState::new(0);
    let _ = UiHolder::create_callback(
        &mut alloc,
        &mut sink,
        &mut counter,
        HolderKind::Callback12,
        &[1, 2],
    );
}

// ---------------- the constructor creates ----------------

#[test]
fn ctor_create_folds_into_constructor_link() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![Some(block(1, 0xDEAD))]),
    };
    let mut ctor = FakeCtor {
        kind: HolderKind::Ctor6,
        link: 0x1000,
    };
    let mut poll = FakePoll {
        answers: VecDeque::from(vec![0, 32]),
        calls: 0,
    };
    let (holder, answer) =
        UiHolder::create_through_ctor(&mut alloc, &mut ctor, &mut poll, HolderKind::Ctor6, &[9; 6]);
    // q = 2, m = (2 << 14) masked, link updated.
    assert_eq!(answer, 0x8000);
    assert_eq!(holder.link(), 0x1000 ^ 0x8000);
    assert_eq!(holder.words(), &[]);
    assert_eq!(poll.calls, 2);
}

#[test]
#[should_panic(expected = "faults on the table load")]
fn ctor_create_null_alloc_panics() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![None]),
    };
    let mut ctor = FakeCtor {
        kind: HolderKind::CtorTex,
        link: 0,
    };
    let mut poll = FakePoll {
        answers: VecDeque::new(),
        calls: 0,
    };
    let _ =
        UiHolder::create_through_ctor(&mut alloc, &mut ctor, &mut poll, HolderKind::CtorTex, &[]);
}

#[test]
#[should_panic(expected = "another kind")]
fn ctor_create_rejects_kind_mismatch() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![Some(block(1, 0))]),
    };
    let mut ctor = FakeCtor {
        kind: HolderKind::Ctor6,
        link: 0,
    };
    let mut poll = FakePoll {
        answers: VecDeque::new(),
        calls: 0,
    };
    let _ = UiHolder::create_through_ctor(&mut alloc, &mut ctor, &mut poll, HolderKind::Ctor5, &[]);
}

#[test]
fn inline_create_stamps_then_folds() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::from(vec![Some(block(2, 0))]),
    };
    let mut counter = CounterState::new(0x3FFF);
    let mut poll = FakePoll {
        answers: VecDeque::from(vec![0, 16]),
        calls: 0,
    };
    let (holder, answer) =
        UiHolder::create_inline(&mut alloc, &mut counter, &mut poll, 0xAA, [1, 2, 3]);
    assert_eq!(counter.next, 0x4000);
    assert_eq!(answer, 0x4000);
    assert_eq!(holder.link(), 0x3FFF ^ 0x4000);
    assert_eq!(holder.words(), &[0xAA, 1, 2, 3]);
}

#[test]
#[should_panic(expected = "faults on the table load")]
fn inline_create_null_alloc_panics() {
    let mut alloc = FakeAlloc {
        queue: VecDeque::new(),
    };
    let mut counter = CounterState::new(0);
    let mut poll = FakePoll {
        answers: VecDeque::new(),
        calls: 0,
    };
    let _ = UiHolder::create_inline(&mut alloc, &mut counter, &mut poll, 0, [0; 3]);
}

// ---------------- the state record ----------------

#[derive(Clone, PartialEq, Eq, Debug)]
struct WordSub {
    id: u8,
    words: Vec<u32>,
    log: std::rc::Rc<std::cell::RefCell<Vec<u8>>>,
}

impl SubRecord for WordSub {
    fn copy_from_source(&mut self, src: &Self) {
        self.log.borrow_mut().push(self.id);
        self.words.clone_from(&src.words);
    }
}

fn test_record(
    log: std::rc::Rc<std::cell::RefCell<Vec<u8>>>,
    base: u32,
) -> UiStateRecord<WordSub, WordSub> {
    UiStateRecord {
        head: [base; 24],
        sub_head: WordSub {
            id: 1,
            words: vec![base + 1; 4],
            log: log.clone(),
        },
        mid: [base + 2; 64],
        sub_tail: WordSub {
            id: 2,
            words: vec![base + 3; 4],
            log,
        },
        tail: [base + 4; 6],
    }
}

#[test]
fn record_copy_delegates_in_order() {
    let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let src = test_record(std::rc::Rc::new(std::cell::RefCell::new(Vec::new())), 100);
    let mut dst = test_record(log.clone(), 0);
    dst.copy_from(&src);
    assert_eq!(*log.borrow(), vec![1u8, 2u8]);
    assert_eq!(dst.head, [100; 24]);
    assert_eq!(dst.sub_head.words, vec![101; 4]);
    assert_eq!(dst.mid, [102; 64]);
    assert_eq!(dst.sub_tail.words, vec![103; 4]);
    assert_eq!(dst.tail, [104; 6]);
}

#[test]
fn record_copy_self_copy_keeps_values() {
    let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut rec = test_record(log.clone(), 5);
    let snapshot = rec.clone();
    rec.copy_from(&snapshot);
    assert_eq!(rec.head, snapshot.head);
    assert_eq!(*log.borrow(), vec![1u8, 2u8]);
}

// ---------------- the bounds accumulator ----------------

struct ScriptMeasure {
    triples: VecDeque<([f32; 3], [f32; 3])>,
}

impl Measure for ScriptMeasure {
    fn measure(&mut self, _item: usize, lo: &mut [f32; 3], hi: &mut [f32; 3]) {
        let (l, h) = self.triples.pop_front().unwrap();
        *lo = l;
        *hi = h;
    }
}

struct LastSink {
    seen: Option<([f32; 3], [f32; 3])>,
    answer: u32,
}

impl BoundsSink for LastSink {
    fn report(
        &mut self,
        lo: &[f32; 3],
        hi: &[f32; 3],
        _tag: u32,
        _extra: u32,
        flag: &mut u32,
        _mode: u32,
    ) -> u32 {
        assert_eq!(*flag, 1);
        self.seen = Some((*lo, *hi));
        self.answer
    }
}

#[test]
fn folds_follow_nan_rules() {
    assert!(fold_max(1.0, f32::NAN).is_nan());
    assert_eq!(fold_max(f32::NAN, 1.0).to_bits(), 1.0f32.to_bits());
    assert!(fold_min(1.0, f32::NAN).is_nan());
    assert_eq!(fold_min(f32::NAN, 1.0).to_bits(), 1.0f32.to_bits());
}

#[test]
fn folds_distinguish_signed_zeroes() {
    // Neither +0.0 > -0.0 nor -0.0 > +0.0, so both folds take `new`.
    assert_eq!(fold_max(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(fold_min(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(fold_max(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
}

#[test]
fn folds_keep_ordered_extremes() {
    assert_eq!(fold_max(2.0, 1.0).to_bits(), 2.0f32.to_bits());
    assert_eq!(fold_max(1.0, 2.0).to_bits(), 2.0f32.to_bits());
    assert_eq!(fold_min(2.0, 1.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(fold_min(1.0, 2.0).to_bits(), 1.0f32.to_bits());
    assert_eq!(
        fold_max(f32::INFINITY, 1.0).to_bits(),
        f32::INFINITY.to_bits()
    );
    assert_eq!(
        fold_min(f32::NEG_INFINITY, 1.0).to_bits(),
        f32::NEG_INFINITY.to_bits()
    );
}

#[test]
fn accumulate_empty_reports_seeds() {
    let mut measure = ScriptMeasure {
        triples: VecDeque::new(),
    };
    let mut sink = LastSink {
        seen: None,
        answer: 0xB,
    };
    let mut flag = 0xDEAD;
    let got = accumulate_bounds(0, 3.0, -3.0, &mut measure, &mut sink, 1, 2, &mut flag, 4);
    assert_eq!(got, 0xB);
    assert_eq!(flag, 1);
    assert_eq!(sink.seen, Some(([-3.0; 3], [3.0; 3])));
}

#[test]
fn accumulate_two_items_folds_both() {
    let mut measure = ScriptMeasure {
        triples: VecDeque::from(vec![
            ([0.0, 10.0, -10.0], [1.0, 2.0, 3.0]),
            ([5.0, -20.0, 0.0], [-1.0, 20.0, 2.0]),
        ]),
    };
    let mut sink = LastSink {
        seen: None,
        answer: 1,
    };
    let mut flag = 0;
    let _ = accumulate_bounds(2, 0.0, 0.0, &mut measure, &mut sink, 0, 0, &mut flag, 0);
    // maxima: max(0,1,-1)=1, max(0,2,20)=20, max(0,3,2)=3.
    // minima: min(0,0,5)=0, min(0,10,-20)=-20, min(0,-10,0)=-10.
    assert_eq!(sink.seen, Some(([0.0, -20.0, -10.0], [1.0, 20.0, 3.0])));
}

// ---------------- the registry ----------------

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(registry::ROWS.len(), 13);
    assert_eq!(registry::proven_count(), 8);
    assert_eq!(registry::missing_count(), 5);
}

#[test]
fn registry_names_the_holder_shape() {
    let names: Vec<&str> = registry::ROWS.iter().map(|row| row.name).collect();
    assert!(names.contains(&"input_ui_factory_10"));
    assert!(names.contains(&"input_ui_create_inline"));
    assert!(names.contains(&"input_ui_state_copy"));
    assert!(names.contains(&"input_ui_bounds_accumulate"));
}
