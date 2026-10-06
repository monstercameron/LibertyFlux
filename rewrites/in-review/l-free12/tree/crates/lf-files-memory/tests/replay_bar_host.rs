//! Host tests for the lifted replay bar: edge cases on the 64-bit host.
//!
//! NaN payloads differ across architectures, so NaN results assert
//! `is_nan` here; every other float asserts exact bits. Panic domains
//! assert the lift refuses what the original leaves undefined.

use lf_files_memory::replay_bar::InnerHandle;
use lf_files_memory::replay_bar::NotifyCtl;
use lf_files_memory::replay_bar::RefreshOutcome;
use lf_files_memory::replay_bar::ReplayBar;
use lf_files_memory::replay_bar::ReplaySlot;
use lf_files_memory::replay_bar::StampPublish;
use lf_files_memory::replay_bar::TimeBases;
use lf_files_memory::replay_bar::blend_factors;
use lf_files_memory::replay_bar::registry;

#[test]
fn registry_counts_match_rows() {
    let proven = registry::ROWS
        .iter()
        .filter(|r| r.state == registry::State::Proven)
        .count();
    assert_eq!(proven, registry::PROVEN);
    assert_eq!(registry::ROWS.len(), registry::TOTAL);
}

#[test]
fn registry_has_no_half_lifted_rows() {
    for row in registry::ROWS {
        assert_ne!(row.state, registry::State::Lifted, "{}", row.routine);
    }
}

fn ranged_bar() -> ReplayBar {
    ReplayBar {
        lo: 2.0,
        hi: 10.0,
        total: 8,
        ..ReplayBar::empty()
    }
}

#[test]
fn scaled_index_rounds_half_away() {
    let bar = ranged_bar();
    assert_eq!(bar.scaled_index(2.0), 0);
    assert_eq!(bar.scaled_index(10.0), 8);
    assert_eq!(bar.scaled_index(6.0), 4);
    // Halfway slots round away from zero on both sides.
    let wide = ReplayBar {
        lo: -4.0,
        hi: 4.0,
        total: 4,
        ..ReplayBar::empty()
    };
    assert_eq!(wide.scaled_index(1.0), 3); // 2.5 -> 3
    assert_eq!(wide.scaled_index(-9.0), -3); // -2.5 -> -3
}

#[test]
fn scaled_index_degenerate_spans() {
    let zero = ReplayBar {
        lo: 3.0,
        hi: 3.0,
        total: 8,
        ..ReplayBar::empty()
    };
    // 8/0 is infinite; the guarded conversion answers MIN.
    assert_eq!(zero.scaled_index(3.0), i32::MIN);
    assert_eq!(zero.scaled_index(99.0), i32::MIN);
    let nan = ReplayBar {
        lo: f32::NAN,
        hi: 1.0,
        total: 8,
        ..ReplayBar::empty()
    };
    assert_eq!(nan.scaled_index(0.5), i32::MIN);
    assert_eq!(ranged_bar().scaled_index(f32::NAN), i32::MIN);
}

#[test]
fn clamped_index_clamps_then_scales() {
    let bar = ranged_bar();
    assert_eq!(bar.clamped_index(-100.0, false), 0);
    assert_eq!(bar.clamped_index(100.0, false), 8);
    // The second window binds tighter when on ...
    let win = ReplayBar {
        rect0: (0.0, 0.0, 4.0, 0.0),
        rect1: (6.0, 0.0, 0.0, 0.0),
        ..ranged_bar()
    };
    assert_eq!(win.clamped_index(1.0, true), 2); // clamped up to 4.0
    assert_eq!(win.clamped_index(1.0, false), 0); // outer window only
    // ... and a NaN input keeps its value through the clamps to MIN.
    assert_eq!(win.clamped_index(f32::NAN, true), i32::MIN);
}

#[test]
fn find_slot_edges() {
    let empty = ReplayBar::empty();
    assert_eq!(empty.find_slot(), None);
    let bar = ReplayBar {
        bound_hi: 10,
        slots: vec![
            ReplaySlot { tag: 1, stamp: 5 },
            ReplaySlot { tag: 2, stamp: 10 },
            ReplaySlot { tag: 3, stamp: 11 },
        ],
        ..ReplayBar::empty()
    };
    // Boundary equality passes; the last passing slot wins.
    assert_eq!(bar.find_slot(), Some(1));
    let below = ReplayBar {
        bound_hi: 4,
        ..bar.clone()
    };
    assert_eq!(below.find_slot(), None);
}

