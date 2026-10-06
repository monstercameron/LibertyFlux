//! Differential cases: the compressor effect.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::sound::compressor::CompressorEffect;
    use lf_sounddiff::rewrites::*;
    use lf_sounddiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls, check_virtual,
        cookie,
    };

    // Object field offsets, as the verified rewrites use them.
    const LISTENER: usize = 0x08;
    const POLL_INDEX: usize = 0x2C;
    const INDEX: usize = 0x30;
    const SUB: usize = 0x74;
    const PARAMS: usize = 0x78;
    const ROWS: usize = 3;
    const WIDTH: usize = 9;
    const OBJ_SIZE: usize = 0xE8;

    /// A test compressor: the 32-bit image plus the listener block.
    /// Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        listener_obj: Image,
        listener_vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let obj = Image::random(OBJ_SIZE, rng);
            let mut listener_obj = Image::random(0x10, rng);
            let mut listener_vtable = VTable::random(8, rng);
            let stubs = Stubs::new();
            listener_vtable.set(0x14, stubs.notify);
            listener_obj.w32(0, listener_vtable.addr());
            Self {
                obj,
                listener_obj,
                listener_vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted compressor owning the same words the image holds.
        fn lift(&self) -> CompressorEffect {
            let mut params = [[0u32; WIDTH]; ROWS];
            for (r, row) in params.iter_mut().enumerate() {
                for (c, w) in row.iter_mut().enumerate() {
                    *w = self.obj.r32(PARAMS + r * WIDTH * 4 + c * 4);
                }
            }
            CompressorEffect {
                listener: cookie(self.obj.r32(LISTENER)),
                poll_index: self.obj.r32(POLL_INDEX),
                index: self.obj.r32(INDEX),
                sub: cookie(self.obj.r32(SUB)),
                params,
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::{CompressorEffect, ROWS, WIDTH};

        /// Copies the row onto itself, storing the same index.
        pub fn rotate_same(v: &CompressorEffect) -> (u32, [[u32; WIDTH]; ROWS]) {
            (v.params[v.index as usize][WIDTH - 1], v.params)
        }

        /// Answers the next row instead of the current one.
        pub fn slot_next(v: &CompressorEffect) -> [u32; WIDTH] {
            v.params[(v.index.wrapping_add(1) % 3) as usize]
        }

        /// Returns the setter answer unchanged.
        pub fn set_raw(answer: u32) -> u32 {
            answer
        }

        /// Selects the poll row from the rotation index.
        pub fn poll_slot(v: &CompressorEffect) -> u32 {
            v.index.wrapping_mul(9).wrapping_add(30)
        }
    }

    #[test]
    fn compressor_rotate_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xC07);
        let mut caught = 0;
        for i in 0..180u32 {
            let mut fx = Fixture::build(&mut rng);
            let index = i % 3;
            let attached = i % 2 == 0;
            let listener = if attached { fx.listener_obj.addr() } else { 0 };
            fx.obj.w32(INDEX, index);
            fx.obj.w32(LISTENER, listener);
            let notify_ans = U32_EDGE[(i as usize) % U32_EDGE.len()];
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[(1, StubKind::Thiscall1, vec![0x7777_1111])]);
            rt::set_virtual(&[("notify", vec![notify_ans])]);
            let mut fake = Fake::new();
            fake.answer("cx.notify", vec![notify_ans]);
            let got = unsafe { fn_008ac600::rw_008ac600(fx.obj.buf.as_mut_ptr()) };
            let want = v.rotate_params(&mut fake);
            assert_eq!(got, want, "index {index} attached {attached}");
            let slot = (index + 1) % 3;
            assert_eq!(fx.obj.r32(INDEX), slot);
            assert_eq!(v.index, slot);
            for r in 0..ROWS {
                for c in 0..WIDTH {
                    assert_eq!(
                        fx.obj.r32(PARAMS + r * WIDTH * 4 + c * 4),
                        v.params[r][c],
                        "row {r} col {c}"
                    );
                }
            }
            assert_only_changed(
                &before,
                &fx.obj.buf,
                &[(PARAMS + (slot as usize) * WIDTH * 4, WIDTH * 4), (INDEX, 4)],
            );
            assert_eq!(
                rt::take_numbered(),
                vec![(1, vec![fx.this()])],
                "rewrite base call"
            );
            let mut expect_lift = vec![("cx.base".to_string(), vec![])];
            if attached {
                check_virtual(rt::take_virtual(), vec![("notify", vec![listener])]);
                expect_lift.push(("cx.notify".to_string(), vec![listener]));
            } else {
                assert!(rt::take_virtual().is_empty());
            }
            assert_eq!(fake.log, expect_lift, "lift calls in order");
            let (wlast, wrows) = wrong::rotate_same(&lift_before(&before));
            if wlast != want || wrows != v.params {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong rotate never caught");
    }

    /// Rebuilds the lift from the pre-run image (the wrong run needs
    /// the entry state; `v` has already advanced).
    fn lift_before(before: &[u8]) -> CompressorEffect {
        let w = |off: usize| u32::from_le_bytes(before[off..off + 4].try_into().unwrap());
        let mut params = [[0u32; WIDTH]; ROWS];
        for (r, row) in params.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = w(PARAMS + r * WIDTH * 4 + c * 4);
            }
        }
        CompressorEffect {
            listener: cookie(w(LISTENER)),
            poll_index: w(POLL_INDEX),
            index: w(INDEX),
            sub: cookie(w(SUB)),
            params,
        }
    }

    #[test]
    fn compressor_slot_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x5107);
        let mut caught = 0;
        for i in 0..60u32 {
            let mut fx = Fixture::build(&mut rng);
            let index = i % 3;
            fx.obj.w32(INDEX, index);
            let before = fx.obj.buf.clone();
            let v = fx.lift();
            rt::set_script(&[]);
            let got = unsafe { fn_008ac680::rw_008ac680(fx.obj.buf.as_ptr()) };
            assert_eq!(got, fx.this().wrapping_add(index * 36).wrapping_add(0x78));
            assert_only_changed(&before, &fx.obj.buf, &[]);
            assert!(rt::take_numbered().is_empty());
            // The lift answers the row the address selects: same words.
            let row = v.slot();
            for c in 0..WIDTH {
                assert_eq!(
                    row[c],
                    fx.obj.r32(PARAMS + (index as usize) * WIDTH * 4 + c * 4),
                    "col {c}"
                );
            }
            if wrong::slot_next(&v) != *row {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong slot never caught");
    }

    #[test]
    fn compressor_set_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x5E7);
        let mut caught = 0;
        // Sweep the answer's low byte across the 0/1 boundary.
        let answers = [
            0u32, 1, 2, 0x7F, 0x80, 0xFF, 0x100, 0x101, 0x1FF, 0xFFFF_FF00, 0xFFFF_FFFF, 0x1234_5600,
            0xABCD_0001, 42,
        ];
        for (i, &ans) in answers.iter().cycle().take(140).enumerate() {
            let fx = Fixture::build(&mut rng);
            let a1 = U32_EDGE[i % U32_EDGE.len()];
            let a2 = rng.u32();
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[(1, StubKind::Thiscall3, vec![ans])]);
            let mut fake = Fake::new();
            fake.answer("cx.set", vec![ans]);
            let got = unsafe { fn_008ac690::rw_008ac690(fx.this(), a1, a2) };
            let want = v.set_param(&mut fake, a1, a2);
            assert_eq!(got, want, "ans {ans:#x}");
            assert_eq!(want, (ans & 0xFFFF_FF00) | u32::from((ans as u8) != 0));
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![(1, vec![fx.this(), a1, a2], "cx.set", vec![a1, a2])],
            );
            if wrong::set_raw(ans) != want {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong set never caught");
    }

    #[test]
    fn compressor_poll_matches() {
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
            // Keep the rotation index apart so the wrong slot differs.
            let index = poll_index.wrapping_add(1 + (i % 5));
            let sub = if i % 2 == 0 { rng.u32() & 0xFFFF_FFFC } else { 0 };
            fx.obj.w32(POLL_INDEX, poll_index);
            fx.obj.w32(INDEX, index);
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
            fake.answer("cx.post", vec![post]);
            let got = unsafe { fn_008ac6b0::rw_008ac6b0(fx.obj.buf.as_mut_ptr()) };
            let want = v.poll(&mut fake);
            assert_eq!(got, want);
            assert_eq!(want, post);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let slot = poll_index.wrapping_mul(9).wrapping_add(30);
            let slot_addr = fx.this().wrapping_add(slot.wrapping_mul(4));
            check_calls(
                rt::take_numbered(),
                std::mem::take(&mut fake.log),
                vec![
                    (1, vec![fx.this()], "cx.pre", vec![]),
                    (2, vec![sub, slot_addr], "cx.sub", vec![sub, slot]),
                    (3, vec![fx.this()], "cx.post", vec![]),
                ],
            );
            if wrong::poll_slot(&v) != slot {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong poll never caught");
    }
}
