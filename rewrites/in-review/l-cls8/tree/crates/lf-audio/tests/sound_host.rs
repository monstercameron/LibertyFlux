//! Host tests for the lifted audio effects: edge cases a reader would ask about.
//!
//! These run on the 64-bit host with a scripted world fake. The
//! differential proof against the verified rewrites lives in the
//! `lf-sounddiff` test crate (32-bit target only).

use lf_audio::sound::compressor::{CompressorEffect, CompressorWorld};
use lf_audio::sound::effect::{Effect, EffectWorld};
use lf_audio::sound::registry;
use lf_audio::sound::reverb::{ReverbEffect, ReverbWorld};
use lf_audio::sound::{ListenerTag, NextTag, ReverbSubTag, SubTag, VoiceTag};
use lf_core::Handle32;

#[derive(Default)]
struct Fake {
    log: Vec<String>,
    answers: Vec<u32>,
}

impl Fake {
    fn answer(&mut self, v: u32) {
        self.answers.push(v);
    }
    fn pop(&mut self) -> u32 {
        if self.answers.is_empty() { 0 } else { self.answers.remove(0) }
    }
}

impl EffectWorld for Fake {
    fn release_voice(&mut self, voice: Option<Handle32<VoiceTag>>) {
        self.log.push(format!("release {voice:?}"));
    }
    fn lookup_voice(&mut self, tag: u32, param_plus_one: u32) -> Option<Handle32<VoiceTag>> {
        self.log.push(format!("lookup {tag:#x} {param_plus_one:#x}"));
        Handle32::new(self.pop())
    }
    fn refresh_entry(&mut self, slot: u32) {
        self.log.push(format!("refresh {slot:#x}"));
    }
    fn poll_voice(&mut self, voice: Option<Handle32<VoiceTag>>) {
        self.log.push(format!("poll {voice:?}"));
    }
}

impl CompressorWorld for Fake {
    fn base_rotate(&mut self) {
        self.log.push("base".to_string());
    }
    fn notify_listener(&mut self, listener: Option<Handle32<ListenerTag>>) -> u32 {
        self.log.push(format!("notify {listener:?}"));
        self.pop()
    }
    fn set_base(&mut self, a1: u32, a2: u32) -> u32 {
        self.log.push(format!("set {a1:#x} {a2:#x}"));
        self.pop()
    }
    fn pre_poll(&mut self) {
        self.log.push("pre".to_string());
    }
    fn sub_poll(&mut self, sub: Option<Handle32<SubTag>>, slot_words: u32) {
        self.log.push(format!("sub {sub:?} {slot_words:#x}"));
    }
    fn post_poll(&mut self) -> u32 {
        self.log.push("post".to_string());
        self.pop()
    }
}

impl ReverbWorld for Fake {
    fn base_advance(&mut self) {
        self.log.push("base".to_string());
    }
    fn advance_next(&mut self, next: Option<Handle32<NextTag>>) -> u32 {
        self.log.push(format!("next {next:?}"));
        self.pop()
    }
    fn base_init(&mut self, a: u32, b: u32) -> u32 {
        self.log.push(format!("init {a:#x} {b:#x}"));
        self.pop()
    }
    fn refresh_hook(&mut self) {
        self.log.push("hook".to_string());
    }
    fn refresh_direct(&mut self) -> u32 {
        self.log.push("direct".to_string());
        self.pop()
    }
    fn pre_poll(&mut self) {
        self.log.push("pre".to_string());
    }
    fn sub_poll(&mut self, sub: Option<Handle32<ReverbSubTag>>, slot_words: u32) {
        self.log.push(format!("sub {sub:?} {slot_words:#x}"));
    }
    fn post_poll(&mut self) -> u32 {
        self.log.push("post".to_string());
        self.pop()
    }
}

fn cookie<T>(v: u32) -> Option<Handle32<T>> {
    Handle32::new(v)
}

#[test]
fn registry_counts_are_pinned() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!((proven, lifted, missing), (13, 0, 3));
    assert_eq!(registry::ROWS.len(), 16);
}

#[test]
fn fresh_effect_is_zeroed_with_bound_one() {
    let v = Effect::new();
    assert_eq!(v.limit, 1);
    assert_eq!(v.enabled, 0);
    assert_eq!(v.slots, [0; 15]);
    assert_eq!(v.voice, None);
    assert_eq!(Effect::default(), v);
}