#[test]
fn millis_truncates_and_guards() {
    let at = |s: f32| ReplayBar {
        seconds: s,
        ..ReplayBar::empty()
    };
    assert_eq!(at(1.5).millis_rounded(), 1500);
    assert_eq!(at(1.9999).millis_rounded(), 1999);
    assert_eq!(at(-1.5).millis_rounded(), (-1500i64) as u32);
    assert_eq!(at(0.0).millis_rounded(), 0);
    assert_eq!(at(f32::NAN).millis_rounded(), 0);
    assert_eq!(at(f32::INFINITY).millis_rounded(), 0);
    assert_eq!(at(1e30).millis_rounded(), 0);
}

#[test]
fn hit_test_is_strict() {
    let bar = ReplayBar {
        rect0: (0.0, 0.0, 10.0, 10.0),
        rect1: (20.0, 0.0, 0.0, 0.0),
        ..ReplayBar::empty()
    };
    assert!(bar.hit_test(15.0, 5.0));
    // Every edge fails: x in (10, 20), y in (0, 10).
    assert!(!bar.hit_test(10.0, 5.0));
    assert!(!bar.hit_test(20.0, 5.0));
    assert!(!bar.hit_test(15.0, 0.0));
    assert!(!bar.hit_test(15.0, 10.0));
    assert!(!bar.hit_test(f32::NAN, 5.0));
    assert!(!bar.hit_test(15.0, f32::NAN));
}

#[test]
fn region_hit_test_selects_rect() {
    let bar = ReplayBar {
        rect0: (0.0, 0.0, 10.0, 10.0),
        rect1: (20.0, 20.0, 30.0, 30.0),
        ..ReplayBar::empty()
    };
    assert!(bar.region_hit_test(5.0, 5.0, 0));
    assert!(!bar.region_hit_test(5.0, 5.0, 1));
    assert!(bar.region_hit_test(25.0, 25.0, 1));
    assert!(!bar.region_hit_test(25.0, 25.0, 0));
    for mode in [2, 3, 100, u32::MAX] {
        assert!(!bar.region_hit_test(5.0, 5.0, mode));
    }
}

#[test]
fn store_cursor_round_trip() {
    let mut bar = ReplayBar::empty();
    assert_eq!(bar.store_cursor(1.25, 42), 42);
    assert_eq!(bar.total, 42);
    assert_eq!(bar.cursor, 1.25f32.to_bits());
    // NaN payloads survive the store bit for bit (no arithmetic).
    let nan = f32::from_bits(0x7fc0_1234);
    bar.store_cursor(nan, 7);
    assert_eq!(bar.cursor, 0x7fc0_1234);
}

fn bases() -> TimeBases {
    TimeBases {
        num_narrow: 1,
        num_wide: 2,
        den_narrow: 4,
        den_wide: 8,
    }
}

#[test]
fn scaled_position_picks_denominator() {
    let bar = ReplayBar {
        total: 8,
        width: 100.0,
        origin: 5.0,
        ..ReplayBar::empty()
    };
    // (4/8)*100+5 = 55, times the picked word as signed.
    assert_eq!(
        bar.scaled_position(4, &mut || false, &bases()).to_bits(),
        (55.0f32 * 4.0).to_bits()
    );
    assert_eq!(
        bar.scaled_position(4, &mut || true, &bases()).to_bits(),
        (55.0f32 * 8.0).to_bits()
    );
    // A zero total divides by zero like the original.
    let zero = ReplayBar { total: 0, ..bar.clone() };
    assert!(zero.scaled_position(4, &mut || false, &bases()).is_infinite());
    assert!(zero.scaled_position(0, &mut || false, &bases()).is_nan());
}

