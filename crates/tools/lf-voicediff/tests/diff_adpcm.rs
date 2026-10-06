//! Differential cases: the ADPCM DirectSound voice.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::voice::dsound_adpcm::{AdpcmLane, AdpcmVoice};
    use lf_voicediff::rewrites::*;
    use lf_voicediff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        FLAG_EDGE, Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls,
        check_virtual, cookie,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const PARAMS: usize = 0x04;
    const RATE: usize = 0x0C;
    const CODEC: usize = 0x10;
    const SRC: usize = 0x14;
    const FLAGS: usize = 0x8C;
    const DEV: usize = 0x90;
    const OUT_W: usize = 0x9E;
    const OUT_B: usize = 0xA0;
    const BASE: usize = 0xA4;
    const LIMIT: usize = 0xB4;
    const COMBINE: usize = 0xC4;
    const STORED: usize = 0xCC;
    const FREQ: usize = 0xD0;
    const LEA_BASE: usize = 0xDC;
    const CURSOR: usize = 0xF4;
    const DIV: usize = 0xFC;
    const OBJ_SIZE: usize = 0x200;

    /// Lane copy base for lane k: (k + 4) * 64.
    fn lane_copy(k: usize) -> usize {
        (k + 4) * 64
    }

    /// Accumulator offset for lane k: k * 64 + 0x138.
    fn lane_acc(k: usize) -> usize {
        k * 64 + 0x138
    }

    /// Remainder offset for lane k: k * 64 + 0x13C.
    fn lane_rem(k: usize) -> usize {
        k * 64 + 0x13C
    }

    /// A test voice: the 32-bit image plus the collaborator blocks it
    /// points at. Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        params: Image,
        dev_obj: Image,
        dev_vtable: VTable,
        vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let params = Image::random(0x20, rng);
            let mut dev_obj = Image::random(0x10, rng);
            let mut dev_vtable = VTable::random(0x20, rng);
            let mut vtable = VTable::random(0x10, rng);
            let stubs = Stubs::new();
            dev_vtable.set(0x10, stubs.cursor);
            dev_vtable.set(0x34, stubs.channel);
            vtable.set(0x18, stubs.gate);
            dev_obj.w32(0, dev_vtable.addr());
            obj.w32(VT, vtable.addr());
            obj.w32(PARAMS, params.addr());
            obj.w32(DEV, dev_obj.addr());
            Self {
                obj,
                params,
                dev_obj,
                dev_vtable,
                vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted voice owning the same words the image holds.
        fn lift(&self, lanes: usize, codec_table: Vec<u32>, predictor: Vec<u8>) -> AdpcmVoice {
            let mut vl = Vec::with_capacity(lanes);
            for k in 0..lanes {
                let mut p = [0u32; 14];
                for (i, w) in p.iter_mut().enumerate() {
                    *w = self.obj.r32(lane_copy(k) + i * 4);
                }
                vl.push(AdpcmLane {
                    params: p,
                    acc: self.obj.r32(lane_acc(k)),
                    rem: self.obj.r32(lane_rem(k)),
                });
            }
            AdpcmVoice {
                flags: self.obj.r8(FLAGS),
                rate: self.obj.r32(RATE),
                codec: cookie(self.obj.r32(CODEC)),
                device: cookie(self.obj.r32(DEV)),
                level: self.params.r32(0),
                base_len: self.obj.r32(BASE),
                limit: self.obj.r32(LIMIT),
                combine: self.obj.r32(COMBINE),
                stored_rate: self.obj.r32(STORED),
                freq: self.obj.r32(FREQ),
                cursor: self.obj.r32(CURSOR),
                divisor: self.obj.r32(DIV),
                lanes: vl,
                codec_table,
                predictor,
                out_word: u16::from_le_bytes([self.obj.r8(OUT_W), self.obj.r8(OUT_W + 1)]),
                out_byte: self.obj.r8(OUT_B),
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        /// Reads the accumulator word instead of the remainder.
        pub fn quiet_acc(acc: u32) -> bool {
            acc == 0
        }

        /// Scales without the final halving.
        pub fn position_scaled(freq: u32, ans: u32, bias: u32) -> u32 {
            ((freq >> 1) << 17).wrapping_add(ans).wrapping_add(bias)
        }

        /// Adds the accumulator instead of subtracting it.
        pub fn block_rem_add(src_bias: u32, acc: u32) -> u32 {
            src_bias.wrapping_add(acc)
        }

        /// Combines below the limit without doubling.
        pub fn seek_combine(combine: u32, v: u32, base: u32) -> u32 {
            combine.wrapping_add(v).wrapping_sub(base)
        }
    }

    #[test]
    fn ad_quiet_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xAD07);
        let mut caught = 0;
        for i in 0..200u32 {
            let mut fx = Fixture::build(&mut rng);
            let divisor = 1 + (i % 2);
            let cursor = i % divisor;
            fx.obj.w32(DIV, divisor);
            fx.obj.w32(CURSOR, cursor);
            let (acc, rem) = match i % 5 {
                0 => (0, 0),
                1 => (1, 0),
                2 => (0, 1),
                3 => (rng.u32(), rng.u32()),
                _ => (U32_EDGE[(i as usize) % U32_EDGE.len()], 0),
            };
            fx.obj.w32(lane_acc(cursor as usize), acc);
            fx.obj.w32(lane_rem(cursor as usize), rem);
            let before = fx.obj.buf.clone();
            let v = fx.lift(divisor as usize, vec![], vec![]);
            let got = unsafe { fn_0088D310::rw_0088d310(fx.obj.buf.as_ptr()) };
            let want = v.lane_rest_quiet();
            assert_eq!(got, u32::from(want), "rem {rem:#x}");
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            if wrong::quiet_acc(acc) != want {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong quiet never caught");
    }

    #[test]
    fn ad_position_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xAD10);
        let mut caught = 0;
        let gates = [0u32, 1, 0x100, 0xFF, 0xFF00, 0x12345600, 0xFFFFFFFF, 3];
        let mut i = 0u32;
        for &gate in &gates {
            for _ in 0..12 {
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w32(FREQ, U32_EDGE[(i as usize) % U32_EDGE.len()]);
                fx.obj.w32(STORED, rng.u32());
                fx.obj.w32(RATE, rng.u32());
                let dev = fx.obj.r32(DEV);
                let freq = fx.obj.r32(FREQ);
                let stored = fx.obj.r32(STORED);
                let base = fx.obj.r32(RATE);
                let cursor = U32_EDGE[(i as usize) % U32_EDGE.len()];
                let pos_ans = rng.u32();
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1, vec![], vec![]);
                rt::set_script(&[(3, StubKind::Cdecl2, vec![pos_ans])]);
                rt::set_virtual(&[("gate", vec![gate]), ("cursor.out", vec![cursor])]);
                let mut fake = Fake::new();
                fake.answer("ad.gate", vec![gate]);
                fake.answer("ad.cursor", vec![cursor]);
                fake.answer("ad.pos", vec![pos_ans]);
                let got = unsafe { fn_0088CEA0::rw_0088CEA0(fx.this()) };
                let want = v.position(&mut fake);
                assert_eq!(got, want, "gate {gate:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                let mut lift_log = std::mem::take(&mut fake.log);
                assert_eq!(
                    lift_log.remove(0),
                    ("ad.gate".to_string(), vec![]),
                    "lift gate call"
                );
                if gate as u8 == 0 {
                    assert_eq!(got, 0xFFFF_FFFF);
                    check_virtual(rt::take_virtual(), vec![("gate", vec![fx.this()])]);
                    assert!(rt::take_numbered().is_empty());
                    assert!(lift_log.is_empty());
                } else {
                    let scaled =
                        (((freq >> 1) << 17).wrapping_add(cursor) >> 1).wrapping_add(stored);
                    let virt = rt::take_virtual();
                    assert_eq!(virt.len(), 2, "gate + cursor calls");
                    assert_eq!(virt[0], ("gate".to_string(), vec![fx.this()]));
                    assert_eq!(virt[1].0, "cursor");
                    assert_eq!(virt[1].1[0], dev);
                    assert_eq!(virt[1].1[2], 0);
                    assert_eq!(
                        lift_log.remove(0),
                        ("ad.cursor".to_string(), vec![dev]),
                        "lift cursor call"
                    );
                    check_calls(
                        rt::take_numbered(),
                        lift_log,
                        vec![(3, vec![scaled, base], "ad.pos", vec![scaled, base])],
                    );
                    if wrong::position_scaled(freq, cursor, stored) != scaled {
                        caught += 1;
                    }
                }
                i += 1;
            }
        }
        assert!(caught > 0, "wrong position never caught");
    }

    #[test]
    fn ad_queue_block_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xAD08);
        let mut caught = 0;
        let mut cases = 0u32;
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
            let edx0 = ans2.wrapping_mul(2) >> 2;
            let edx1 = (edx0 >> 11) + u32::from(edx0 & 0x7ff != 0);
            let pred_len = (edx1.wrapping_mul(3) as usize) + 3;
            let pred = Image::random(pred_len.max(8), &mut rng);
            fx.obj.w32(LEA_BASE, pred.addr());
            let pred_bytes = pred.buf.to_vec();
            let synth = a0 != 0 && flags & 0x10 != 0;
            let before = fx.obj.buf.clone();
            let mut v = fx.lift(divisor as usize, table_words.clone(), pred_bytes.clone());
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![ans1]),
                (2, StubKind::Cdecl2, vec![ans2]),
            ]);
            let mut fake = Fake::new();
            fake.answer("ad.codec", vec![ans1]);
            fake.answer("ad.rate", vec![ans2]);
            unsafe { fn_0088C950::rw_0088C950(fx.this(), a0) };
            v.queue_block(&mut fake, &src_words, src_bias, a0);
            for k in 0..divisor as usize {
                for w in 0..14 {
                    assert_eq!(fx.obj.r32(lane_copy(k) + w * 4), v.lanes[k].params[w]);
                }
                assert_eq!(fx.obj.r32(lane_acc(k)), v.lanes[k].acc, "trial {trial}");
                assert_eq!(fx.obj.r32(lane_rem(k)), v.lanes[k].rem, "trial {trial}");
            }
            assert_eq!(fx.obj.r32(STORED), v.stored_rate, "trial {trial}");
            assert_eq!(fx.obj.r32(CURSOR), v.cursor, "trial {trial}");
            assert_eq!(
                u16::from_le_bytes([fx.obj.r8(OUT_W), fx.obj.r8(OUT_W + 1)]),
                v.out_word,
                "trial {trial}"
            );
            assert_eq!(fx.obj.r8(OUT_B), v.out_byte, "trial {trial}");
            // The copy, acc and rem are contiguous: 64 bytes a lane.
            let mut changed = vec![(lane_copy(cursor as usize), 64), (CURSOR, 4)];
            if synth {
                changed.push((STORED, 4));
                changed.push((OUT_W, 2));
                changed.push((OUT_B, 1));
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![
                        (
                            1,
                            vec![codec.addr(), a0],
                            "ad.codec",
                            vec![codec.addr(), a0],
                        ),
                        (2, vec![a0, rate], "ad.rate", vec![a0, rate]),
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
            assert_eq!(&pred.buf[..], &pred_bytes[..]);
            let acc = fx.obj.r32(lane_acc(cursor as usize));
            let rem = fx.obj.r32(lane_rem(cursor as usize));
            if wrong::block_rem_add(src_bias, acc) != rem {
                caught += 1;
            }
            cases += 1;
            let _ = (t2, t1, codec, pred);
        }
        assert!(cases > 100, "too few cases: {cases}");
        assert!(caught > 0, "wrong block never caught");
    }

    #[test]
    fn ad_seek_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xAD03);
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
                fx.obj
                    .w32(RATE, U32_EDGE[((i + 1) as usize) % U32_EDGE.len()]);
                let level = rng.u32();
                fx.params.w32(0, level);
                let pos = if i % 4 == 0 {
                    rng.u32() % 3_000_000
                } else {
                    U32_EDGE[((i + 2) as usize) % U32_EDGE.len()]
                };
                let resolve_ans = rng.u32();
                let dev = fx.obj.r32(DEV);
                let base = fx.obj.r32(BASE);
                let limit = fx.obj.r32(LIMIT);
                let combine = fx.obj.r32(COMBINE);
                let rate = fx.obj.r32(RATE);
                let before = fx.obj.buf.clone();
                let mut v = fx.lift(1, vec![], vec![]);
                rt::set_script(&[
                    (1, StubKind::Cdecl2, vec![resolve_ans]),
                    (2, StubKind::Thiscall2, vec![0]),
                    (4, StubKind::Thiscall2, vec![0]),
                ]);
                let mut fake = Fake::new();
                fake.answer("ad.length", vec![resolve_ans]);
                unsafe { fn_0088D330::rw_0088d330(fx.obj.buf.as_mut_ptr(), pos) };
                v.seek(&mut fake, pos);
                assert_eq!(fx.obj.r8(FLAGS), v.flags, "flags {flags:#x} pos {pos:#x}");
                assert_eq!(fx.obj.r8(FLAGS), flags | 8);
                assert_only_changed(&before, &fx.obj.buf, &[(FLAGS, 1)]);
                let helper = flags & 2 != 0 && base != 0;
                // The channel call carries the computed cursor on both sides.
                let virt = rt::take_virtual();
                assert_eq!(virt.len(), 1, "one channel call");
                assert_eq!(virt[0].0, "channel");
                assert_eq!(virt[0].1[0], dev);
                let chan_cursor = virt[0].1[1];
                let mut lift_log = std::mem::take(&mut fake.log);
                let mut saw_channel = None;
                lift_log.retain(|(n, a)| {
                    if n == "ad.channel" {
                        saw_channel = Some(a.clone());
                        false
                    } else {
                        true
                    }
                });
                let saw_channel = saw_channel.expect("lift channel call");
                assert_eq!(saw_channel, vec![dev, chan_cursor], "channel cursor");
                let mut expect = vec![];
                if helper {
                    if pos < limit {
                        expect.push((1, vec![pos, rate], "ad.length", vec![pos, rate]));
                    } else {
                        expect.push((
                            1,
                            vec![pos.wrapping_sub(limit), rate],
                            "ad.length",
                            vec![pos.wrapping_sub(limit), rate],
                        ));
                    }
                }
                if flags & 0x10 != 0 {
                    expect.push((2, vec![fx.this(), 1], "ad.refill", vec![1]));
                }
                expect.push((4, vec![fx.this(), level], "ad.gain", vec![level]));
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
}
