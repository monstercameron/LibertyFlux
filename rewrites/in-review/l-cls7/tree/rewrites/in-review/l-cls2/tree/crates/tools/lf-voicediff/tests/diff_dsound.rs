//! Differential cases: the DirectSound voice.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::voice::dsound::{DSoundLane, DSoundVoice};
    use lf_voicediff::rewrites::*;
    use lf_voicediff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls,
        check_virtual, cookie,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const PARAMS: usize = 0x04;
    const RATE: usize = 0x0C;
    const SRC: usize = 0x14;
    const FLAGS: usize = 0x8C;
    const DEV_A: usize = 0x90;
    const DEV_B: usize = 0x94;
    const RESTART: usize = 0x98;
    const BASE: usize = 0xA0;
    const LIMIT: usize = 0xB0;
    const COMBINE: usize = 0xC0;
    const CACHED: usize = 0xC8;
    const FREQ: usize = 0xCC;
    const CURSOR: usize = 0xE0;
    const DIV: usize = 0xE8;
    const LANE: usize = 0xEC;
    const OBJ_SIZE: usize = 0x1C0;

    /// A test voice: the 32-bit image plus the collaborator blocks it
    /// points at. Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        params: Image,
        dev_a_obj: Image,
        dev_b_obj: Image,
        dev_vtable: VTable,
        vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let params = Image::random(0x20, rng);
            let mut dev_a_obj = Image::random(0x10, rng);
            let mut dev_b_obj = Image::random(0x10, rng);
            let mut dev_vtable = VTable::random(0x20, rng);
            let mut vtable = VTable::random(0x10, rng);
            let stubs = Stubs::new();
            dev_vtable.set(0x08, stubs.release);
            dev_vtable.set(0x10, stubs.cursor);
            dev_vtable.set(0x24, stubs.seek);
            dev_vtable.set(0x30, stubs.resume);
            dev_vtable.set(0x34, stubs.channel);
            vtable.set(0x18, stubs.gate);
            dev_a_obj.w32(0, dev_vtable.addr());
            dev_b_obj.w32(0, dev_vtable.addr());
            obj.w32(VT, vtable.addr());
            obj.w32(PARAMS, params.addr());
            obj.w32(DEV_A, dev_a_obj.addr());
            obj.w32(DEV_B, dev_b_obj.addr());
            Self {
                obj,
                params,
                dev_a_obj,
                dev_b_obj,
                dev_vtable,
                vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted voice owning the same words the image holds.
        fn lift(&self, lanes: usize) -> DSoundVoice {
            let mut vl = Vec::with_capacity(lanes);
            for k in 0..lanes {
                let base = LANE + k * 64;
                let mut p = [0u32; 14];
                for (i, w) in p.iter_mut().enumerate() {
                    *w = self.obj.r32(base + i * 4);
                }
                vl.push(DSoundLane {
                    window: p,
                    even: self.obj.r32(k * 64 + 0x124),
                    rest: self.obj.r32(k * 64 + 0x128),
                });
            }
            DSoundVoice {
                flags: self.obj.r8(FLAGS),
                rate: self.obj.r32(RATE),
                restart_pos: self.obj.r32(RESTART),
                params_status: self.params.buf[0x18],
                device_a: cookie(self.obj.r32(DEV_A)),
                device_b: cookie(self.obj.r32(DEV_B)),
                cached_voice: self.obj.r32(CACHED),
                freq: self.obj.r32(FREQ),
                base_len: self.obj.r32(BASE),
                limit: self.obj.r32(LIMIT),
                combine: self.obj.r32(COMBINE),
                level: self.params.r32(0),
                cursor: self.obj.r32(CURSOR),
                divisor: self.obj.r32(DIV),
                lanes: vl,
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        /// Drops the resume-pending half of the first arm.
        pub fn stopping_first(flags: u8) -> bool {
            flags & 0x01 != 0
        }

        /// Forgets nothing but reads the wrong lane word (even, not rest).
        pub fn quiet_even(even: u32) -> bool {
            even == 0
        }

        /// Sets the loop flag from the wrong bits: bit 0 or bit 3.
        pub fn resume_loop(flags: u8) -> bool {
            flags & 0x09 != 0
        }

        /// Scales without the final halving.
        pub fn position_scaled(freq: u32, ans: u32, bias: u32) -> u32 {
            ((freq >> 1) << 17).wrapping_add(ans).wrapping_add(bias)
        }

        /// Adds the derived count instead of subtracting it.
        pub fn slot_rest_add(src_bias: u32, even: u32) -> u32 {
            src_bias.wrapping_add(even)
        }

        /// Combines below the limit without doubling.
        pub fn seek_combine(combine: u32, v: u32, base: u32) -> u32 {
            combine.wrapping_add(v).wrapping_sub(base)
        }

        /// Releases only the primary device.
        pub fn teardown_single(dev_a: bool) -> u32 {
            u32::from(dev_a)
        }
    }

    #[test]
    fn ds_stopping_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD506);
        let mut caught = 0;
        for flags in 0..=255u8 {
            let mut fx = Fixture::build(&mut rng);
            fx.obj.w8(FLAGS, flags);
            // Restart low bit and params stopping bit sweep with flags.
            let restart = if flags as u32 % 3 == 0 {
                rng.u32() | 1
            } else {
                rng.u32() & !1
            };
            fx.obj.w32(RESTART, restart);
            fx.params.buf[0x18] = if flags as u32 % 5 == 0 { 0x40 } else { 0 };
            let v = fx.lift(1);
            let before = fx.obj.buf.clone();
            let got = unsafe { fn_0088D2E0::rw_0088d2e0(fx.obj.buf.as_ptr()) };
            let want = v.is_stopping();
            assert_eq!(got, u8::from(want), "flags {flags:#x}");
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            // Wrong first arm disagrees when bit 3 shows without bit 0
            // and no other arm fires.
            let others = restart & 1 != 0 || fx.params.buf[0x18] & 0x40 != 0;
            if !others && wrong::stopping_first(flags) != (flags & 0x09 != 0) {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong stopping never caught");
    }

    #[test]
    fn ds_quiet_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD507);
        let mut caught = 0;
        for i in 0..200u32 {
            let mut fx = Fixture::build(&mut rng);
            let divisor = 1 + (i % 2);
            let cursor = i % divisor;
            fx.obj.w32(DIV, divisor);
            fx.obj.w32(CURSOR, cursor);
            let (even, rest) = match i % 5 {
                0 => (0, 0),
                1 => (1, 0),
                2 => (0, 1),
                3 => (rng.u32(), rng.u32()),
                _ => (U32_EDGE[(i as usize) % U32_EDGE.len()], 0),
            };
            fx.obj.w32((cursor as usize) * 64 + 0x124, even);
            fx.obj.w32((cursor as usize) * 64 + 0x128, rest);
            let before = fx.obj.buf.clone();
            let v = fx.lift(divisor as usize);
            let got = unsafe { fn_0088C320::rw_0088c320(fx.obj.buf.as_ptr()) };
            let want = v.slot_rest_quiet();
            assert_eq!(got, u32::from(want), "rest {rest:#x}");
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            if wrong::quiet_even(even) != want {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong quiet never caught");
    }

    #[test]
    fn ds_resume_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD504);
        let mut caught = 0;
        for i in 0..240u32 {
            let mut fx = Fixture::build(&mut rng);
            let flags = if i < 64 { (i % 32) as u8 } else { rng.u8() };
            fx.obj.w8(FLAGS, flags);
            let dev_a = fx.obj.r32(DEV_A);
            let restart = fx.obj.r32(RESTART);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1);
            let mut fake = Fake::new();
            unsafe { fn_0088D570::rw_0088D570(fx.this()) };
            v.resume(&mut fake);
            assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x}");
            let active = flags & 8 != 0 && flags & 1 == 0;
            if active {
                assert_eq!(fx.obj.r8(FLAGS), flags & !8);
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                let looping = flags & 0x12 != 0;
                check_virtual(
                    rt::take_virtual(),
                    vec![
                        ("resume", vec![dev_a, 0, 0, u32::from(looping)]),
                        ("seek", vec![dev_a, fx.this().wrapping_add(RESTART as u32)]),
                    ],
                );
                // The lift passes the restart value where the rewrite
                // passes its address; the recorded address reads back
                // the same value.
                let pos_addr = fx.this().wrapping_add(RESTART as u32);
                let back = unsafe { (pos_addr as *const u32).read_unaligned() };
                assert_eq!(back, restart);
                assert_eq!(
                    std::mem::take(&mut fake.log),
                    vec![
                        ("ds.resume".to_string(), vec![dev_a, u32::from(looping)]),
                        ("ds.seek".to_string(), vec![dev_a, restart]),
                    ],
                    "lift device calls"
                );
            } else {
                assert_eq!(fx.obj.r8(FLAGS), flags);
                assert_only_changed(&before, &fx.obj.buf, &[]);
                assert!(rt::take_virtual().is_empty());
                assert!(fake.log.is_empty());
            }
            if active && wrong::resume_loop(flags) != (flags & 0x12 != 0) {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong resume never caught");
    }

    #[test]
    fn ds_position_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD510);
        let mut caught = 0;
        let gates = [0u32, 1, 0x100, 0xFF, 0xFF00, 0x12345600, 0xFFFFFFFF, 3];
        let mut i = 0u32;
        for &gate in &gates {
            for _ in 0..12 {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w32(FREQ, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.obj.w32(CACHED, rng.u32());
                fx.obj.w32(RATE, rng.u32());
                let dev_a = fx.obj.r32(DEV_A);
                let freq = fx.obj.r32(FREQ);
                let cached = fx.obj.r32(CACHED);
                let base = fx.obj.r32(RATE);
                let cursor = U32_EDGE[(i as usize) % U32_EDGE.len()];
                let pos_ans = rng.u32();
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1);
                rt::set_script(&[(3, StubKind::Cdecl2, vec![pos_ans])]);
                rt::set_virtual(&[("gate", vec![gate]), ("cursor.out", vec![cursor])]);
                let mut fake = Fake::new();
                fake.answer("ds.gate", vec![gate]);
                fake.answer("ds.cursor", vec![cursor]);
                fake.answer("ds.pos", vec![pos_ans]);
                let got = unsafe { fn_0088BF80::rw_0088BF80(fx.this()) };
                let want = v.position(&mut fake);
                assert_eq!(got, want, "gate {gate:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                let mut lift_log = std::mem::take(&mut fake.log);
                assert_eq!(
                    lift_log.remove(0),
                    ("ds.gate".to_string(), vec![]),
                    "lift gate call"
                );
                if gate as u8 == 0 {
                    assert_eq!(got, 0xFFFF_FFFF);
                    check_virtual(rt::take_virtual(), vec![("gate", vec![fx.this()])]);
                    assert!(rt::take_numbered().is_empty());
                    assert!(lift_log.is_empty());
                } else {
                    let scaled =
                        (((freq >> 1) << 17).wrapping_add(cursor) >> 1).wrapping_add(cached);
                    // The cursor stub's out-word is the rewrite's frame
                    // address; compare around it.
                    let virt = rt::take_virtual();
                    assert_eq!(virt.len(), 2, "gate + cursor calls");
                    assert_eq!(virt[0], ("gate".to_string(), vec![fx.this()]));
                    assert_eq!(virt[1].0, "cursor");
                    assert_eq!(virt[1].1[0], dev_a);
                    assert_eq!(virt[1].1[2], 0);
                    // The lift-side cursor call has no numbered twin.
                    assert_eq!(
                        lift_log.remove(0),
                        ("ds.cursor".to_string(), vec![dev_a]),
                        "lift cursor call"
                    );
                    check_calls(
                        rt::take_numbered(),
                        lift_log,
                        vec![(3, vec![scaled, base], "ds.pos", vec![scaled, base])],
                    );
                    if wrong::position_scaled(freq, cursor, cached) != scaled {
                        caught += 1;
                    }
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong position never caught");
    }

    #[test]
    fn ds_refresh_slot_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD508);
        let mut caught = 0;
        let mut cases = 0u32;
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
            (999, 1000),
            (1_000_000, 1000),
            (0xFFFF_FFFF, 1000),
            (1000, 0xFFFF_FFFF),
            (123456, 789),
        ] {
            pairs.push((r, c));
        }
        for _ in 0..60 {
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
                let voice = rng.u32();
                let used = if cases % 4 == 0 {
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
                    (1, StubKind::Cdecl2, vec![voice]),
                    (2, StubKind::Stdcall1, vec![used]),
                ]);
                let mut fake = Fake::new();
                fake.answer("ds.resolve", vec![voice]);
                fake.answer("ds.consume", vec![used]);
                let got = unsafe { fn_0088B8F0::rw_0088b8f0(fx.this(), count) };
                let want = v.refresh_slot(&mut fake, &src_words, src_bias, count);
                assert_eq!(got, want, "rate {rate:#x} count {count:#x} flags {flags:#x}");
                for k in 0..divisor as usize {
                    let base = LANE + k * 64;
                    for w in 0..14 {
                        assert_eq!(fx.obj.r32(base + w * 4), v.lanes[k].window[w]);
                    }
                    assert_eq!(fx.obj.r32(k * 64 + 0x124), v.lanes[k].even);
                    assert_eq!(fx.obj.r32(k * 64 + 0x128), v.lanes[k].rest);
                }
                assert_eq!(fx.obj.r32(CACHED), v.cached_voice);
                assert_eq!(fx.obj.r32(CURSOR), v.cursor);
                let mut changed =
                    vec![(LANE + (cursor as usize) * 64, 56 + 8), (CURSOR, 4)];
                if synth {
                    changed.push((CACHED, 4));
                    check_calls(
                        rt::take_numbered(),
                        std::mem::take(&mut fake.log),
                        vec![
                            (1, vec![count, rate], "ds.resolve", vec![count, rate]),
                            (2, vec![count], "ds.consume", vec![count]),
                        ],
                    );
                } else {
                    assert!(rt::take_numbered().is_empty());
                    assert!(fake.log.is_empty());
                }
                assert_only_changed(&before, &fx.obj.buf, &changed);
                for k in 0..14 {
                    assert_eq!(src.r32(k * 4), src_words[k]);
                }
                let even = fx.obj.r32((cursor as usize) * 64 + 0x124);
                let rest = fx.obj.r32((cursor as usize) * 64 + 0x128);
                if wrong::slot_rest_add(src_bias, even) != rest {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases > 400, "too few cases: {cases}");
        assert!(caught > 0, "wrong slot never caught");
    }

    #[test]
    fn ds_seek_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD503);
        let mut caught = 0;
        let mut i = 0u32;
        for flags in [0x00u8, 0x02, 0x10, 0x12, 0x1F, 0xFF, rng.u8(), rng.u8()] {
            for _ in 0..30 {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w8(FLAGS, flags);
                let base_zero = i % 3 == 0;
                fx.obj.w32(BASE, if base_zero { 0 } else { rng.u32() });
                fx.obj.w32(LIMIT, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.obj.w32(COMBINE, rng.u32());
                fx.obj.w32(RATE, U32_EDGE[((i + 1) as usize) % U32_EDGE.len()]);
                let level = rng.u32();
                fx.params.w32(0, level);
                let pos = U32_EDGE[((i + 2) as usize) % U32_EDGE.len()];
                let resolve_ans = rng.u32();
                let finish_ans = rng.u32();
                let dev_a = fx.obj.r32(DEV_A);
                let base = fx.obj.r32(BASE);
                let limit = fx.obj.r32(LIMIT);
                let combine = fx.obj.r32(COMBINE);
                let rate = fx.obj.r32(RATE);
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1);
                rt::set_script(&[
                    (1, StubKind::Cdecl2, vec![resolve_ans]),
                    (2, StubKind::Thiscall2, vec![0]),
                    (4, StubKind::Thiscall2, vec![finish_ans]),
                ]);
                let mut fake = Fake::new();
                fake.answer("ds.length", vec![resolve_ans]);
                fake.answer("ds.finish", vec![finish_ans]);
                let got = unsafe { fn_0088C340::rw_0088c340(fx.this(), pos) };
                let want = v.seek(&mut fake, pos);
                assert_eq!(got, want, "flags {flags:#x} pos {pos:#x}");
                assert_eq!(fx.obj.r8(FLAGS), v.flags);
                assert_eq!(fx.obj.r8(FLAGS), flags | 8);
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                let helper = flags & 2 != 0 && base != 0;
                // The channel call carries the computed length on both sides.
                let virt = rt::take_virtual();
                assert_eq!(virt.len(), 1, "one channel call");
                assert_eq!(virt[0].0, "channel");
                assert_eq!(virt[0].1[0], dev_a);
                let chan_len = virt[0].1[1];
                let mut lift_log = std::mem::take(&mut fake.log);
                // Lift the channel call out: it has no numbered twin.
                let mut saw_channel = None;
                lift_log.retain(|(n, a)| {
                    if n == "ds.channel" {
                        saw_channel = Some(a.clone());
                        false
                    } else {
                        true
                    }
                });
                let saw_channel = saw_channel.expect("lift channel call");
                assert_eq!(saw_channel, vec![dev_a, chan_len], "channel length");
                let mut expect = vec![];
                if helper {
                    if pos >= limit {
                        expect.push((
                            1,
                            vec![pos.wrapping_sub(limit), rate],
                            "ds.length",
                            vec![pos.wrapping_sub(limit), rate],
                        ));
                    } else {
                        expect.push((1, vec![pos, rate], "ds.length", vec![pos, rate]));
                    }
                }
                if flags & 0x10 != 0 {
                    expect.push((2, vec![fx.this(), 1], "ds.notify", vec![1]));
                }
                expect.push((4, vec![fx.this(), level], "ds.finish", vec![level]));
                check_calls(rt::take_numbered(), lift_log, expect);
                if helper
                    && pos < limit
                    && wrong::seek_combine(combine, resolve_ans, base)
                        != combine
                            .wrapping_add(resolve_ans.wrapping_mul(2))
                            .wrapping_sub(base)
                {
                    caught += 1;
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong seek never caught");
    }

    #[test]
    fn ds_teardown_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xD502);
        let mut caught = 0;
        for i in 0..120u32 {
            let mut fx = Fixture::build(&mut rng);
            // Each device slot null or present.
            let null_a = i % 2 == 0;
            let null_b = i % 3 == 0;
            if null_a {
                fx.obj.w32(DEV_A, 0);
            }
            if null_b {
                fx.obj.w32(DEV_B, 0);
            }
            let dev_a = fx.obj.r32(DEV_A);
            let dev_b = fx.obj.r32(DEV_B);
            let base_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(1);
            rt::set_script(&[(2, StubKind::Thiscall1, vec![base_ans])]);
            let mut fake = Fake::new();
            fake.answer("ds.base", vec![base_ans]);
            let got = unsafe { fn_0088C570::rw_0088C570(fx.this()) };
            let want = v.teardown(&mut fake);
            assert_eq!(got, want, "null_a {null_a} null_b {null_b}");
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let mut expect_v = vec![];
            if !null_a {
                expect_v.push(("release", vec![dev_a]));
            }
            if !null_b {
                expect_v.push(("release", vec![dev_b]));
            }
            check_virtual(rt::take_virtual(), expect_v);
            // Lift log: one release per present device, then base.
            let mut lift_log = std::mem::take(&mut fake.log);
            let mut releases = 0;
            lift_log.retain(|(n, _)| {
                if n == "ds.release" {
                    releases += 1;
                    false
                } else {
                    true
                }
            });
            check_calls(
                rt::take_numbered(),
                lift_log,
                vec![(2, vec![fx.this()], "ds.base", vec![])],
            );
            let want_releases = u32::from(!null_a) + u32::from(!null_b);
            assert_eq!(releases, want_releases, "lift releases");
            if wrong::teardown_single(!null_a) != want_releases {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong teardown never caught");
    }
}