#[test]
fn scans_cover_empty_and_edges() {
    let empty = ReplayBar::empty();
    assert_eq!(empty.scan_forward(0, &mut |_| 99), (0, 100));
    assert_eq!(empty.scan_backward(u32::MAX, &mut |_| 0), (0, 100, 0));
    let bar = ReplayBar {
        total: 7,
        slots: vec![
            ReplaySlot { tag: 10, stamp: 1 },
            ReplaySlot { tag: 20, stamp: 2 },
        ],
        ..ReplayBar::empty()
    };
    // Forward: equality never triggers; the re-score wins.
    let mut calls = 0;
    let (score, tag) = bar.scan_forward(5, &mut |_| {
        calls += 1;
        5
    });
    assert_eq!((score, tag), (7, 100));
    assert_eq!(calls, 2);
    // Backward over the same table triggers on the last entry first.
    let (score, tag, stamp) = bar.scan_backward(6, &mut |s| if s == 2 { 1 } else { 9 });
    assert_eq!((tag, stamp), (20, 2));
    assert_eq!(score, 1);
    // Nothing below zero: the table answers the default.
    assert_eq!(bar.scan_backward(0, &mut |_| 0), (0, 100, 0));
}

#[test]
fn scans_rescore_the_hit() {
    let bar = ReplayBar {
        slots: vec![ReplaySlot { tag: 9, stamp: 3 }],
        ..ReplayBar::empty()
    };
    let mut answers = [50u32, 60].into_iter();
    let (score, tag) = bar.scan_forward(40, &mut |_| answers.next().unwrap());
    assert_eq!((score, tag), (60, 9));
}

#[test]
fn measure_and_ratio_publish_markers() {
    let bar = ReplayBar {
        total: 8,
        hi: 10.0,
        lo: 2.0,
        bound_lo: 11,
        bound_hi: 22,
        ..ReplayBar::empty()
    };
    let mut seen = Vec::new();
    let (ratio, first, second) = bar.measure_slots(&mut |s| {
        seen.push(s);
        s * 2
    });
    assert_eq!(ratio.to_bits(), 1.0f32.to_bits());
    assert_eq!((first, second), (22, 44));
    assert_eq!(seen, [11, 22]);
    // The ratio publisher reads one sample over the same span.
    let (q, lo, hi) = bar.slot_ratio(&mut || 4);
    assert_eq!(q.to_bits(), 0.5f32.to_bits());
    assert_eq!((lo, hi), (11, 22));
    // A zero span divides by zero like the original.
    let flat = ReplayBar {
        hi: 3.0,
        lo: 3.0,
        ..bar.clone()
    };
    let (r, _, _) = flat.measure_slots(&mut |_| 0);
    assert!(r.is_infinite());
}

#[test]
fn clamp_bound_min_max_and_passthrough() {
    let mut bar = ReplayBar::empty();
    let mut sampled = false;
    assert_eq!(
        bar.clamp_bound(5, 0, &mut || {
            sampled = true;
            9
        }),
        0
    );
    assert!(sampled);
    assert_eq!(bar.bound_lo, 5);
    assert_eq!(bar.clamp_bound(5, 1, &mut || 9), 9);
    assert_eq!(bar.bound_hi, 9);
    // Unknown selectors store nothing but still sample, answering back.
    sampled = false;
    assert_eq!(
        bar.clamp_bound(5, 99, &mut || {
            sampled = true;
            1
        }),
        99
    );
    assert!(sampled);
    assert_eq!((bar.bound_lo, bar.bound_hi), (5, 9));
}

#[test]
fn select_entry_empty_and_reselect() {
    let mut bar = ReplayBar::empty();
    let mut state = StampPublish { flags: 0, stamp: 0 };
    let mut watched = false;
    assert_eq!(
        bar.select_entry(0, &mut || watched = true, &mut || 0, &mut |_| {}, &mut state),
        None
    );
    assert!(!watched);
    // Re-selecting the index skips the watcher but still compares stamps.
    bar.slots = vec![ReplaySlot { tag: 0, stamp: 50 }];
    bar.selected = 0;
    let mut codes = Vec::new();
    let mut samples = [10u32, 90].into_iter();
    let got = bar.select_entry(
        0,
        &mut || watched = true,
        &mut || samples.next().unwrap(),
        &mut |c| codes.push(c),
        &mut state,
    );
    assert!(!watched);
    assert_eq!(codes, [6, 0x0e]);
    assert_eq!(got, Some(50));
    assert_eq!(state, StampPublish { flags: 1, stamp: 50 });
    assert_eq!(bar.sel_mark, 0xffff_ffff);
}

