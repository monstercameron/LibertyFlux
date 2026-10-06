//! Differential cases: the software mixer voice.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::voice::soft::{SoftLane, SoftVoice};
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
    const AUX: usize = 0x10;
    const SRC: usize = 0x14;
    const FLAGS: usize = 0x8C;
    const LANE: usize = 0x90;
    const MODE_CMP: usize = 0x110;
    const MODE_COUNT: usize = 0x114;
    const CURSOR: usize = 0x118;
    const DIV: usize = 0x120;
    const FREQ: usize = 0x124;
    const TUNING: usize = 0x128;
    const CHILD: usize = 0x130;
    const BUFFER: usize = 0x138;
    const MODE_BYTE: usize = 0x13C;
    const OBJ_SIZE: usize = 0x180;

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
        fn lift(&self, lanes: usize) -> SoftVoice {
            let mut vl = Vec::with_capacity(lanes);
            for k in 0..lanes {
                let base = LANE + k * 64;
                let mut p = [0u32; 14];
                for (i, w) in p.iter_mut().enumerate() {
                    *w = self.obj.r32(base + i * 4);
                }
                vl.push(SoftLane {
                    params: p,
                    pos: self.obj.r32(base + 0x38),
                    rem: self.obj.r32(base + 0x3C),
                });
            }
            SoftVoice {
                flags: self.obj.r8(FLAGS),
                rate: self.obj.r32(RATE),
                aux: cookie(self.obj.r32(AUX)),
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
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::SoftVoice;
        use lf_audio::voice::SoftWorld;

        /// Drops the stopping-bit half of the first arm.
        pub fn stopping_first<W: SoftWorld>(v: &SoftVoice, w: &mut W) -> bool {
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

        /// Forgets the synth gate.
        pub fn quiet_no_gate(v: &SoftVoice) -> bool {
            v.lanes[v.cursor as usize].rem == 0
        }

        /// Clears only the stopped bit.
        pub fn stop_half(v: &mut SoftVoice) {
            v.flags &= !0x01;
        }

        /// Uses the entry flags instead of re-reading (same here), but
        /// folds the wrong bits: bit 0 or bit 3.
        pub fn resume_mode_bits(flags: u8) -> u32 {
            u32::from(flags & 0x09 != 0)
        }

        /// Treats NaN levels as zero.
        pub fn start_nan_zero(level: u32) -> bool {
            f32::from_bits(level) == 0.0 || f32::from_bits(level).is_nan()
        }

        /// Frees the buffer unconditionally.
        pub fn teardown_always_free(v: &SoftVoice) -> bool {
            v.buffer.is_some()
        }

        /// Reduces the poll answer at the wrong boundary.
        pub fn refresh_bit(ans: u32) -> u32 {
            u32::from(ans > 0x10000)
        }

        /// Scales without the final halving.
        pub fn position_scaled(freq: u32, ans: u32, bias: u32) -> u32 {
            ((freq >> 1) << 17).wrapping_add(ans).wrapping_add(bias)
        }

        /// Adds the derived count instead of subtracting it.
        pub fn lane_rem_add(src_bias: u32, pos: u32) -> u32 {
            src_bias.wrapping_add(pos)
        }

        /// Answers the step remainder instead of the quotient.
        pub fn lane_quotient(next: u32, divisor: u32) -> u32 {
            next % divisor
        }
    }

    #[test]
    fn soft_stopping_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x5007);
        let mut caught = 0;
        let mut cases = 0;
        // Exhaustive flags; query answers sweep the low byte and upper bytes.
        let queries = [0u32, 1, 0xFF, 0x100, 0xFF00, 0x12345600, 0xFFFFFFFF, 42];
        for flags in 0..=255u8 {
            for &q in &queries {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w8(FLAGS, flags);
                fx.params.buf[0x18] = if cases % 3 == 0 { 0x40 } else { 0x00 };
                let child = fx.obj.r32(CHILD);
                let v = fx.lift(1);
                let before = fx.obj.buf.clone();
                rt::set_script(&[(1, StubKind::Thiscall1, vec![q])]);
                let mut fake = Fake::new();
                fake.answer("soft.query", vec![q]);
                let got = unsafe { fn_0088E430::rw_0088e430(fx.obj.buf.as_ptr()) };
                let want = v.is_stopping(&mut fake);
                assert_eq!(got, u8::from(want), "flags {flags:#x} query {q:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![(1, vec![child], "soft.query", vec![child])],
                );
                let mut wf = Fake::new();
                wf.answer("soft.query", vec![q]);
                if wrong::stopping_first(&v, &mut wf) != want {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases > 2000, "too few cases: {cases}");
        assert!(caught > 0, "wrong stopping never caught");
    }

    #[test]
    fn soft_quiet_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x90E7);
        let mut caught = 0;
        for i in 0..300u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < FLAG_EDGE.len() as u32 {
                FLAG_EDGE[i as usize]
            } else {
                rng.u8()
            };
            fx.obj.w8(FLAGS, flags);
            let divisor = 1 + (i % 2);
            fx.obj.w32(DIV, divisor);
            let cursor = i % divisor;
            fx.obj.w32(CURSOR, cursor);
            // Remainder: zero, nonzero, edge.
            let rem = match i % 4 {
                0 => 0,
                1 => 1,
                2 => U32_EDGE[(i as usize) % U32_EDGE.len()],
                _ => rng.u32(),
            };
            fx.obj
                .w32(LANE + (cursor as usize) * 64 + 0x3C, rem);
            let before = fx.obj.buf.clone();
            let v = fx.lift(divisor as usize);
            let got = unsafe { fn_0088E470::rw_0088e470(fx.obj.buf.as_ptr()) };
            let want = v.lane_rest_quiet();
            assert_eq!(got, u8::from(want), "flags {flags:#x} rem {rem:#x}");
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            if wrong::quiet_no_gate(&v) != want {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong quiet never caught");
    }

    #[test]
    fn soft_stop_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x5709);
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
            let mut v = fx.lift(1);
            rt::set_script(&[(1, StubKind::Thiscall1, vec![stop_ans])]);
            let mut fake = Fake::new();
            fake.answer("soft.stop", vec![stop_ans]);
            let got = unsafe { fn_0088E660::rw_0088E660(fx.this()) };
            let want = v.stop(&mut fake);
            assert_eq!(got, want, "flags {flags:#x}");
            assert_eq!(fx.obj.r8(FLAGS), v.flags);
            assert_eq!(fx.obj.r8(FLAGS), flags & !0x09);
            assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![(1, vec![child], "soft.stop", vec![child])],
            );
            let mut w = SoftVoice {
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
    fn soft_resume_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xE54E);
        let mut caught = 0;
        for i in 0..240u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < 64 { (i % 32) as u8 } else { rng.u8() };
            fx.obj.w8(FLAGS, flags);
            let child = fx.obj.r32(CHILD);
            let restart_ans = rng.u32();
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1);
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![restart_ans]),
                (2, StubKind::Thiscall2, vec![0]),
            ]);
            let mut fake = Fake::new();
            fake.answer("soft.restart", vec![restart_ans]);
            unsafe { fn_0088E680::rw_0088E680(fx.this()) };
            v.resume(&mut fake);
            assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x}");
            let pending = flags & 8 != 0;
            let mode = ((((flags >> 3) | flags) >> 1) & 1) as u32;
            if pending {
                assert_eq!(fx.obj.r8(FLAGS), flags & !8);
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![
                        (1, vec![fx.this()], "soft.restart", vec![]),
                        (2, vec![child, mode], "soft.resume", vec![child, mode]),
                    ],
                );
            } else {
                assert_eq!(fx.obj.r8(FLAGS), flags);
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
    fn soft_start_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x5747);
        let mut caught = 0;
        // Level words: zeros, denormals, NaNs with payloads, randoms.
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
                let mut v = fx.lift(1);
                rt::set_script(&[
                    (1, StubKind::Thiscall2, vec![0]),
                    (2, StubKind::Cdecl2, vec![rate_ans]),
                    (3, StubKind::Thiscall2, vec![0]),
                    (4, StubKind::Thiscall2, vec![0]),
                ]);
                let mut fake = Fake::new();
                fake.answer("soft.rate", vec![rate_ans]);
                unsafe { fn_0088E4A0::rw_0088E4A0(fx.this(), a0) };
                v.start(&mut fake, a0);
                assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x} level {level:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                let synth = flags & 0x10 != 0;
                let mut expect = vec![];
                if synth {
                    expect.push((1, vec![fx.this(), 1], "soft.own_start", vec![1]));
                } else {
                    expect.push((2, vec![a0, base], "soft.rate", vec![a0, base]));
                    expect.push((
                        3,
                        vec![child, rate_ans.wrapping_mul(2)],
                        "soft.start",
                        vec![child, rate_ans.wrapping_mul(2)],
                    ));
                }
                expect.push((4, vec![fx.this(), level], "soft.level", vec![level]));
                check_calls(rt::take_numbered(), std::mem::take(&mut fake.log), expect);
                // Wrong lift: NaN-as-zero disagrees exactly on NaN levels.
                let wzero = wrong::start_nan_zero(level);
                let is_zero = f32::from_bits(level) == 0.0;
                if wzero != is_zero {
                    caught += 1;
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong start never caught");
    }

    #[test]
    fn soft_teardown_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x7EA4);
        let mut caught = 0;
        for i in 0..160u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < FLAG_EDGE.len() as u32 {
                FLAG_EDGE[i as usize]
            } else {
                rng.u8()
            };
            fx.obj.w8(FLAGS, flags);
            // Null or present child.
            let null_child = i % 3 == 0;
            if null_child {
                fx.obj.w32(CHILD, 0);
            }
            let child = fx.obj.r32(CHILD);
            let buf = fx.obj.r32(BUFFER);
            let base_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let pre = fx.lift(1);
            let mut v = fx.lift(1);
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::Cdecl1, vec![0]),
                (3, StubKind::Thiscall1, vec![base_ans]),
            ]);
            let mut fake = Fake::new();
            fake.answer("soft.base", vec![base_ans]);
            let got = unsafe { fn_0088E610::rw_0088E610(fx.this()) };
            let want = v.teardown(&mut fake);
            assert_eq!(got, want, "flags {flags:#x} null_child {null_child}");
            assert_eq!(fx.obj.r32(CHILD), words(v.child));
            assert_eq!(fx.obj.r32(BUFFER), words(v.buffer));
            let synth = flags & 0x10 != 0;
            let mut changed = vec![];
            if !null_child {
                assert_eq!(fx.obj.r32(CHILD), 0);
                changed.push((CHILD, 4));
            }
            if synth {
                assert_eq!(fx.obj.r32(BUFFER), 0);
                changed.push((BUFFER, 4));
            }
            assert_only_changed(&before, &fx.obj.buf, &changed);
            let mut expect = vec![];
            if !null_child {
                expect.push((1, vec![child, 0], "soft.shutdown", vec![child]));
            }
            if synth {
                expect.push((2, vec![buf], "soft.free", vec![buf]));
            }
            expect.push((3, vec![fx.this()], "soft.base", vec![]));
            check_calls(rt::take_numbered(), std::mem::take(&mut fake.log), expect);
            // Wrong lift frees whenever a buffer is present, ignoring the
            // synth gate: caught when they disagree.
            if wrong::teardown_always_free(&pre) != synth {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong teardown never caught");
    }

    #[test]
    fn soft_refresh_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xEF11);
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
                        let mut v = fx.lift(1);
                        rt::set_script(&[
                            (1, StubKind::Thiscall1, vec![poll]),
                            (2, StubKind::Thiscall2, vec![0]),
                            (4, StubKind::Thiscall2, vec![0]),
                            (5, StubKind::Thiscall1, vec![restart_ans]),
                        ]);
                        let mut fake = Fake::new();
                        fake.answer("soft.poll", vec![poll]);
                        fake.answer("soft.restart", vec![restart_ans]);
                        let got = unsafe { fn_0088E6C0::rw_0088E6C0(fx.this()) };
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
                            expect.push((1, vec![child], "soft.poll", vec![child]));
                            if mismatch && direct {
                                expect.push((2, vec![fx.this(), 0], "soft.own_start", vec![0]));
                            }
                        }
                        expect.push((4, vec![fx.this(), level], "soft.level", vec![level]));
                        expect.push((5, vec![fx.this()], "soft.restart", vec![]));
                        // The fallback trait call has no numbered twin; lift
                        // it out before the joint comparison.
                        let mut lift_log = std::mem::take(&mut fake.log);
                        let mut saw_fallback = false;
                        lift_log.retain(|(n, _)| {
                            if n == "soft.fallback" {
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
    fn soft_position_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9010);
        let mut caught = 0;
        let gates = [0u32, 1, 0x100, 0xFF, 0xFF00, 0x12345600, 0xFFFFFFFF, 3];
        let mut i = 0u32;
        for &gate in &gates {
            for _ in 0..12 {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w32(FREQ, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.obj.w32(TUNING, rng.u32());
                fx.obj.w32(RATE, rng.u32());
                let child = fx.obj.r32(CHILD);
                let freq = fx.obj.r32(FREQ);
                let tuning = fx.obj.r32(TUNING);
                let base = fx.obj.r32(RATE);
                let poll = U32_EDGE[(i as usize) % U32_EDGE.len()];
                let pos_ans = rng.u32();
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1);
                rt::set_script(&[
                    (2, StubKind::Thiscall1, vec![poll]),
                    (3, StubKind::Cdecl2, vec![pos_ans]),
                ]);
                rt::set_virtual(&[("gate", vec![gate])]);
                let mut fake = Fake::new();
                fake.answer("soft.gate", vec![gate]);
                fake.answer("soft.poll", vec![poll]);
                fake.answer("soft.pos", vec![pos_ans]);
                let got = unsafe { fn_0088E230::rw_0088E230(fx.this()) };
                let want = v.position(&mut fake);
                assert_eq!(got, want, "gate {gate:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_virtual(rt::take_virtual(), vec![("gate", vec![fx.this()])]);
                // The lift-side gate call precedes the numbered twins.
                let mut lift_log = std::mem::take(&mut fake.log);
                assert_eq!(
                    lift_log.remove(0),
                    ("soft.gate".to_string(), vec![]),
                    "lift gate call"
                );
                if gate as u8 == 0 {
                    assert_eq!(got, 0xFFFF_FFFF);
                    assert!(rt::take_numbered().is_empty());
                    assert!(lift_log.is_empty());
                } else {
                    let scaled = (((freq >> 1) << 17).wrapping_add(poll) >> 1)
                        .wrapping_add(tuning);
                    check_calls(
                        rt::take_numbered(),
                        lift_log,
                        vec![
                            (2, vec![child], "soft.poll", vec![child]),
                            (3, vec![scaled, base], "soft.pos", vec![scaled, base]),
                        ],
                    );
                }
                i += 1;
            }
        }
        // Wrong lift: scaling without the final halving differs whenever
        // the sum is odd or the shift drops a bit; probe directly.
        for _ in 0..50 {
            let (f, p, b) = (rng.u32(), rng.u32(), rng.u32());
            let good = (((f >> 1) << 17).wrapping_add(p) >> 1).wrapping_add(b);
            if wrong::position_scaled(f, p, b) != good {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong position never caught");
    }

    #[test]
    fn soft_refresh_lane_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x8F08);
        let mut caught = 0;
        let mut cases = 0u32;
        // (rate, count) grid: edges, milli-boundaries, randoms.
        let mut pairs: Vec<(u32, u32)> = vec![];
        for &r in &U32_EDGE {
            for &c in &U32_EDGE {
                pairs.push((r, c));
            }
        }
        for &(r, c) in &[
            (1000, 1),
            (1500, 1),
            (1, 1000),
            (1, 1500),
            (999, 1000),
            (1_000_000, 1000),
            (0xFFFF_FFFF, 1000),
            (1000, 0xFFFF_FFFF),
            (123456, 789),
        ] {
            pairs.push((r, c));
        }
        for _ in 0..120 {
            pairs.push((rng.u32(), rng.count()));
        }
        for (rate, count) in pairs {
            for flags in [0x00u8, 0x10, 0x1F, rng.u8()] {
                let mut fx = Fixture::build(&mut rng);
                let divisor = 1 + (cases % 2);
                let cursor = cases % divisor;
                fx.obj.w32(DIV, divisor);
                fx.obj.w32(CURSOR, cursor);
                fx.obj.w32(RATE, rate);
                fx.obj.w8(FLAGS, flags);
                let aux = fx.obj.r32(AUX);
                // Source block: 14 words; bias is word 3 (+0xC).
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
                let refined = rng.u32();
                let took = if cases % 4 == 0 {
                    0
                } else if cases % 4 == 1 {
                    1 + rng.u32() % 100
                } else if cases % 4 == 2 {
                    0x8000_0000 | (rng.u32() % 100)
                } else {
                    rng.u32()
                };
                let synth = count != 0 && flags & 0x10 != 0;
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(divisor as usize);
                rt::set_script(&[
                    (1, StubKind::Cdecl2, vec![refined]),
                    (2, StubKind::Thiscall2, vec![took]),
                ]);
                let mut fake = Fake::new();
                fake.answer("soft.refine", vec![refined]);
                fake.answer("soft.consume", vec![took]);
                let got =
                    unsafe { fn_0088D960::rw_88d960(fx.obj.buf.as_mut_ptr(), count) };
                let want = v.refresh_lane(&mut fake, &src_words, src_bias, count);
                assert_eq!(got, want, "rate {rate:#x} count {count:#x} flags {flags:#x}");
                // Lanes agree word for word.
                for k in 0..divisor as usize {
                    let base = LANE + k * 64;
                    for w in 0..14 {
                        assert_eq!(fx.obj.r32(base + w * 4), v.lanes[k].params[w]);
                    }
                    assert_eq!(fx.obj.r32(base + 0x38), v.lanes[k].pos);
                    assert_eq!(fx.obj.r32(base + 0x3C), v.lanes[k].rem);
                }
                assert_eq!(fx.obj.r32(TUNING), v.tuning);
                assert_eq!(fx.obj.r32(CURSOR), v.cursor);
                let mut changed = vec![(LANE + (cursor as usize) * 64, 64), (CURSOR, 4)];
                if synth {
                    changed.push((TUNING, 4));
                    check_calls(
                        rt::take_numbered(),
                        std::mem::take(&mut fake.log),
                        vec![
                            (1, vec![count, rate], "soft.refine", vec![count, rate]),
                            (2, vec![aux, count], "soft.consume", vec![aux, count]),
                        ],
                    );
                } else {
                    assert!(rt::take_numbered().is_empty());
                    assert!(fake.log.is_empty());
                }
                assert_only_changed(&before, &fx.obj.buf, &changed);
                // Source block untouched by both sides.
                for k in 0..14 {
                    assert_eq!(src.r32(k * 4), src_words[k]);
                }
                let lane_base = LANE + (cursor as usize) * 64;
                let pos = fx.obj.r32(lane_base + 0x38);
                let rem = fx.obj.r32(lane_base + 0x3C);
                if wrong::lane_rem_add(src_bias, pos) != rem {
                    caught += 1;
                }
                let next = cursor.wrapping_add(1);
                if wrong::lane_quotient(next, divisor) != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases > 500, "too few cases: {cases}");
        assert!(caught > 0, "wrong lane never caught");
    }
}
