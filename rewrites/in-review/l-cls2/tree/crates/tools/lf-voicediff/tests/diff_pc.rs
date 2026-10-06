//! Differential cases: the PC ADPCM voice.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::voice::pc_adpcm::{PcAdpcmVoice, PcLane};
    use lf_voicediff::rewrites::*;
    use lf_voicediff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        FLAG_EDGE, Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls,
        check_virtual, cookie, words,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const PARAMS: usize = 0x04;
    const RATE: usize = 0x0C;
    const CODEC: usize = 0x10;
    const SRC: usize = 0x14;
    const FLAGS: usize = 0x8C;
    const LANE: usize = 0x90;
    const MODE_CMP: usize = 0x110;
    const MODE_COUNT: usize = 0x114;
    const CURSOR: usize = 0x118;
    const DIV: usize = 0x120;
    const FREQ: usize = 0x124;
    const TUNING: usize = 0x128;
    const LEA_BASE: usize = 0x134;
    const CHILD: usize = 0x140;
    const BUFFER: usize = 0x148;
    const MODE_BYTE: usize = 0x14C;
    const OUT_W: usize = 0x14E;
    const OUT_B: usize = 0x150;
    const OBJ_SIZE: usize = 0x1C0;

    /// A test voice: the 32-bit image plus the collaborator blocks it
    /// points at. Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        params: Image,
        child_obj: Image,
        vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let params = Image::random(0x20, rng);
            let child_obj = Image::random(0x10, rng);
            let mut vtable = VTable::random(0x10, rng);
            let stubs = Stubs::new();
            vtable.set(0x18, stubs.gate);
            vtable.set(0x14, stubs.fallback);
            obj.w32(VT, vtable.addr());
            obj.w32(PARAMS, params.addr());
            obj.w32(CHILD, child_obj.addr());
            Self {
                obj,
                params,
                child_obj,
                vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted voice owning the same words the image holds.
        #[allow(clippy::too_many_arguments)]
        fn lift(
            &self,
            lanes: usize,
            mixer: u32,
            mixer_word: u32,
            codec_table: Vec<u32>,
            predictor: Vec<u8>,
        ) -> PcAdpcmVoice {
            let mut vl = Vec::with_capacity(lanes);
            for k in 0..lanes {
                let base = LANE + k * 64;
                let mut p = [0u32; 14];
                for (i, w) in p.iter_mut().enumerate() {
                    *w = self.obj.r32(base + i * 4);
                }
                vl.push(PcLane {
                    params: p,
                    acc: self.obj.r32(base + 0x38),
                    rem: self.obj.r32(base + 0x3C),
                });
            }
            PcAdpcmVoice {
                flags: self.obj.r8(FLAGS),
                rate: self.obj.r32(RATE),
                codec: cookie(self.obj.r32(CODEC)),
                level: self.params.r32(0),
                params_status: self.params.buf[0x18],
                child: cookie(self.obj.r32(CHILD)),
                buffer: cookie(self.obj.r32(BUFFER)),
                lanes: vl,
                cursor: self.obj.r32(CURSOR),
                divisor: self.obj.r32(DIV),
                tuning: self.obj.r32(TUNING),
                freq: self.obj.r32(FREQ),
                mode_cmp: self.obj.r32(MODE_CMP),
                mode_count: self.obj.r32(MODE_COUNT),
                mode_byte: self.obj.r8(MODE_BYTE),
                mixer_word,
                mixer: cookie(mixer),
                codec_table,
                predictor,
                out_word: u16::from_le_bytes([self.obj.r8(OUT_W), self.obj.r8(OUT_W + 1)]),
                out_byte: self.obj.r8(OUT_B),
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::PcAdpcmVoice;
        use lf_audio::voice::PcWorld;

        /// Drops the stopping-bit half of the first arm.
        pub fn stopping_first<W: PcWorld>(v: &PcAdpcmVoice, w: &mut W) -> bool {
            if v.flags & 1 != 0 {
                return true;
            }
            if v.flags & 8 != 0 {
                return true;
            }
            if w.child_query(v.child) & 0xFF != 0 {
                return true;
            }
            v.params_status & 0x40 != 0
        }

        /// Clears only the stopped bit.
        pub fn stop_half(v: &mut PcAdpcmVoice) {
            v.flags &= !0x01;
        }

        /// Folds the wrong bits: bit 0 or bit 3.
        pub fn resume_mode_bits(flags: u8) -> u32 {
            u32::from(flags & 0x09 != 0)
        }

        /// Treats NaN levels as zero.
        pub fn start_nan_zero(level: u32) -> bool {
            f32::from_bits(level) == 0.0 || f32::from_bits(level).is_nan()
        }

        /// Gates the buffer release on the synth bit (the software
        /// voice's rule, not this class's).
        pub fn teardown_gated(flags: u8) -> bool {
            flags & 0x10 != 0
        }

        /// Reduces the poll answer at the wrong boundary.
        pub fn refresh_bit(ans: u32) -> u32 {
            u32::from(ans > 0x10000)
        }

        /// Saturates the lead at zero with a comparison flip.
        pub fn lead_flip(m: u32, c: u32) -> u32 {
            if m <= c { m.wrapping_sub(c) } else { 0 }
        }

        /// Adds the derived count instead of subtracting it.
        pub fn block_rem_add(src_bias: u32, acc: u32) -> u32 {
            src_bias.wrapping_add(acc)
        }
    }

    #[test]
    fn pc_stopping_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C06);
        let mut caught = 0;
        let queries = [0u32, 1, 0xFF, 0x100, 0xFF00, 0x12345600, 0xFFFFFFFF, 42];
        for flags in 0..=255u8 {
            for &q in &queries {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w8(FLAGS, flags);
                fx.params.buf[0x18] = if (flags as u32 + q) % 3 == 0 { 0x40 } else { 0 };
                let child = fx.obj.r32(CHILD);
                let v = fx.lift(1, 0, 0, vec![], vec![]);
                let before = fx.obj.buf.clone();
                rt::set_script(&[(1, StubKind::Thiscall1, vec![q])]);
                let mut fake = Fake::new();
                fake.answer("pc.query", vec![q]);
                let got = unsafe { fn_0088F2F0::rw_0088f2f0(fx.obj.buf.as_ptr()) };
                let want = v.is_stopping(&mut fake);
                assert_eq!(got, u8::from(want), "flags {flags:#x} query {q:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![(1, vec![child], "pc.query", vec![child])],
                );
                let mut wf = Fake::new();
                wf.answer("pc.query", vec![q]);
                if wrong::stopping_first(&v, &mut wf) != want {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong stopping never caught");
    }

    #[test]
    fn pc_stop_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C05);
        let mut caught = 0;
        for i in 0..200u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < FLAG_EDGE.len() as u32 {
                FLAG_EDGE[i as usize]
            } else {
                rng.u8()
            };
            fx.obj.w8(FLAGS, flags);
            let child = fx.obj.r32(CHILD);
            let stop_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1, 0, 0, vec![], vec![]);
            rt::set_script(&[(1, StubKind::Thiscall1, vec![stop_ans])]);
            let mut fake = Fake::new();
            fake.answer("pc.stop", vec![stop_ans]);
            let got = unsafe { fn_0088F4E0::rw_0088F4E0(fx.this()) };
            let want = v.stop(&mut fake);
            assert_eq!(got, want, "flags {flags:#x}");
            assert_eq!(fx.obj.r8(FLAGS), v.flags);
            assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![(1, vec![child], "pc.stop", vec![child])],
            );
            let mut w = PcAdpcmVoice {
                flags,
                ..v.clone()
            };
            wrong::stop_half(&mut w);
            if w.flags != v.flags {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong stop never caught");
    }

    #[test]
    fn pc_resume_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C04);
        let mut caught = 0;
        for i in 0..240u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < 64 { (i % 32) as u8 } else { rng.u8() };
            fx.obj.w8(FLAGS, flags);
            let child = fx.obj.r32(CHILD);
            let restart_ans = rng.u32();
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1, 0, 0, vec![], vec![]);
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![restart_ans]),
                (2, StubKind::Thiscall2, vec![0]),
            ]);
            let mut fake = Fake::new();
            fake.answer("pc.restart", vec![restart_ans]);
            unsafe { fn_0088F500::rw_0088F500(fx.this()) };
            v.resume(&mut fake);
            assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x}");
            let pending = flags & 8 != 0;
            let mode = ((((flags >> 3) | flags) >> 1) & 1) as u32;
            if pending {
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![
                        (1, vec![fx.this()], "pc.restart", vec![]),
                        (2, vec![child, mode], "pc.resume", vec![child, mode]),
                    ],
                );
            } else {
                assert_only_changed(&before, &fx.obj.buf, &[]);
                assert!(rt::take_numbered().is_empty());
                assert!(fake.log.is_empty());
            }
            if pending && wrong::resume_mode_bits(flags) != mode {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong resume never caught");
    }

    #[test]
    fn pc_start_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C03);
        let mut caught = 0;
        let mut levels = vec![0u32, 0x8000_0000, 1, 0x7FC0_0000, 0x7F80_0001, 0xFFC0_1234];
        for _ in 0..20 {
            levels.push(rng.u32());
        }
        let mut i = 0u32;
        for &level in &levels {
            for flags in [0x00u8, 0x10, 0x1F, 0xEF, rng.u8()] {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w8(FLAGS, flags);
                fx.obj.w32(RATE, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.params.w32(0, level);
                let a0 = U32_EDGE[(i as usize) % U32_EDGE.len()];
                let rate_ans = rng.u32();
                let child = fx.obj.r32(CHILD);
                let base = fx.obj.r32(RATE);
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1, 0, 0, vec![], vec![]);
                rt::set_script(&[
                    (1, StubKind::Thiscall2, vec![0]),
                    (2, StubKind::Cdecl2, vec![rate_ans]),
                    (3, StubKind::Thiscall2, vec![0]),
                    (4, StubKind::Thiscall2, vec![0]),
                ]);
                let mut fake = Fake::new();
                fake.answer("pc.rate", vec![rate_ans]);
                unsafe { fn_0088F330::rw_0088F330(fx.this(), a0) };
                v.start(&mut fake, a0);
                assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x} level {level:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                let synth = flags & 0x10 != 0;
                let mut expect = vec![];
                if synth {
                    expect.push((1, vec![fx.this(), 1], "pc.own_start", vec![1]));
                } else {
                    expect.push((2, vec![a0, base], "pc.rate", vec![a0, base]));
                    expect.push((
                        3,
                        vec![child, rate_ans.wrapping_mul(2)],
                        "pc.start",
                        vec![child, rate_ans.wrapping_mul(2)],
                    ));
                }
                expect.push((4, vec![fx.this(), level], "pc.level", vec![level]));
                check_calls(rt::take_numbered(), std::mem::take(&mut fake.log), expect);
                if wrong::start_nan_zero(level) != (f32::from_bits(level) == 0.0) {
                    caught += 1;
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong start never caught");
    }

    #[test]
    fn pc_teardown_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C02);
        let mut caught = 0;
        for i in 0..160u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < FLAG_EDGE.len() as u32 {
                FLAG_EDGE[i as usize]
            } else {
                rng.u8()
            };
            fx.obj.w8(FLAGS, flags);
            let null_child = i % 3 == 0;
            if null_child {
                fx.obj.w32(CHILD, 0);
            }
            let child = fx.obj.r32(CHILD);
            let buf = fx.obj.r32(BUFFER);
            let base_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1, 0, 0, vec![], vec![]);
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::Cdecl1, vec![0]),
                (3, StubKind::Thiscall1, vec![base_ans]),
            ]);
            let mut fake = Fake::new();
            fake.answer("pc.base", vec![base_ans]);
            let got = unsafe { fn_0088F4A0::rw_0088F4A0(fx.this()) };
            let want = v.teardown(&mut fake);
            assert_eq!(got, want, "flags {flags:#x} null_child {null_child}");
            assert_eq!(fx.obj.r32(CHILD), words(v.child));
            assert_eq!(fx.obj.r32(BUFFER), words(v.buffer));
            assert_eq!(fx.obj.r32(BUFFER), 0, "buffer always released");
            let mut changed = vec![(BUFFER, 4)];
            if !null_child {
                assert_eq!(fx.obj.r32(CHILD), 0);
                changed.push((CHILD, 4));
            }
            assert_only_changed(&before, &fx.obj.buf, &changed);
            let mut expect = vec![];
            if !null_child {
                expect.push((1, vec![child, 0], "pc.shutdown", vec![child]));
            }
            expect.push((2, vec![buf], "pc.free", vec![buf]));
            expect.push((3, vec![fx.this()], "pc.base", vec![]));
            check_calls(rt::take_numbered(), std::mem::take(&mut fake.log), expect);
            // The software voice's gated rule would skip the release off
            // the synth path; this class never skips.
            if !wrong::teardown_gated(flags) {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong teardown never caught");
    }

    #[test]
    fn pc_refresh_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xEF11 - 1);
        let mut caught = 0;
        let polls = [0u32, 1, 0xFFFF, 0x10000, 0x10001, 0xFFFFFFFF, 42];
        let mut i = 0u32;
        for &poll in &polls {
            for flags in [0x00u8, 0x10, 0x1F, rng.u8()] {
                for mode_byte in [0u8, 1] {
                    for mode_count in [0u32, 1, 5] {
                        let mut fx = Fixture::build(&mut rng);
                        fx.obj.w8(FLAGS, flags);
                        fx.obj.w8(MODE_BYTE, mode_byte);
                        fx.obj.w32(MODE_COUNT, mode_count);
                        let want_cmp = if i % 2 == 0 {
                            u32::from(poll >= 0x10000)
                        } else {
                            1 - u32::from(poll >= 0x10000)
                        };
                        fx.obj.w32(MODE_CMP, want_cmp);
                        let level = rng.u32();
                        fx.params.w32(0, level);
                        let child = fx.obj.r32(CHILD);
                        let restart_ans = rng.u32();
                        let before = fx.obj.buf.clone();
                        let mut v = fx.lift(1, 0, 0, vec![], vec![]);
                        rt::set_script(&[
                            (1, StubKind::Thiscall1, vec![poll]),
                            (2, StubKind::Thiscall2, vec![0]),
                            (4, StubKind::Thiscall2, vec![0]),
                            (5, StubKind::Thiscall1, vec![restart_ans]),
                        ]);
                        let mut fake = Fake::new();
                        fake.answer("pc.poll", vec![poll]);
                        fake.answer("pc.restart", vec![restart_ans]);
                        let got = unsafe { fn_0088F540::rw_0088F540(fx.this()) };
                        let want = v.refresh(&mut fake);
                        assert_eq!(got, want, "poll {poll:#x} flags {flags:#x}");
                        assert_eq!(fx.obj.r32(MODE_COUNT), v.mode_count);
                        let synth = flags & 0x10 != 0;
                        let bit = u32::from(poll >= 0x10000);
                        let mismatch = synth && bit != want_cmp;
                        let direct = mode_byte == 0 || mode_count > 0;
                        let decremented = mismatch && mode_byte != 0 && mode_count > 0;
                        if decremented {
                            assert_only_changed(&before, &fx.obj.buf, &[(MODE_COUNT, 4)]);
                        } else {
                            assert_only_changed(&before, &fx.obj.buf, &[]);
                        }
                        let mut expect = vec![];
                        if synth {
                            expect.push((1, vec![child], "pc.poll", vec![child]));
                            if mismatch && direct {
                                expect.push((2, vec![fx.this(), 0], "pc.own_start", vec![0]));
                            }
                        }
                        expect.push((4, vec![fx.this(), level], "pc.level", vec![level]));
                        expect.push((5, vec![fx.this()], "pc.restart", vec![]));
                        let mut lift_log = std::mem::take(&mut fake.log);
                        let mut saw_fallback = false;
                        lift_log.retain(|(n, _)| {
                            if n == "pc.fallback" {
                                saw_fallback = true;
                                false
                            } else {
                                true
                            }
                        });
                        check_calls(rt::take_numbered(), lift_log, expect);
                        if mismatch && !direct {
                            check_virtual(
                                rt::take_virtual(),
                                vec![("fallback", vec![fx.this()])],
                            );
                            assert!(saw_fallback, "lift fallback call missing");
                        } else {
                            assert!(rt::take_virtual().is_empty());
                            assert!(!saw_fallback, "spurious lift fallback call");
                        }
                        if synth && wrong::refresh_bit(poll) != bit {
                            caught += 1;
                        }
                        i += 1;
                    }
                }
            }
        }
        assert!(i > 100, "too few cases: {i}");
        assert!(caught > 0, "wrong refresh never caught");
    }

    #[test]
    fn pc_position_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C10);
        let mut caught = 0;
        let gates = [0u32, 1, 0x100, 0xFF, 0xFF00, 0x12345600, 0xFFFFFFFF, 3];
        let mut i = 0u32;
        for &gate in &gates {
            for flags in [0x00u8, 0x10, 0x1F, rng.u8()] {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w8(FLAGS, flags);
                fx.obj.w32(FREQ, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.obj.w32(TUNING, rng.u32());
                fx.obj.w32(RATE, rng.u32());
                let child = fx.obj.r32(CHILD);
                let freq = fx.obj.r32(FREQ);
                let tuning = fx.obj.r32(TUNING);
                let base = fx.obj.r32(RATE);
                // Mixer block with the scaling word at +0x80.
                let mut mixer = Image::random(0x84, &mut rng);
                let mixer_word = if i % 3 == 0 { rng.u32() } else { mixer.r32(0x80) };
                mixer.w32(0x80, mixer_word);
                unsafe {
                    rt::MIXER_CELL = mixer.addr();
                }
                let (m, c) = (
                    U32_EDGE[(i as usize) % U32_EDGE.len()],
                    U32_EDGE[((i + 3) as usize) % U32_EDGE.len()],
                );
                let poll = U32_EDGE[((i + 5) as usize) % U32_EDGE.len()];
                let (p_ans, q_ans, pos_ans) = (rng.u32(), rng.u32(), rng.u32());
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1, mixer.addr(), mixer_word, vec![], vec![]);
                rt::set_script(&[
                    (2, StubKind::Thiscall1, vec![m]),
                    (3, StubKind::Thiscall1, vec![c]),
                    (4, StubKind::Cdecl2, vec![p_ans, q_ans, pos_ans]),
                    (5, StubKind::Thiscall1, vec![poll]),
                ]);
                rt::set_virtual(&[("gate", vec![gate])]);
                let mut fake = Fake::new();
                fake.answer("pc.gate", vec![gate]);
                fake.answer("pc.measure", vec![m]);
                fake.answer("pc.mixcur", vec![c]);
                fake.answer("pc.pos", vec![p_ans, q_ans, pos_ans]);
                fake.answer("pc.poll", vec![poll]);
                let got = unsafe { fn_0088F030::rw_0088F030(fx.this()) };
                let want = v.position(&mut fake);
                assert_eq!(got, want, "gate {gate:#x} flags {flags:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_virtual(rt::take_virtual(), vec![("gate", vec![fx.this()])]);
                let mut lift_log = std::mem::take(&mut fake.log);
                assert_eq!(
                    lift_log.remove(0),
                    ("pc.gate".to_string(), vec![]),
                    "lift gate call"
                );
                let synth = flags & 0x10 != 0;
                if gate as u8 == 0 {
                    assert_eq!(got, 0xFFFF_FFFF);
                    assert!(rt::take_numbered().is_empty());
                    assert!(lift_log.is_empty());
                } else if synth {
                    let lead = if m >= c { m.wrapping_sub(c) } else { 0 };
                    assert_eq!(got, p_ans.wrapping_add(q_ans));
                    check_calls(
                        rt::take_numbered(),
                        lift_log,
                        vec![
                            (2, vec![child], "pc.measure", vec![child]),
                            (3, vec![mixer.addr()], "pc.mixcur", vec![mixer.addr()]),
                            (4, vec![tuning, base], "pc.pos", vec![tuning, base]),
                            (4, vec![lead, mixer_word], "pc.pos", vec![lead, mixer_word]),
                        ],
                    );
                    if wrong::lead_flip(m, c) != lead {
                        caught += 1;
                    }
                } else {
                    let scaled = (((freq >> 1) << 17).wrapping_add(poll) >> 1)
                        .wrapping_add(tuning);
                    check_calls(
                        rt::take_numbered(),
                        lift_log,
                        vec![
                            (5, vec![child], "pc.poll", vec![child]),
                            (4, vec![scaled, base], "pc.pos", vec![scaled, base]),
                        ],
                    );
                    // The wrong lead also disagrees on plain-path cases
                    // whenever measure/cursor straddle; probe anyway.
                    if wrong::lead_flip(m, c) != (if m >= c { m.wrapping_sub(c) } else { 0 }) {
                        caught += 1;
                    }
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong position never caught");
    }

    #[test]
    fn pc_queue_block_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9C08);
        let mut caught = 0;
        let mut cases = 0u32;
        // Resolved entries stay inside a 64-word table; converted rates
        // stay small so the predictor fetch stays tiny, plus one large
        // rate with a matching predictor.
        let table_words: Vec<u32> = (0..64).map(|_| rng.u32()).collect();
        for trial in 0..160u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if trial < FLAG_EDGE.len() as u32 {
                FLAG_EDGE[trial as usize]
            } else {
                rng.u8()
            };
            fx.obj.w8(FLAGS, flags);
            let divisor = 1 + (trial % 2);
            let cursor = trial % divisor;
            fx.obj.w32(DIV, divisor);
            fx.obj.w32(CURSOR, cursor);
            fx.obj.w32(RATE, rng.u32());
            let rate = fx.obj.r32(RATE);
            let a0 = if trial % 5 == 0 {
                0
            } else {
                U32_EDGE[(trial as usize) % U32_EDGE.len()]
            };
            // Source block: 14 words; bias is word 3.
            let mut src = Image::random(0x40, &mut rng);
            for k in 0..14 {
                src.w32(k * 4, rng.u32());
            }
            fx.obj.w32(SRC, src.addr());
            let mut src_words = [0u32; 14];
            for (k, w) in src_words.iter_mut().enumerate() {
                *w = src.r32(k * 4);
            }
            let src_bias = src.r32(0xC);
            // Codec blocks: [codec+0x7C] -> t1 -> t2 words.
            let mut t2 = Image::random(64 * 4, &mut rng);
            for (k, w) in table_words.iter().enumerate() {
                t2.w32(k * 4, *w);
            }
            let mut t1 = Image::random(8, &mut rng);
            t1.w32(0, t2.addr());
            let mut codec = Image::random(0x80, &mut rng);
            codec.w32(0x7C, t1.addr());
            fx.obj.w32(CODEC, codec.addr());
            let ans1 = (rng.u32() % 32) as u32;
            // Converted rate: small, plus edge shapes; one huge case.
            let ans2 = if trial == 7 {
                0xFFFF_FFFF
            } else {
                match trial % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 0x7FF,
                    3 => 0x800,
                    4 => rng.u32() % 0x10000,
                    _ => U32_EDGE[(trial as usize) % U32_EDGE.len()] % 0x20000,
                }
            };
            // Predictor sized for the fetch: 3*edx1+3 bytes.
            let edx0 = ans2.wrapping_mul(2) >> 2;
            let edx1 = (edx0 >> 11) + u32::from(edx0 & 0x7ff != 0);
            let pred_len = (edx1.wrapping_mul(3) as usize) + 3;
            let pred = Image::random(pred_len.max(8), &mut rng);
            fx.obj.w32(LEA_BASE, pred.addr());
            let pred_bytes = pred.buf.to_vec();
            let synth = a0 != 0 && flags & 0x10 != 0;
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(
                divisor as usize,
                0,
                0,
                table_words.clone(),
                pred_bytes.clone(),
            );
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![ans1]),
                (2, StubKind::Cdecl2, vec![ans2]),
            ]);
            let mut fake = Fake::new();
            fake.answer("pc.codec", vec![ans1]);
            fake.answer("pc.rate", vec![ans2]);
            unsafe { fn_0088E7B0::rw_0088E7B0(fx.this(), a0) };
            v.queue_block(&mut fake, &src_words, src_bias, a0);
            // Lanes agree word for word.
            for k in 0..divisor as usize {
                let base = LANE + k * 64;
                for w in 0..14 {
                    assert_eq!(fx.obj.r32(base + w * 4), v.lanes[k].params[w]);
                }
                assert_eq!(fx.obj.r32(base + 0x38), v.lanes[k].acc, "trial {trial}");
                assert_eq!(fx.obj.r32(base + 0x3C), v.lanes[k].rem, "trial {trial}");
            }
            assert_eq!(fx.obj.r32(TUNING), v.tuning, "trial {trial}");
            assert_eq!(fx.obj.r32(CURSOR), v.cursor, "trial {trial}");
            assert_eq!(
                u16::from_le_bytes([fx.obj.r8(OUT_W), fx.obj.r8(OUT_W + 1)]),
                v.out_word,
                "trial {trial}"
            );
            assert_eq!(fx.obj.r8(OUT_B), v.out_byte, "trial {trial}");
            let lane_base = LANE + (cursor as usize) * 64;
            let mut changed = vec![(lane_base, 64), (CURSOR, 4)];
            if synth {
                changed.push((TUNING, 4));
                changed.push((OUT_W, 2));
                changed.push((OUT_B, 1));
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![
                        (
                            1,
                            vec![codec.addr(), a0],
                            "pc.codec",
                            vec![codec.addr(), a0],
                        ),
                        (2, vec![a0, rate], "pc.rate", vec![a0, rate]),
                    ],
                );
            } else {
                assert!(rt::take_numbered().is_empty());
                assert!(fake.log.is_empty());
            }
            assert_only_changed(&before, &fx.obj.buf, &changed);
            // External blocks untouched.
            for k in 0..14 {
                assert_eq!(src.r32(k * 4), src_words[k]);
            }
            assert_eq!(&pred.buf[..], &pred_bytes[..]);
            let acc = fx.obj.r32(lane_base + 0x38);
            let rem = fx.obj.r32(lane_base + 0x3C);
            if wrong::block_rem_add(src_bias, acc) != rem {
                caught += 1;
            }
            cases += 1;
            let _ = (t2, t1, codec, pred);
        }
        assert!(cases > 100, "too few cases: {cases}");
        assert!(caught > 0, "wrong block never caught");
    }
}