#[test]
fn select_entry_quiet_when_stamps_match() {
    let mut bar = ReplayBar {
        slots: vec![ReplaySlot { tag: 0, stamp: 50 }],
        ..ReplayBar::empty()
    };
    let mut state = StampPublish {
        flags: 0x10,
        stamp: 7,
    };
    let mut codes = Vec::new();
    let mut samples = [50u32, 50].into_iter();
    let got = bar.select_entry(
        0,
        &mut || {},
        &mut || samples.next().unwrap(),
        &mut |c| codes.push(c),
        &mut state,
    );
    // Equality publishes on neither side; the second sample wins.
    assert_eq!(got, Some(50));
    assert!(codes.is_empty());
    assert_eq!(state.stamp, 7);
    assert_eq!(state.flags, 0x10);
}

#[test]
#[should_panic(expected = "past the table")]
fn select_entry_past_the_end_panics() {
    let mut bar = ReplayBar {
        slots: vec![ReplaySlot { tag: 0, stamp: 1 }],
        ..ReplayBar::empty()
    };
    let mut state = StampPublish { flags: 0, stamp: 0 };
    let _ = bar.select_entry(1, &mut || {}, &mut || 0, &mut |_| {}, &mut state);
}

struct ScriptClock {
    a: u32,
    b: u32,
    calls: Vec<char>,
}

impl lf_files_memory::replay_bar::ClockRead for ScriptClock {
    fn clock_a(&mut self) -> u32 {
        self.calls.push('a');
        self.a
    }
    fn clock_b(&mut self) -> u32 {
        self.calls.push('b');
        self.b
    }
}

#[test]
fn time_factor_null_link_is_quiet_zero() {
    let bar = ReplayBar {
        weight: f32::NAN,
        ..ReplayBar::empty()
    };
    let mut clock = ScriptClock {
        a: 0,
        b: 0,
        calls: Vec::new(),
    };
    let mut selected = false;
    let got = bar.time_factor(
        &mut clock,
        &mut || {
            selected = true;
            true
        },
        &bases(),
    );
    assert_eq!(got.to_bits(), 0.0f32.to_bits());
    assert!(clock.calls.is_empty());
    assert!(!selected);
}

#[test]
fn time_factor_combines_ticks_and_bases() {
    let bar = ReplayBar {
        weight: 2.0,
        clock: lf_files_memory::replay_bar::ClockHandle::new(0x1234),
        ..ReplayBar::empty()
    };
    // (17 - 7 + 2*2)/1: the wide numerator feeds the product, the
    // narrow one the divisor (the denominator pair is unused here).
    let wide_bases = TimeBases {
        num_narrow: 1,
        num_wide: 2,
        den_narrow: 100,
        den_wide: 200,
    };
    let mut clock = ScriptClock {
        a: 0,
        b: 7,
        calls: Vec::new(),
    };
    let mut wides = [true, false].into_iter();
    let got = bar.time_factor(&mut clock, &mut || wides.next().unwrap(), &wide_bases);
    assert_eq!(got.to_bits(), 14.0f32.to_bits());
    assert_eq!(clock.calls, ['b']);
}

#[test]
fn blend_factors_scales_low_ratios() {
    // Ratio 1/2: the blend gains the ratio, the scale survives.
    let mut scale = 4.0;
    let mut clock = ScriptClock {
        a: 6,
        b: 3,
        calls: Vec::new(),
    };
    let mut wides = [false, false].into_iter();
    let narrow = TimeBases {
        num_narrow: 1,
        num_wide: 9,
        den_narrow: 2,
        den_wide: 9,
    };
    let (first, second, blend) =
        blend_factors(&mut clock, &mut scale, &narrow, &mut || wides.next().unwrap());
    assert_eq!((first.to_bits(), second.to_bits()), (6.0f32.to_bits(), 3.0f32.to_bits()));
    // (6/3)*4 = 8, times the ratio 1/2.
    assert_eq!(blend.to_bits(), 4.0f32.to_bits());
    assert_eq!(scale.to_bits(), 4.0f32.to_bits());
    assert_eq!(clock.calls, ['a', 'b']);
}

#[test]
fn blend_factors_divides_scale_on_high_ratios() {
    // Ratio 3/1: the scale is divided, the blend keeps the product.
    let mut scale = 12.0;
    let mut clock = ScriptClock {
        a: 6,
        b: 3,
        calls: Vec::new(),
    };
    let mut wides = [true, true].into_iter();
    let wide = TimeBases {
        num_narrow: 9,
        num_wide: 3,
        den_narrow: 9,
        den_wide: 1,
    };
    let (_, _, blend) =
        blend_factors(&mut clock, &mut scale, &wide, &mut || wides.next().unwrap());
    assert_eq!(blend.to_bits(), 24.0f32.to_bits());
    assert_eq!(scale.to_bits(), 4.0f32.to_bits());
}

