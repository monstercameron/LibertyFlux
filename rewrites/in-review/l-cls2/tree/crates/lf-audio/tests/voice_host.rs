//! Host tests for the lifted audio voices: edge cases and truth tables.
//!
//! These run on the 64-bit host (no rewrites here; the differential proof
//! against the verified 32-bit rewrites is the `lf-voicediff` crate).
//! They pin the domains the registry narrows (empty selections, zero
//! divisors, out-of-table entries panic) and cross-check the shared
//! arithmetic against independent computations.

use lf_audio::voice::dsound::{DSoundVoice, DSoundWorld};
use lf_audio::voice::dsound_adpcm::{AdpcmVoice, AdpcmWorld};
use lf_audio::voice::pc_adpcm::{PcAdpcmVoice, PcWorld};
use lf_audio::voice::registry::{State, counts, ROWS};
use lf_audio::voice::shared;
use lf_audio::voice::soft::{SoftVoice, SoftWorld};
use lf_audio::voice::{AuxTag, BufferTag, ChildTag, CodecTag, DeviceTag, MixerTag};
use lf_core::Handle32;
use std::collections::{HashMap, VecDeque};

/// Scriptable world fake for host tests (no runtime underneath).
#[derive(Default)]
struct Canned {
    answers: HashMap<&'static str, VecDeque<u32>>,
    log: Vec<(String, Vec<u32>)>,
}

impl Canned {
    fn new() -> Self {
        Self::default()
    }

    fn answer(&mut self, name: &'static str, values: Vec<u32>) -> &mut Self {
        self.answers.insert(name, values.into_iter().collect());
        self
    }

    fn call(&mut self, name: &'static str, args: Vec<u32>) -> u32 {
        self.log.push((name.to_string(), args));
        self.answers.get_mut(name).and_then(VecDeque::pop_front).unwrap_or(0)
    }

    fn unit(&mut self, name: &'static str, args: Vec<u32>) {
        self.log.push((name.to_string(), args));
    }
}

fn words<T>(c: Option<Handle32<T>>) -> u32 {
    Handle32::raw_or_zero(c)
}