#[test]
fn empty_rotation_answers_quotient() {
    // No iteration runs: the answer is (count + 1) / 3.
    for (count, quotient) in [(0u32, 0u32), (2, 1), (5, 2), (u32::MAX, 0)] {
        let mut v = Effect::new();
        v.count = count;
        v.limit = 0;
        assert_eq!(v.rotate_slots(), quotient, "count {count:#x}");
        assert_eq!(v.slots, [0; 15]);
    }
}

#[test]
fn rotation_wraps_count() {
    // count + 1 wraps: the destination row still computes mod 3.
    let mut v = Effect::new();
    v.count = u32::MAX;
    v.limit = 0;
    assert_eq!(v.rotate_slots(), 0);
}

#[test]
fn rotation_copies_row_to_next() {
    let mut v = Effect::new();
    for (i, s) in v.slots.iter_mut().enumerate() {
        *s = 100 + u32::try_from(i).unwrap();
    }
    v.count = 0;
    v.limit = 5;
    let last = v.rotate_slots();
    assert_eq!(last, 104);
    assert_eq!(&v.slots[5..10], &[100, 101, 102, 103, 104]);
    assert_eq!(&v.slots[0..5], &[100, 101, 102, 103, 104]);
}

#[test]
fn rotation_bound_reaches_trailing_word() {
    // Limit 11 with count 0 writes the trailing word (index 15) from
    // slot 10, and the re-read bound (low byte pinned to 0 here) ends
    // the loop.
    let mut v = Effect::new();
    v.count = 0;
    v.limit = 11;
    // The word flows down the overlapping chain (slot 0 -> 5 -> 10 ->
    // trailing word), not straight from slot 10, which the copy
    // overwrites first.
    v.slots[0] = 0xAABB_CC00;
    let last = v.rotate_slots();
    assert_eq!(last, 0xAABB_CC00);
    assert_eq!(v.limit, 0);
    assert_eq!(v.enabled, 0xCC);
    assert_eq!(v.tail, [0xBB, 0xAA]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn rotation_panics_past_modelled_window() {
    let mut v = Effect::new();
    v.count = u32::MAX;
    v.limit = 1;
    let _ = v.rotate_slots();
}

#[test]
fn attach_null_block_records_and_fails() {
    let mut v = Effect::new();
    v.info = cookie(0x1234);
    let mut w = Fake::default();
    assert!(!v.attach(&mut w, None, 0xFFFF_FFFF, 7));
    assert_eq!(v.info, None);
    assert!(w.log.is_empty());
    assert_eq!(v.count, 0);
}

#[test]
fn attach_all_ones_tag_skips_lookup() {
    let mut v = Effect::new();
    let mut w = Fake::default();
    assert!(v.attach(&mut w, cookie(0x2000), 0xFFFF_FFFF, 9));
    assert_eq!(v.voice, None);
    assert_eq!(v.param, 9);
    assert_eq!(v.count, 1);
    assert_eq!(v.index, 0);
    assert_eq!(v.slots, [0x3F80_0000; 15]);
    assert!(w.log.is_empty());
}

#[test]
fn attach_lookup_passes_param_plus_one_wrapping() {
    let mut v = Effect::new();
    let mut w = Fake::default();
    w.answer(0x3000);
    assert!(v.attach(&mut w, cookie(0x2000), 0x10, u32::MAX));
    assert_eq!(v.voice, cookie(0x3000));
    assert_eq!(w.log, vec!["lookup 0x10 0x0".to_string()]);
}

#[test]
fn poll_gate_matrix() {
    // (ready, enabled, voice) -> (refresh?, poll?)
    for ready in [0u32, 1] {
        for enabled in [0u8, 1] {
            for voice in [None, cookie(0x4000)] {
                let mut v = Effect::new();
                v.ready = ready;
                v.enabled = enabled;
                v.voice = voice;
                v.index = 2;
                let mut w = Fake::default();
                v.poll(&mut w);
                let mut expect = Vec::new();
                if ready != 0 && enabled != 0 {
                    expect.push(format!("refresh {:#x}", 2 * 5 + 13));
                }
                if voice.is_some() {
                    expect.push(format!("poll {voice:?}"));
                }
                assert_eq!(w.log, expect, "ready {ready} enabled {enabled}");
            }
        }
    }
}

#[test]
fn reset_keeps_voice_cookie() {
    let mut v = Effect::new();
    v.voice = cookie(0x5000);
    let mut w = Fake::default();
    v.reset(&mut w);
    assert_eq!(v.voice, cookie(0x5000));
    assert_eq!(w.log.len(), 1);
    let mut v = Effect::new();
    let mut w = Fake::default();
    v.reset(&mut w);
    assert!(w.log.is_empty());
}

#[test]
fn compressor_set_normalises_low_byte() {
    for (ans, expect) in [
        (0u32, 0u32),
        (1, 1),
        (2, 1),
        (0xFF, 1),
        (0x100, 0x100),
        (0x101, 0x101),
        (0xFFFF_FF00, 0xFFFF_FF00),
        (0xFFFF_FFFF, 0xFFFF_FF01),
        (0xDEAD_BEEF, 0xDEAD_BE01),
    ] {
        let mut v = CompressorEffect {
            listener: None,
            poll_index: 0,
            index: 0,
            sub: None,
            params: [[0; 9]; 3],
        };
        let mut w = Fake::default();
        w.answer(ans);
        assert_eq!(v.set_param(&mut w, 3, 4), expect, "ans {ans:#x}");
    }
}

#[test]
fn compressor_slot_selects_row() {
    let v = CompressorEffect {
        listener: None,
        poll_index: 0,
        index: 1,
        sub: None,
        params: [[1; 9], [2; 9], [3; 9]],
    };
    assert_eq!(v.slot(), &[2; 9]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn compressor_slot_panics_past_last_row() {
    let v = CompressorEffect {
        listener: None,
        poll_index: 0,
        index: 3,
        sub: None,
        params: [[0; 9]; 3],
    };
    let _ = v.slot();
}

#[test]
fn compressor_rotate_cycles_rows() {
    let mut v = CompressorEffect {
        listener: None,
        poll_index: 0,
        index: 0,
        sub: None,
        params: [[10; 9], [20; 9], [30; 9]],
    };
    let mut w = Fake::default();
    assert_eq!(v.rotate_params(&mut w), 10);
    assert_eq!(v.index, 1);
    assert_eq!(v.params[1], [10; 9]);
    assert_eq!(v.rotate_params(&mut w), 10);
    assert_eq!(v.index, 2);
    assert_eq!(v.params[2], [10; 9]);
    assert_eq!(w.log, vec!["base".to_string(), "base".to_string()]);
}

#[test]
fn compressor_rotate_notifies_listener() {
    let mut v = CompressorEffect {
        listener: cookie(0x6000),
        poll_index: 0,
        index: 2,
        sub: None,
        params: [[1; 9], [2; 9], [7; 9]],
    };
    let mut w = Fake::default();
    w.answer(0x99);
    assert_eq!(v.rotate_params(&mut w), 0x99);
    assert_eq!(v.index, 0);
    assert_eq!(v.params[0], [7; 9]);
    assert_eq!(w.log.len(), 2);
}

#[test]
fn compressor_poll_orders_calls() {
    let mut v = CompressorEffect {
        listener: None,
        poll_index: 2,
        index: 0,
        sub: cookie(0x7000),
        params: [[0; 9]; 3],
    };
    let mut w = Fake::default();
    w.answer(0x42);
    assert_eq!(v.poll(&mut w), 0x42);
    assert_eq!(w.log[0], "pre");
    assert_eq!(w.log[1], format!("sub {:?} {:#x}", cookie::<SubTag>(0x7000), 2 * 9 + 30));
    assert_eq!(w.log[2], "post");
}

fn fresh_reverb() -> ReverbEffect {
    ReverbEffect {
        info: cookie(0x1000),
        next: None,
        poll_index: 0,
        step: 0,
        chans: [[0; 5]; 3],
        staging: [0; 4],
        hold: 1,
        sub: None,
    }
}

#[test]
fn reverb_advance_stores_past_floor() {
    let mut v = fresh_reverb();
    // Stored values below the floor refresh; the one above keeps.
    v.chans[0] = [
        0.5f32.to_bits(),
        2.0f32.to_bits(),
        f32::NEG_INFINITY.to_bits(),
        0.0f32.to_bits(),
        0xAAAA,
    ];
    let mut w = Fake::default();
    let preset = [10, 20, 30, 40];
    let got = v.advance(&mut w, &preset, 1.0);
    assert_eq!(v.chans[0][0..3], [10, 2.0f32.to_bits(), 30]);
    assert_eq!(v.chans[0][3], 40);
    assert_eq!(v.chans[0][4], 0xAAAA);
    // The shuffle runs before the refresh, so the new row holds the
    // entry values, not the refreshed ones.
    assert_eq!(
        v.chans[1],
        [
            0.5f32.to_bits(),
            2.0f32.to_bits(),
            f32::NEG_INFINITY.to_bits(),
            0.0f32.to_bits(),
            0xAAAA
        ]
    );
    assert_eq!(v.step, 1);
    assert_eq!(got, 40);
    assert_eq!(w.log, vec!["base".to_string()]);
}

#[test]
fn reverb_advance_tie_does_not_store() {
    // Strict comparison: equal to the floor is not past it.
    let mut v = fresh_reverb();
    v.chans[1] = [1.0f32.to_bits(); 5];
    v.step = 1;
    let mut w = Fake::default();
    let got = v.advance(&mut w, &[9, 9, 9, 9], 1.0);
    assert_eq!(v.chans[1][0..4], [1.0f32.to_bits(); 4]);
    // Nothing stored: the answer is the step counter.
    assert_eq!(got, 1);
}

#[test]
fn reverb_advance_hold_clear_forces_store() {
    let mut v = fresh_reverb();
    v.chans[2] = [100.0f32.to_bits(); 5];
    v.step = 2;
    v.hold = 0;
    let mut w = Fake::default();
    let got = v.advance(&mut w, &[1, 2, 3, 4], 0.0);
    assert_eq!(v.chans[2][0..4], [1, 2, 3, 4]);
    assert_eq!(got, 4);
    assert_eq!(v.step, 0);
}

#[test]
fn reverb_advance_nan_floor_stores_nothing() {
    // NaN comparisons are false: with the hold flag set, nothing stores.
    let mut v = fresh_reverb();
    v.chans[0] = [0x3F80_0000; 5];
    let mut w = Fake::default();
    let got = v.advance(&mut w, &[1, 2, 3, 4], f32::NAN);
    assert_eq!(v.chans[0][0..4], [0x3F80_0000; 4]);
    assert_eq!(got, 0);
}

#[test]
fn reverb_advance_hands_to_next() {
    let mut v = fresh_reverb();
    v.next = cookie(0x8000);
    let mut w = Fake::default();
    w.answer(0x77);
    assert_eq!(v.advance(&mut w, &[0; 4], 0.0), 0x77);
    assert_eq!(w.log.len(), 2);
    assert_eq!(w.log[0], "base");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn reverb_advance_panics_past_last_row() {
    let mut v = fresh_reverb();
    v.step = 3;
    let mut w = Fake::default();
    let _ = v.advance(&mut w, &[0; 4], 0.0);
}

#[test]
fn reverb_channel_selects_row() {
    let mut v = fresh_reverb();
    v.chans = [[1; 5], [2; 5], [3; 5]];
    v.step = 2;
    assert_eq!(v.channel(), &[3; 5]);
}

#[test]
fn reverb_init_gate_and_fanout() {
    // Zero low byte: the answer returns unchanged, nothing else happens.
    let mut v = fresh_reverb();
    let mut w = Fake::default();
    w.answer(0x1234_5600);
    assert_eq!(v.init(&mut w, &[1, 2, 3, 4], 5, 6), 0x1234_5600);
    assert_eq!(v.hold, 1);
    assert_eq!(w.log.len(), 1);
    // Nonzero low byte: fan-out, hooks, forced low byte.
    let mut w = Fake::default();
    w.answer(0xFF);
    w.answer(0x1111);
    w.answer(0x2222);
    w.answer(0xABCD_00EF);
    v.chans = [[0xEE; 5]; 3];
    assert_eq!(v.init(&mut w, &[1, 2, 3, 4], 5, 6), 0xABCD_0001);
    assert_eq!(v.hold, 0);
    for r in 0..3 {
        assert_eq!(v.chans[r][0..4], [1, 2, 3, 4]);
        assert_eq!(v.chans[r][4], 0xEE);
    }
    assert_eq!(v.staging, [1, 2, 3, 4]);
    assert_eq!(
        w.log,
        vec![
            "init 0x5 0x6".to_string(),
            "hook".to_string(),
            "direct".to_string(),
            "hook".to_string(),
            "direct".to_string(),
            "hook".to_string(),
            "direct".to_string(),
        ]
    );
}

#[test]
fn reverb_poll_orders_calls() {
    let mut v = fresh_reverb();
    v.poll_index = 1;
    v.sub = cookie(0x9000);
    let mut w = Fake::default();
    w.answer(0x51);
    assert_eq!(v.poll(&mut w), 0x51);
    assert_eq!(
        w.log[1],
        format!("sub {:?} {:#x}", cookie::<ReverbSubTag>(0x9000), 1 * 5 + 29)
    );
}