#[test]
fn blend_factors_nan_ratio_divides_scale() {
    // An unordered ratio takes the divide path (NaN never exceeds 1.0).
    let mut scale = 5.0;
    let mut clock = ScriptClock {
        a: 1,
        b: 1,
        calls: Vec::new(),
    };
    let zero = TimeBases {
        num_narrow: 0,
        num_wide: 0,
        den_narrow: 0,
        den_wide: 0,
    };
    let (_, _, blend) =
        blend_factors(&mut clock, &mut scale, &zero, &mut || false);
    assert_eq!(blend.to_bits(), 5.0f32.to_bits());
    assert!(scale.is_nan());
}

fn ctl() -> NotifyCtl {
    NotifyCtl {
        inner: InnerHandle::new(0x5000).unwrap(),
    }
}

struct ScriptHub {
    answers: Vec<u32>,
    cleared: Vec<u32>,
    flag: u8,
}

impl lf_files_memory::replay_bar::NotifyHub for ScriptHub {
    fn lookup(&mut self) -> Option<lf_files_memory::replay_bar::NotifierHandle> {
        let addr = self.answers.remove(0);
        lf_files_memory::replay_bar::NotifierHandle::new(addr)
    }
    fn clear_suppress(&mut self, id: lf_files_memory::replay_bar::NotifierHandle) {
        self.cleared.push(id.get());
        self.flag &= 0xfe;
    }
}

#[test]
fn notify_probe_below_bound_returns_probe() {
    let mut refreshed = false;
    let mut hub = ScriptHub {
        answers: vec![1, 2],
        cleared: Vec::new(),
        flag: 1,
    };
    let got = ctl().maybe_notify(10, 0, &mut || 9, &mut || refreshed = true, &mut hub);
    assert_eq!(got, RefreshOutcome::BelowBound(9));
    assert!(!refreshed);
    assert!(hub.cleared.is_empty());
}

#[test]
fn notify_suppressed_set_matches() {
    for state in [2u32, 7, 8, 11, 12, 13, 14, 15, 16, 17] {
        let mut refreshed = false;
        let mut hub = ScriptHub {
            answers: vec![1, 2],
            cleared: Vec::new(),
            flag: 1,
        };
        let got = ctl().maybe_notify(10, state, &mut || 10, &mut || refreshed = true, &mut hub);
        assert_eq!(got, RefreshOutcome::Suppressed(state), "state={state}");
        assert!(!refreshed);
    }
    // Neighbours of the set refresh normally.
    for state in [0u32, 1, 3, 9, 10, 18, 19, 100, u32::MAX] {
        let mut refreshed = false;
        let mut hub = ScriptHub {
            answers: vec![0x7000, 0x7000],
            cleared: Vec::new(),
            flag: 1,
        };
        let got = ctl().maybe_notify(10, state, &mut || 10, &mut || refreshed = true, &mut hub);
        assert!(refreshed, "state={state}");
        assert_eq!(
            got,
            RefreshOutcome::Refreshed(
                lf_files_memory::replay_bar::NotifierHandle::new(0x7000).unwrap()
            )
        );
        assert_eq!(hub.cleared, [0x7000]);
        assert_eq!(hub.flag, 0);
    }
}

#[test]
fn notify_null_first_fetch_stops() {
    let mut refreshed = false;
    let mut hub = ScriptHub {
        answers: vec![0, 0x7000],
        cleared: Vec::new(),
        flag: 1,
    };
    let got = ctl().maybe_notify(10, 0, &mut || 10, &mut || refreshed = true, &mut hub);
    assert_eq!(got, RefreshOutcome::NoNotifier);
    assert!(refreshed);
    assert!(hub.cleared.is_empty());
    assert_eq!(hub.flag, 1);
}

#[test]
#[should_panic(expected = "null")]
fn notify_null_second_fetch_panics() {
    let mut hub = ScriptHub {
        answers: vec![0x7000, 0],
        cleared: Vec::new(),
        flag: 1,
    };
    let _ = ctl().maybe_notify(10, 0, &mut || 10, &mut || {}, &mut hub);
}