impl SoftWorld for Canned {
    fn child_query(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("q", vec![words(c)])
    }
    fn child_stop(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("stop", vec![words(c)])
    }
    fn child_resume(&mut self, c: Option<Handle32<ChildTag>>, m: u32) {
        self.unit("resume", vec![words(c), m]);
    }
    fn child_start(&mut self, c: Option<Handle32<ChildTag>>, r: u32) {
        self.unit("start", vec![words(c), r]);
    }
    fn child_poll(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("poll", vec![words(c)])
    }
    fn child_shutdown(&mut self, c: Option<Handle32<ChildTag>>) {
        self.unit("shut", vec![words(c)]);
    }
    fn own_start(&mut self, m: u32) {
        self.unit("own", vec![m]);
    }
    fn restart(&mut self) -> u32 {
        self.call("restart", vec![])
    }
    fn convert_rate(&mut self, w: u32, b: u32) -> u32 {
        self.call("rate", vec![w, b])
    }
    fn report_level(&mut self, l: u32) {
        self.unit("level", vec![l]);
    }
    fn free_buffer(&mut self, b: Option<Handle32<BufferTag>>) {
        self.unit("free", vec![words(b)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("base", vec![])
    }
    fn state_gate(&mut self) -> u32 {
        self.call("gate", vec![])
    }
    fn mode_fallback(&mut self) {
        self.unit("fb", vec![]);
    }
    fn convert_position(&mut self, s: u32, b: u32) -> u32 {
        self.call("pos", vec![s, b])
    }
    fn refine_count(&mut self, c: u32, r: u32) -> u32 {
        self.call("refine", vec![c, r])
    }
    fn consume_samples(&mut self, a: Option<Handle32<AuxTag>>, c: u32) -> u32 {
        self.call("consume", vec![words(a), c])
    }
}

impl PcWorld for Canned {
    fn child_query(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("q", vec![words(c)])
    }
    fn child_stop(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("stop", vec![words(c)])
    }
    fn child_resume(&mut self, c: Option<Handle32<ChildTag>>, m: u32) {
        self.unit("resume", vec![words(c), m]);
    }
    fn child_start(&mut self, c: Option<Handle32<ChildTag>>, r: u32) {
        self.unit("start", vec![words(c), r]);
    }
    fn child_poll(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("poll", vec![words(c)])
    }
    fn child_measure(&mut self, c: Option<Handle32<ChildTag>>) -> u32 {
        self.call("measure", vec![words(c)])
    }
    fn child_shutdown(&mut self, c: Option<Handle32<ChildTag>>) {
        self.unit("shut", vec![words(c)]);
    }
    fn own_start(&mut self, m: u32) {
        self.unit("own", vec![m]);
    }
    fn restart(&mut self) -> u32 {
        self.call("restart", vec![])
    }
    fn convert_rate(&mut self, w: u32, b: u32) -> u32 {
        self.call("rate", vec![w, b])
    }
    fn report_level(&mut self, l: u32) {
        self.unit("level", vec![l]);
    }
    fn free_buffer(&mut self, b: Option<Handle32<BufferTag>>) {
        self.unit("free", vec![words(b)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("base", vec![])
    }
    fn state_gate(&mut self) -> u32 {
        self.call("gate", vec![])
    }
    fn mode_fallback(&mut self) {
        self.unit("fb", vec![]);
    }
    fn convert_position(&mut self, s: u32, b: u32) -> u32 {
        self.call("pos", vec![s, b])
    }
    fn resolve_codec(&mut self, c: Option<Handle32<CodecTag>>, a: u32) -> u32 {
        self.call("codec", vec![words(c), a])
    }
    fn mixer_cursor(&mut self, m: Option<Handle32<MixerTag>>) -> u32 {
        self.call("mix", vec![words(m)])
    }
}

impl DSoundWorld for Canned {
    fn state_gate(&mut self) -> u32 {
        self.call("gate", vec![])
    }
    fn play_cursor(&mut self, d: Option<Handle32<DeviceTag>>) -> u32 {
        self.call("cursor", vec![words(d)])
    }
    fn convert_position(&mut self, s: u32, b: u32) -> u32 {
        self.call("pos", vec![s, b])
    }
    fn resolve_voice(&mut self, c: u32, r: u32) -> u32 {
        self.call("resolve", vec![c, r])
    }
    fn consume_stream(&mut self, c: u32) -> u32 {
        self.call("consume", vec![c])
    }
    fn resolve_length(&mut self, p: u32, r: u32) -> u32 {
        self.call("length", vec![p, r])
    }
    fn notify_start(&mut self, m: u32) {
        self.unit("notify", vec![m]);
    }
    fn channel_set_length(&mut self, d: Option<Handle32<DeviceTag>>, l: u32) {
        self.unit("channel", vec![words(d), l]);
    }
    fn finish_start(&mut self, l: u32) -> u32 {
        self.call("finish", vec![l])
    }
    fn release_device(&mut self, d: Option<Handle32<DeviceTag>>) {
        self.unit("release", vec![words(d)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("base", vec![])
    }
    fn device_resume(&mut self, d: Option<Handle32<DeviceTag>>, l: bool) {
        self.unit("resume", vec![words(d), u32::from(l)]);
    }
    fn device_seek(&mut self, d: Option<Handle32<DeviceTag>>, p: u32) {
        self.unit("seek", vec![words(d), p]);
    }
}

impl AdpcmWorld for Canned {
    fn state_gate(&mut self) -> u32 {
        self.call("gate", vec![])
    }
    fn play_cursor(&mut self, d: Option<Handle32<DeviceTag>>) -> u32 {
        self.call("cursor", vec![words(d)])
    }
    fn convert_position(&mut self, s: u32, b: u32) -> u32 {
        self.call("pos", vec![s, b])
    }
    fn resolve_codec(&mut self, c: Option<Handle32<CodecTag>>, a: u32) -> u32 {
        self.call("codec", vec![words(c), a])
    }
    fn convert_rate(&mut self, w: u32, b: u32) -> u32 {
        self.call("rate", vec![w, b])
    }
    fn resolve_length(&mut self, p: u32, r: u32) -> u32 {
        self.call("length", vec![p, r])
    }
    fn refill_region(&mut self, m: u32) {
        self.unit("refill", vec![m]);
    }
    fn channel_set_cursor(&mut self, d: Option<Handle32<DeviceTag>>, c: u32) {
        self.unit("channel", vec![words(d), c]);
    }
    fn forward_gain(&mut self, g: u32) {
        self.unit("gain", vec![g]);
    }
}

fn soft_voice() -> SoftVoice {
    SoftVoice {
        flags: 0,
        rate: 44100,
        aux: Handle32::new(0x1000),
        level: 0,
        params_status: 0,
        child: Handle32::new(0x2000),
        buffer: Handle32::new(0x3000),
        lanes: vec![Default::default(), Default::default()],
        cursor: 0,
        divisor: 2,
        tuning: 0,
        freq: 0,
        mode_cmp: 0,
        mode_count: 0,
        mode_byte: 0,
    }
}

fn pc_voice() -> PcAdpcmVoice {
    PcAdpcmVoice {
        flags: 0,
        rate: 44100,
        codec: Handle32::new(0x1000),
        level: 0,
        params_status: 0,
        child: Handle32::new(0x2000),
        buffer: Handle32::new(0x3000),
        lanes: vec![Default::default(), Default::default()],
        cursor: 0,
        divisor: 2,
        tuning: 0,
        freq: 0,
        mode_cmp: 0,
        mode_count: 0,
        mode_byte: 0,
        mixer_word: 0,
        mixer: Handle32::new(0x4000),
        codec_table: vec![0u32; 64],
        predictor: vec![0u8; 64],
        out_word: 0,
        out_byte: 0,
    }
}

fn ds_voice() -> DSoundVoice {
    DSoundVoice {
        flags: 0,
        rate: 44100,
        restart_pos: 0,
        params_status: 0,
        device_a: Handle32::new(0x1000),
        device_b: Handle32::new(0x2000),
        cached_voice: 0,
        freq: 0,
        base_len: 0,
        limit: 0,
        combine: 0,
        level: 0,
        cursor: 0,
        divisor: 2,
        lanes: vec![Default::default(), Default::default()],
    }
}

fn ad_voice() -> AdpcmVoice {
    AdpcmVoice {
        flags: 0,
        rate: 44100,
        codec: Handle32::new(0x1000),
        device: Handle32::new(0x2000),
        level: 0,
        base_len: 0,
        limit: 0,
        combine: 0,
        stored_rate: 0,
        freq: 0,
        cursor: 0,
        divisor: 2,
        lanes: vec![Default::default(), Default::default()],
        codec_table: vec![0u32; 64],
        predictor: vec![0u8; 64],
        out_word: 0,
        out_byte: 0,
    }
}

#[test]
fn registry_counts_pin_proof_state() {
    let (proven, lifted, missing) = counts();
    assert_eq!(proven, 28, "proven methods");
    assert_eq!(lifted, 0, "lifted-but-unproven methods");
    assert_eq!(missing, 9, "missing methods");
    assert_eq!(ROWS.len(), 37, "verified rows");
    assert!(
        ROWS.iter()
            .filter(|r| r.state == State::Missing)
            .all(|r| !r.narrows.is_empty()),
        "every missing row states why"
    );
}

#[test]
fn stop_clears_exactly_bits_0_and_3() {
    for flags in 0..=255u8 {
        let mut v = soft_voice();
        v.flags = flags;
        let mut w = Canned::new();
        w.answer("stop", vec![0x1234]);
        assert_eq!(v.stop(&mut w), 0x1234);
        assert_eq!(v.flags, flags & !0x09, "flags {flags:#x}");
        let mut p = pc_voice();
        p.flags = flags;
        let mut w = Canned::new();
        w.answer("stop", vec![7]);
        assert_eq!(p.stop(&mut w), 7);
        assert_eq!(p.flags, flags & !0x09, "pc flags {flags:#x}");
    }
}

#[test]
fn resume_mode_is_bit1_or_bit4() {
    for flags in 0..=255u8 {
        let mode = shared::resume_mode(flags);
        assert_eq!(mode, u32::from(flags & 0x12 != 0), "flags {flags:#x}");
    }
    // And resume passes exactly that to the child.
    for flags in [0x08u8, 0x0A, 0x18, 0x1A, 0xFF] {
        let mut v = soft_voice();
        v.flags = flags;
        let mut w = Canned::new();
        v.resume(&mut w);
        assert_eq!(v.flags, flags & !0x08);
        assert!(w.log.iter().any(|(n, a)| n == "resume" && a[1] == u32::from(flags & 0x12 != 0)));
    }
    // Without the pending bit, nothing happens and nobody is called.
    let mut v = soft_voice();
    v.flags = 0xF7;
    let mut w = Canned::new();
    v.resume(&mut w);
    assert_eq!(v.flags, 0xF7);
    assert!(w.log.is_empty());
}

#[test]
fn start_flag_matrix() {
    // (level, expect_resume_bit, expect_stop_bits)
    let cases = [
        (0u32, false),
        (0x8000_0000, false), // negative zero is zero
        (1, true),
        (0x7FC0_0000, true), // quiet NaN counts as nonzero
        (0x7F80_0001, true), // signalling NaN counts as nonzero
        (0xFF80_0001, true),
        (0x7F80_0000, true), // infinities are nonzero
        (0x0000_0001, true), // smallest denormal is nonzero
    ];
    for (level, nonzero) in cases {
        for synth in [false, true] {
            let mut v = soft_voice();
            v.flags = if synth { 0x10 } else { 0 };
            v.level = level;
            let mut w = Canned::new();
            w.answer("rate", vec![100]);
            v.start(&mut w, 50);
            if nonzero {
                assert_eq!(v.flags & 0x08, 0x08, "level {level:#x}");
            } else {
                assert_eq!(v.flags & 0x41, 0x41, "level {level:#x}");
            }
        }
    }
}

#[test]
fn teardown_pc_always_frees_soft_gates_on_synth() {
    for flags in [0x00u8, 0xEF, 0x10, 0xFF] {
        let mut p = pc_voice();
        p.flags = flags;
        let mut w = Canned::new();
        p.teardown(&mut w);
        assert!(w.log.iter().any(|(n, _)| n == "free"), "pc flags {flags:#x}");
        assert_eq!(p.buffer, None);
        let mut v = soft_voice();
        v.flags = flags;
        let mut w = Canned::new();
        v.teardown(&mut w);
        assert_eq!(
            w.log.iter().any(|(n, _)| n == "free"),
            flags & 0x10 != 0,
            "soft flags {flags:#x}"
        );
    }
    // A null child is never shut down but the base still runs.
    let mut v = soft_voice();
    v.child = None;
    v.flags = 0x10;
    let mut w = Canned::new();
    w.answer("base", vec![9]);
    assert_eq!(v.teardown(&mut w), 9);
    assert!(!w.log.iter().any(|(n, _)| n == "shut"));
}

#[test]
fn dsound_resume_gate_matrix() {
    // Active only with bit 3 set and bit 0 clear.
    for flags in 0..=255u8 {
        let mut v = ds_voice();
        v.flags = flags;
        v.restart_pos = 0x5555_0001;
        let mut w = Canned::new();
        v.resume(&mut w);
        let active = flags & 8 != 0 && flags & 1 == 0;
        assert_eq!(!w.log.is_empty(), active, "flags {flags:#x}");
        if active {
            assert_eq!(v.flags, flags & !8);
            let looping = flags & 0x12 != 0;
            assert!(w.log.iter().any(|(n, a)| n == "resume" && a[1] == u32::from(looping)));
            assert!(w.log.iter().any(|(n, a)| n == "seek" && a[1] == 0x5555_0001));
        } else {
            assert_eq!(v.flags, flags);
        }
    }
}

#[test]
fn dsound_stopping_arms() {
    // Each arm alone fires; none fires on a clean voice.
    let mut v = ds_voice();
    assert!(!v.is_stopping());
    v.flags = 0x01;
    assert!(v.is_stopping());
    v.flags = 0x08;
    assert!(v.is_stopping());
    v.flags = 0x00;
    v.restart_pos = 0x1000_0001;
    assert!(v.is_stopping());
    v.restart_pos = 0x1000_0000;
    assert!(!v.is_stopping());
    v.params_status = 0x40;
    assert!(v.is_stopping());
    v.params_status = 0xBF;
    assert!(!v.is_stopping());
}

#[test]
fn ring_wrap_and_quotient() {
    // Divisor 1: every step wraps to lane 0 and answers the count.
    let mut v = soft_voice();
    v.divisor = 1;
    v.cursor = 0;
    let mut w = Canned::new();
    assert_eq!(v.refresh_lane(&mut w, &[1u32; 14], 0, 0), 1);
    assert_eq!(v.cursor, 0);
    // Divisor 2: step 0->1 answers 0, step 1->0 answers 1.
    let mut v = soft_voice();
    v.cursor = 0;
    assert_eq!(v.refresh_lane(&mut w, &[2u32; 14], 5, 0), 0);
    assert_eq!(v.cursor, 1);
    assert_eq!(v.refresh_lane(&mut w, &[3u32; 14], 7, 0), 1);
    assert_eq!(v.cursor, 0);
    assert_eq!(v.lanes[0].rem, 5u32.wrapping_sub(v.lanes[0].pos));
    // The copied words land verbatim.
    assert_eq!(v.lanes[1].params, [3u32; 14]);
}

#[test]
#[should_panic]
fn ring_zero_divisor_panics() {
    let mut v = soft_voice();
    v.divisor = 0;
    v.refresh_lane(&mut Canned::new(), &[0u32; 14], 0, 0);
}

#[test]
#[should_panic]
fn ring_cursor_past_lanes_panics() {
    let v = soft_voice();
    let mut v = v;
    v.cursor = 7;
    v.lane_rest_quiet();
}

#[test]
#[should_panic]
fn queue_block_codec_entry_past_table_panics() {
    let mut v = pc_voice();
    let mut w = Canned::new();
    w.answer("codec", vec![0x1000_0000]);
    w.answer("rate", vec![0]);
    v.flags = 0x10;
    v.queue_block(&mut w, &[0u32; 14], 0, 1);
}

#[test]
fn milli_floor_known_values() {
    assert_eq!(shared::milli_floor_doubled(1000, 1), 2);
    assert_eq!(shared::milli_floor_doubled(1500, 1), 2); // floor(1.5) = 1
    assert_eq!(shared::milli_floor_doubled(1999, 1), 2);
    assert_eq!(shared::milli_floor_doubled(2000, 1), 4);
    assert_eq!(shared::milli_floor_doubled(1, 0), 0);
    assert_eq!(shared::milli_floor_doubled(0, 999), 0);
    assert_eq!(shared::milli_floor_doubled(1_000_000, 1000), 2_000_000);
    // Out of 64-bit range stores zero.
    assert_eq!(shared::milli_floor_doubled(u32::MAX, u32::MAX), 0);
    // Cross-check against f64 arithmetic on small values (exact there).
    let mut x: u64 = 0x1234_5678;
    for _ in 0..500 {
        x = x.wrapping_mul(0x5851_F42D_4C95_7F2D).wrapping_add(0x1405_7B7E_F767_814F);
        let rate = (x >> 32) as u32 % 2_000_000;
        let count = (x as u32) % 2_000_000;
        let exact = ((f64::from(rate) * f64::from(count) * 0.001).floor() as u64 * 2) as u32;
        // f32 rounding can shift borderline products by one floor step;
        // allow that single step of slack.
        let got = shared::milli_floor_doubled(rate, count);
        assert!(
            got == exact || got == exact.wrapping_sub(2) || got == exact.wrapping_add(2),
            "rate {rate} count {count}: got {got}, f64 says {exact}"
        );
    }
}

#[test]
fn round_magic_spellings_agree_and_mean_floor() {
    // Both spellings agree with each other and with floor on finite
    // values, including subnormals and x.5 ties.
    let mut vals = vec![
        0.0,
        -0.0,
        0.5,
        -0.5,
        1.5,
        -1.5,
        2.5,
        -2.5,
        100.4999,
        8_388_607.0,
        8_388_608.0,
        9.0e18,
        -9.0e18,
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1e-30,
        -1e-30,
        f32::MAX,
        f32::MIN,
    ];
    let mut x: u64 = 0xDEAD_BEEF;
    for _ in 0..500 {
        x = x.wrapping_mul(0x5851_F42D_4C95_7F2D).wrapping_add(0x1405_7B7E_F767_814F);
        vals.push(f32::from_bits((x >> 32) as u32));
    }
    for v in vals {
        let a = shared::round_down_magic_soft(v);
        let b = shared::round_down_magic_adpcm(v);
        if v.is_nan() {
            // Both stay NaN; payloads may differ by spelling, and both
            // truncate to zero downstream.
            assert!(a.is_nan() && b.is_nan(), "NaN in, non-NaN out at {v}");
        } else {
            assert_eq!(a.to_bits(), b.to_bits(), "spellings differ at {v}");
            if v.is_finite() {
                assert_eq!(a, v.floor(), "not the floor at {v}");
            } else {
                assert!(a.is_infinite() && a.is_sign_positive() == v.is_sign_positive());
            }
        }
    }
}

#[test]
fn synth_acc_rounding_edges() {
    // edx1 rounds up unless the low 11 bits are exactly zero.
    // ans2 = 0x4000: edx0 = 0x2000, low bits set -> edx1 = 0x4000... check directly.
    let (acc, edx1) = shared::synth_acc(0, 0);
    assert_eq!((acc, edx1), (0, 0));
    // The table word feeds the subtrahend: edi = (w*2)>>2, so four
    // apart in the table word is two apart in the accumulator.
    let (acc1, _) = shared::synth_acc(0x100, 0x2000);
    let (acc2, _) = shared::synth_acc(0x104, 0x2000);
    assert_eq!(acc1.wrapping_sub(acc2), 2);
    // edx0 = 0x1000 (low 11 bits zero): no round-up, edx1 = 2.
    let (_, edx1) = shared::synth_acc(0, 0x2000);
    assert_eq!(edx1, 2);
    // edx0 = 0x1001 (low bits set): rounds up, edx1 = 3.
    let (_, edx1) = shared::synth_acc(0, 0x2002);
    assert_eq!(edx1, 3);
}

#[test]
fn position_arg_wraps() {
    assert_eq!(shared::position_arg(0, 0, 0), 0);
    assert_eq!(
        shared::position_arg(0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF),
        (((0xFFFF_FFFFu32 >> 1) << 17)
            .wrapping_add(0xFFFF_FFFF)
            >> 1)
        .wrapping_add(0xFFFF_FFFF)
    );
    // Word 2 halves to one, shifts up 17, halves back to 0x10000.
    assert_eq!(shared::position_arg(2, 0, 0), 0x10000);
    // A bare cursor halves into place.
    assert_eq!(shared::position_arg(0, 0x20000, 0), 0x10000);
}

#[test]
fn adpcm_predictor_fetch_is_little_endian() {
    let mut v = ad_voice();
    v.flags = 0x10;
    v.codec_table = vec![0x2222_2222u32; 64];
    v.predictor = vec![0u8; 64];
    // ans2 = 0x2000 -> edx0 = 0x1000 -> edx1 = 2 -> fetch at 6..8.
    v.predictor[6] = 0xCD;
    v.predictor[7] = 0xAB;
    v.predictor[8] = 0xEF;
    let mut w = Canned::new();
    w.answer("codec", vec![3]); // word index 6
    w.answer("rate", vec![0x2000]);
    v.queue_block(&mut w, &[9u32; 14], 100, 5);
    assert_eq!(v.out_word, 0xABCD);
    assert_eq!(v.out_byte, 0xEF);
    assert_eq!(v.stored_rate, 0x2000);
    let (acc, _) = shared::synth_acc(0x2222_2222, 0x2000);
    assert_eq!(v.lanes[0].acc, acc);
    assert_eq!(v.lanes[0].rem, 100u32.wrapping_sub(acc));
    assert_eq!(v.cursor, 1);
}

#[test]
fn pc_position_mixer_paths() {
    // Synth path: measure >= cursor keeps the difference.
    let mut v = pc_voice();
    v.flags = 0x10;
    v.tuning = 11;
    v.rate = 13;
    v.mixer_word = 17;
    let mut w = Canned::new();
    w.answer("gate", vec![1]);
    w.answer("measure", vec![100]);
    w.answer("mix", vec![30]);
    w.answer("pos", vec![1000, 2000]);
    assert_eq!(v.position(&mut w), 3000);
    assert!(w.log.iter().any(|(n, a)| n == "pos" && a == &vec![11, 13]));
    assert!(w.log.iter().any(|(n, a)| n == "pos" && a == &vec![70, 17]));
    // Measure below cursor saturates the lead at zero.
    let mut v = pc_voice();
    v.flags = 0x10;
    let mut w = Canned::new();
    w.answer("gate", vec![1]);
    w.answer("measure", vec![30]);
    w.answer("mix", vec![100]);
    w.answer("pos", vec![5, 6]);
    assert_eq!(v.position(&mut w), 11);
    assert!(w.log.iter().any(|(n, a)| n == "pos" && a[0] == 0));
    // Gate shut: all-ones, nothing else called.
    let mut v = pc_voice();
    v.flags = 0x10;
    let mut w = Canned::new();
    w.answer("gate", vec![0x100]); // low byte zero: shut
    assert_eq!(v.position(&mut w), 0xFFFF_FFFF);
    assert_eq!(w.log.len(), 1);
}

#[test]
fn refresh_decrements_only_when_set() {
    // Selector set, counter positive: direct start, counter drops.
    let mut v = soft_voice();
    v.flags = 0x10;
    v.mode_cmp = 0;
    v.mode_byte = 1;
    v.mode_count = 3;
    let mut w = Canned::new();
    w.answer("poll", vec![0x1_0000]);
    w.answer("restart", vec![77]);
    assert_eq!(v.refresh(&mut w), 77);
    assert_eq!(v.mode_count, 2);
    assert!(w.log.iter().any(|(n, a)| n == "own" && a == &vec![0]));
    // Selector set, counter zero: fallback slot instead.
    let mut v = soft_voice();
    v.flags = 0x10;
    v.mode_cmp = 0;
    v.mode_byte = 1;
    v.mode_count = 0;
    let mut w = Canned::new();
    w.answer("poll", vec![0x1_0000]);
    w.answer("restart", vec![78]);
    assert_eq!(v.refresh(&mut w), 78);
    assert_eq!(v.mode_count, 0);
    assert!(w.log.iter().any(|(n, _)| n == "fb"));
    assert!(!w.log.iter().any(|(n, _)| n == "own"));
    // No mismatch: neither runs, level still relays, restart tails.
    let mut v = soft_voice();
    v.flags = 0x10;
    v.mode_cmp = 1;
    let mut w = Canned::new();
    w.answer("poll", vec![0x1_0000]);
    w.answer("restart", vec![79]);
    assert_eq!(v.refresh(&mut w), 79);
    assert!(!w.log.iter().any(|(n, _)| n == "own" || n == "fb"));
    assert!(w.log.iter().any(|(n, _)| n == "level"));
}
