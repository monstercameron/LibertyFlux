//! Differential cases: the reverb effect.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::sound::reverb::ReverbEffect;
    use lf_sounddiff::rewrites::*;
    use lf_sounddiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls, check_virtual,
        cookie,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const INFO: usize = 0x04;
    const NEXT: usize = 0x08;
    const POLL_INDEX: usize = 0x2C;
    const STEP: usize = 0x30;
    const CHANS: usize = 0x74;
    const ROWS: usize = 3;
    const WIDTH: usize = 5;
    const STAGING: usize = 0xB0;
    const HOLD: usize = 0xC4;
    const SUB: usize = 0xC8;
    const OBJ_SIZE: usize = 0xD0;
    // Unaligned preset word offsets inside the info block.
    const PRESET_OFFS: [usize; 4] = [0x0F, 0x13, 0x17, 0x1B];

    /// A test reverb: the 32-bit image plus the blocks it points at.
    /// Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        preset: Image,
        self_vtable: VTable,
        next_obj: Image,
        next_vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let preset = Image::random(0x20, rng);
            let mut self_vtable = VTable::random(8, rng);
            let mut next_obj = Image::random(0x10, rng);
            let mut next_vtable = VTable::random(8, rng);
            let stubs = Stubs::new();
            self_vtable.set(0x14, stubs.hook);
            next_vtable.set(0x14, stubs.next);
            next_obj.w32(0, next_vtable.addr());
            obj.w32(VT, self_vtable.addr());
            obj.w32(INFO, preset.addr());
            Self {
                obj,
                preset,
                self_vtable,
                next_obj,
                next_vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted reverb owning the same words the image holds.
        fn lift(&self) -> ReverbEffect {
            let mut chans = [[0u32; WIDTH]; ROWS];
            for (r, row) in chans.iter_mut().enumerate() {
                for (c, w) in row.iter_mut().enumerate() {
                    *w = self.obj.r32(CHANS + r * WIDTH * 4 + c * 4);
                }
            }
            let mut staging = [0u32; 4];
            for (i, w) in staging.iter_mut().enumerate() {
                *w = self.obj.r32(STAGING + i * 4);
            }
            ReverbEffect {
                info: cookie(self.obj.r32(INFO)),
                next: cookie(self.obj.r32(NEXT)),
                poll_index: self.obj.r32(POLL_INDEX),
                step: self.obj.r32(STEP),
                chans,
                staging,
                hold: self.obj.r8(HOLD),
                sub: cookie(self.obj.r32(SUB)),
            }
        }

        /// The four preset words the block holds at their unaligned offsets.
        fn preset_words(&self) -> [u32; 4] {
            let mut out = [0u32; 4];
            for (i, w) in out.iter_mut().enumerate() {
                let o = PRESET_OFFS[i];
                *w = u32::from_le_bytes(self.preset.buf[o..o + 4].try_into().unwrap());
            }
            out
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::{ROWS, ReverbEffect, WIDTH};

        /// Refreshes with a lax comparison (`floor >= mem`), storing on
        /// ties the original skips.
        pub fn advance_lax(
            v: &ReverbEffect,
            preset: &[u32; 4],
            floor: f32,
        ) -> (u32, [[u32; WIDTH]; ROWS]) {
            let mut c = v.clone();
            let slot = c.step.wrapping_add(1) % 3;
            let from = c.step as usize;
            c.chans[slot as usize] = c.chans[from];
            let mut last_stored = false;
            for (k, word) in preset.iter().enumerate() {
                let mem = f32::from_bits(c.chans[from][k]);
                let store = floor >= mem || c.hold == 0;
                if store {
                    c.chans[from][k] = *word;
                }
                if k == 3 {
                    last_stored = store;
                }
            }
            let tail = if last_stored {
                c.chans[from][3]
            } else {
                c.step
            };
            (tail, c.chans)
        }

        /// Answers the next row instead of the current one.
        pub fn slot_next(v: &ReverbEffect) -> [u32; WIDTH] {
            v.chans[(v.step.wrapping_add(1) % 3) as usize]
        }

        /// Fans out to the channels but skips the staging row.
        pub fn init_no_staging(
            v: &ReverbEffect,
            preset: &[u32; 4],
        ) -> ([[u32; WIDTH]; ROWS], [u32; 4]) {
            let mut c = v.clone();
            for row in &mut c.chans {
                row[0] = preset[0];
                row[1] = preset[1];
                row[2] = preset[2];
                row[3] = preset[3];
            }
            (c.chans, c.staging)
        }

        /// Selects the poll row from the step index.
        pub fn poll_slot(v: &ReverbEffect) -> u32 {
            v.step.wrapping_mul(5).wrapping_add(29)
        }
    }

    /// Edge float bit patterns: signed zeros, ones, infinities, NaNs
    /// with several payloads, subnormals, the 2^23 bias.
    const FLOOR_BITS: [u32; 12] = [
        0x0000_0000,
        0x8000_0000,
        0x3F80_0000,
        0xBF80_0000,
        0x7F80_0000,
        0xFF80_0000,
        0x7FC0_0000,
        0x7FFF_FFFF,
        0xFFC0_0001,
        0x0000_0001,
        0x4B00_0000,
        0x3A83_126F,
    ];

    #[test]
    fn reverb_advance_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xAD0A);
        let mut caught = 0;
        let mut cases = 0;
        for i in 0..240u32 {
            let mut fx = Fixture::build(&mut rng);
            let step = i % 3;
            let hold = match i % 4 {
                0 => 0,
                1 => 1,
                2 => 0xFF,
                _ => rng.u8(),
            };
            let floor = f32::from_bits(if i % 3 == 0 {
                FLOOR_BITS[(i as usize) % FLOOR_BITS.len()]
            } else {
                rng.u32()
            });
            let linked = i % 2 == 0;
            let next = if linked { fx.next_obj.addr() } else { 0 };
            fx.obj.w32(STEP, step);
            fx.obj.w8(HOLD, hold);
            fx.obj.w32(NEXT, next);
            let next_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let preset = fx.preset_words();
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            unsafe {
                rt::FLOOR_CELL = floor;
            }
            rt::set_script(&[(1, StubKind::Thiscall1, vec![0x2222_3333])]);
            rt::set_virtual(&[("next", vec![next_ans])]);
            let mut fake = Fake::new();
            fake.answer("rv.next", vec![next_ans]);
            let got = unsafe { fn_008ac240::rw_008AC240(fx.obj.buf.as_mut_ptr()) };
            let want = v.advance(&mut fake, &preset, floor);
            assert_eq!(got, want, "step {step} hold {hold:#x} floor {floor:?}");
            for r in 0..ROWS {
                for c in 0..WIDTH {
                    assert_eq!(
                        fx.obj.r32(CHANS + r * WIDTH * 4 + c * 4),
                        v.chans[r][c],
                        "row {r} col {c}"
                    );
                }
            }
            for s in 0..4 {
                assert_eq!(fx.obj.r32(STAGING + s * 4), v.staging[s], "staging {s}");
            }
            assert_eq!(fx.obj.r32(STEP), (step + 1) % 3);
            assert_eq!(v.step, (step + 1) % 3);
            assert_eq!(fx.obj.r8(HOLD), hold);
            assert_only_changed(&before, &fx.obj.buf, &[(CHANS, 60), (STEP, 4)]);
            assert_eq!(
                rt::take_numbered(),
                vec![(1, vec![fx.this()])],
                "rewrite base call"
            );
            let mut expect_lift = vec![("rv.base".to_string(), vec![])];
            if linked {
                check_virtual(rt::take_virtual(), vec![("next", vec![next])]);
                expect_lift.push(("rv.next".to_string(), vec![next]));
            } else {
                assert!(rt::take_virtual().is_empty());
            }
            assert_eq!(fake.log, expect_lift, "lift calls in order");
            // The wrong run replays the entry state.
            let entry = lift_chans(&before);
            let (wtail, wchans) = wrong::advance_lax(
                &ReverbEffect {
                    chans: entry,
                    step,
                    hold,
                    ..v.clone()
                },
                &preset,
                floor,
            );
            if !linked && (wtail != want || wchans != v.chans) {
                caught += 1;
            }
            cases += 1;
        }
        // Pinned ties: stored words exactly equal to the floor with the
        // hold flag set, which the strict comparison skips.
        for step in 0..3u32 {
            let mut fx = Fixture::build(&mut rng);
            let floor = 1.5f32;
            fx.obj.w32(STEP, step);
            fx.obj.w8(HOLD, 1);
            fx.obj.w32(NEXT, 0);
            for k in 0..4 {
                fx.obj
                    .w32(CHANS + (step as usize) * WIDTH * 4 + k * 4, floor.to_bits());
            }
            let mut preset = fx.preset_words();
            preset[3] = step.wrapping_add(1);
            fx.preset.buf[PRESET_OFFS[3]..PRESET_OFFS[3] + 4]
                .copy_from_slice(&preset[3].to_le_bytes());
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            unsafe {
                rt::FLOOR_CELL = floor;
            }
            rt::set_script(&[(1, StubKind::Thiscall1, vec![0])]);
            let mut fake = Fake::new();
            let got = unsafe { fn_008ac240::rw_008AC240(fx.obj.buf.as_mut_ptr()) };
            let want = v.advance(&mut fake, &preset, floor);
            // Nothing stored: the answer is the step counter.
            assert_eq!(got, step);
            assert_eq!(want, step);
            assert_only_changed(&before, &fx.obj.buf, &[(CHANS, 60), (STEP, 4)]);
            let entry = lift_chans(&before);
            let (wtail, _) = wrong::advance_lax(
                &ReverbEffect {
                    chans: entry,
                    step,
                    hold: 1,
                    ..v.clone()
                },
                &preset,
                floor,
            );
            if wtail != want {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases > 200, "too few cases: {cases}");
        assert!(caught > 0, "wrong advance never caught");
    }

    /// The channel rows of a pre-run image.
    fn lift_chans(before: &[u8]) -> [[u32; WIDTH]; ROWS] {
        let mut chans = [[0u32; WIDTH]; ROWS];
        for (r, row) in chans.iter_mut().enumerate() {
            for (c, w) in row.iter_mut().enumerate() {
                let o = CHANS + r * WIDTH * 4 + c * 4;
                *w = u32::from_le_bytes(before[o..o + 4].try_into().unwrap());
            }
        }
        chans
    }

    #[test]
    fn reverb_channel_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xC4A0);
        let mut caught = 0;
        for i in 0..60u32 {
            let mut fx = Fixture::build(&mut rng);
            let step = i % 3;
            fx.obj.w32(STEP, step);
            let before = fx.obj.buf.clone();
            let v = fx.lift();
            rt::set_script(&[]);
            let got = unsafe { fn_008ac340::rw_008ac340(fx.obj.buf.as_ptr()) };
            assert_eq!(got, fx.this().wrapping_add(step * 20).wrapping_add(0x74));
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            let row = v.channel();
            for (c, word) in row.iter().enumerate() {
                assert_eq!(
                    *word,
                    fx.obj.r32(CHANS + (step as usize) * WIDTH * 4 + c * 4),
                    "col {c}"
                );
            }
            if wrong::slot_next(&v) != *row {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong channel never caught");
    }

    #[test]
    fn reverb_init_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x1A17);
        let mut caught = 0;
        // Early-out answers: the low byte is zero.
        for (i, &ans) in [0u32, 0x100, 0xDEAD_BE00].iter().enumerate() {
            let mut fx = Fixture::build(&mut rng);
            let (a, b) = (rng.u32(), U32_EDGE[i % U32_EDGE.len()]);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[(1, StubKind::Thiscall3, vec![ans])]);
            let mut fake = Fake::new();
            fake.answer("rv.init", vec![ans]);
            let preset = fx.preset_words();
            let got = unsafe { fn_008ac350::rw_008ac350(fx.obj.buf.as_mut_ptr(), a, b) };
            let want = v.init(&mut fake, &preset, a, b);
            assert_eq!(got, ans);
            assert_eq!(want, ans);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![(1, vec![fx.this(), a, b], "rv.init", vec![a, b])],
            );
            assert!(rt::take_virtual().is_empty());
        }
        // Full path: the low byte is nonzero.
        for i in 0..60u32 {
            let mut fx = Fixture::build(&mut rng);
            let (a, b) = (rng.u32(), U32_EDGE[(i as usize) % U32_EDGE.len()]);
            let ans = U32_EDGE[(i as usize) % U32_EDGE.len()] | 1;
            let directs = [
                rng.u32(),
                rng.u32(),
                U32_EDGE[(i as usize) % U32_EDGE.len()],
            ];
            let before = fx.obj.buf.clone();
            // Force the staging row apart from the preset so the wrong
            // version (which skips it) is always caught.
            let mut preset = fx.preset_words();
            let staging_before: Vec<u32> = (0..4).map(|k| fx.obj.r32(STAGING + k * 4)).collect();
            if preset == staging_before.as_slice() {
                preset[0] ^= 1;
                fx.preset.buf[PRESET_OFFS[0]..PRESET_OFFS[0] + 4]
                    .copy_from_slice(&preset[0].to_le_bytes());
            }
            let mut v = fx.lift();
            rt::set_script(&[
                (1, StubKind::Thiscall3, vec![ans]),
                (3, StubKind::Thiscall1, directs.to_vec()),
            ]);
            rt::set_virtual(&[("hook", vec![0xAAAA_BBBB, 0xCCCC_DDDD, 0xEEEE_FFFF])]);
            let mut fake = Fake::new();
            fake.answer("rv.init", vec![ans]);
            fake.answer("rv.direct", directs.to_vec());
            let got = unsafe { fn_008ac350::rw_008ac350(fx.obj.buf.as_mut_ptr(), a, b) };
            let want = v.init(&mut fake, &preset, a, b);
            let expect = (directs[2] & 0xFFFF_FF00) | 1;
            assert_eq!(got, expect, "ans {ans:#x}");
            assert_eq!(want, expect);
            assert_eq!(fx.obj.r8(HOLD), 0);
            assert_eq!(v.hold, 0);
            for r in 0..ROWS {
                for (k, (cell, expect)) in v.chans[r].iter().zip(preset.iter()).enumerate() {
                    assert_eq!(fx.obj.r32(CHANS + r * WIDTH * 4 + k * 4), *expect);
                    assert_eq!(*cell, *expect);
                }
                // The fifth word of each row is untouched.
                let o = CHANS + r * WIDTH * 4 + 16;
                assert_eq!(&before[o..o + 4], &fx.obj.buf[o..o + 4]);
                assert_eq!(
                    v.chans[r][4],
                    u32::from_le_bytes(before[o..o + 4].try_into().unwrap())
                );
            }
            for (k, (cell, expect)) in v.staging.iter().zip(preset.iter()).enumerate() {
                assert_eq!(fx.obj.r32(STAGING + k * 4), *expect);
                assert_eq!(*cell, *expect);
            }
            assert_only_changed(
                &before,
                &fx.obj.buf,
                &[
                    (CHANS, 16),
                    (CHANS + 20, 16),
                    (CHANS + 40, 16),
                    (STAGING, 16),
                    (HOLD, 1),
                ],
            );
            assert_eq!(
                rt::take_numbered(),
                vec![
                    (1, vec![fx.this(), a, b]),
                    (3, vec![fx.this()]),
                    (3, vec![fx.this()]),
                    (3, vec![fx.this()]),
                ],
                "rewrite numbered calls in order"
            );
            check_virtual(
                rt::take_virtual(),
                vec![
                    ("hook", vec![fx.this()]),
                    ("hook", vec![fx.this()]),
                    ("hook", vec![fx.this()]),
                ],
            );
            assert_eq!(
                fake.log,
                vec![
                    ("rv.init".to_string(), vec![a, b]),
                    ("rv.hook".to_string(), vec![]),
                    ("rv.direct".to_string(), vec![]),
                    ("rv.hook".to_string(), vec![]),
                    ("rv.direct".to_string(), vec![]),
                    ("rv.hook".to_string(), vec![]),
                    ("rv.direct".to_string(), vec![]),
                ],
                "lift calls in order"
            );
            let (_, wstaging) = wrong::init_no_staging(&lift_full(&before, &v), &preset);
            if wstaging != v.staging {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong init never caught");
    }

    /// Rebuilds the lift's pre-run rows from the entry image.
    fn lift_full(before: &[u8], v: &ReverbEffect) -> ReverbEffect {
        let mut out = v.clone();
        out.chans = lift_chans(before);
        for (k, w) in out.staging.iter_mut().enumerate() {
            let o = STAGING + k * 4;
            *w = u32::from_le_bytes(before[o..o + 4].try_into().unwrap());
        }
        out
    }

    #[test]
    fn reverb_poll_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9011);
        let mut caught = 0;
        for i in 0..120u32 {
            let mut fx = Fixture::build(&mut rng);
            let poll_index = if i % 3 == 0 {
                U32_EDGE[(i as usize) % U32_EDGE.len()]
            } else {
                rng.u32()
            };
            let step = poll_index.wrapping_add(1 + (i % 5));
            let sub = if i % 2 == 0 {
                rng.u32() & 0xFFFF_FFFC
            } else {
                0
            };
            fx.obj.w32(POLL_INDEX, poll_index);
            fx.obj.w32(STEP, step);
            fx.obj.w32(SUB, sub);
            let post = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![0x1111_2222]),
                (2, StubKind::Thiscall2, vec![0x3333_4444]),
                (3, StubKind::Thiscall1, vec![post]),
            ]);
            let mut fake = Fake::new();
            fake.answer("rv.post", vec![post]);
            let got = unsafe { fn_008ac450::rw_008ac450(fx.obj.buf.as_mut_ptr()) };
            let want = v.poll(&mut fake);
            assert_eq!(got, want);
            assert_eq!(want, post);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let slot = poll_index.wrapping_mul(5).wrapping_add(29);
            let slot_addr = fx.this().wrapping_add(slot.wrapping_mul(4));
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![
                    (1, vec![fx.this()], "rv.pre", vec![]),
                    (2, vec![sub, slot_addr], "rv.sub", vec![sub, slot]),
                    (3, vec![fx.this()], "rv.post", vec![]),
                ],
            );
            if wrong::poll_slot(&v) != slot {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong poll never caught");
    }
}
