//! Differential cases: the string's state methods.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! world call in order. Each case also runs a deliberately wrong lift,
//! which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_fontstr_diff::rewrites::*;
    use lf_fontstr_diff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        CORNER_A, CORNER_B, F32_EDGE, Fake, G_SLOT86, Image, OBJ_SIZE, POS, R_CA, R_CB, R_COL,
        R_POS, R_RDY, R_SCR, R_TAG, R_WA, R_WB, ROW_STRIDE, ROW_TEXT, Rng, STYLED, TAG, TEXT,
        U32_EDGE, WMODE, assert_only_changed, check_lift, check_numbered, check_virtual, lift_of,
    };

    /// One fixture: the 32-bit image with terminated live and row texts.
    struct Fixture {
        obj: Image,
    }

    impl Fixture {
        fn build(rng: &mut Rng, nrows: usize) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            obj.w32(0, 0x2222_2222);
            let live_len = 8 + (rng.u32() % 24) as usize;
            let live = rng.cstr(live_len);
            obj.wbytes(TEXT, &live);
            for s in 0..nrows {
                let len = (rng.u32() % 40) as usize;
                let t = rng.cstr(len);
                obj.wbytes(ROW_TEXT + s * 256, &t);
            }
            Self { obj }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_input_frontend::font_string::{FontString, FontWorld};

        /// Width base taken from the wrong corner: the second instead
        /// of the first.
        pub fn snapshot_width_base(corner_a: f32, corner_b: f32, width_mode: u8) -> f32 {
            let _ = corner_a;
            if width_mode != 0 { corner_b } else { 0.0 }
        }

        /// Notify polarity inverted.
        pub fn set_position<W: FontWorld>(
            s: &mut FontString,
            world: &mut W,
            x: f32,
            y: f32,
            hold: bool,
        ) {
            s.set_position(world, x, y, !hold);
        }

        /// Registry answer half-swapped.
        pub fn resolve_handle<W: FontWorld>(s: &mut FontString, world: &mut W, key: u32) {
            struct Swap<'a, W: ?Sized>(&'a mut W);
            impl<W: FontWorld + ?Sized> FontWorld for Swap<'_, W> {
                fn notify_position(&mut self, snap: [u32; 4]) {
                    self.0.notify_position(snap);
                }
                fn resolve_text(&mut self, key: u32) -> [u32; 2] {
                    let [a, b] = self.0.resolve_text(key);
                    [b, a]
                }
                fn resolve_sized(&mut self, size: u32) -> [u32; 2] {
                    self.0.resolve_sized(size)
                }
                fn refresh_parent(&mut self) {
                    self.0.refresh_parent();
                }
                fn copy_text(&mut self, text: &[u8; 256]) {
                    self.0.copy_text(text);
                }
                fn guard_veto(&mut self) -> bool {
                    self.0.guard_veto()
                }
                fn sink_primary(&mut self, bits: u32) -> u32 {
                    self.0.sink_primary(bits)
                }
                fn notify_a(&mut self) {
                    self.0.notify_a();
                }
                fn notify_b(&mut self) {
                    self.0.notify_b();
                }
                fn metric_a(&mut self) -> f32 {
                    self.0.metric_a()
                }
                fn metric_b(&mut self) -> f32 {
                    self.0.metric_b()
                }
                fn metric_c(&mut self) -> f32 {
                    self.0.metric_c()
                }
                fn metric_d(&mut self) -> f32 {
                    self.0.metric_d()
                }
                fn render(&mut self) -> u32 {
                    self.0.render()
                }
                fn refresh_base(&mut self) {
                    self.0.refresh_base();
                }
                fn push_scale(&mut self, bits: u32) {
                    self.0.push_scale(bits);
                }
                fn visible(&mut self) -> bool {
                    self.0.visible()
                }
                fn advance(&mut self) -> f32 {
                    self.0.advance()
                }
                fn push_style(&mut self, style: u32) {
                    self.0.push_style(style);
                }
                fn push_position(&mut self, lo: u32, hi: u32) {
                    self.0.push_position(lo, hi);
                }
                fn push_a(&mut self, bits: u32) {
                    self.0.push_a(bits);
                }
                fn push_colour(&mut self, colour: u32) {
                    self.0.push_colour(colour);
                }
                fn push_row_word(&mut self, bits: u32) {
                    self.0.push_row_word(bits);
                }
                fn push_one(&mut self) {
                    self.0.push_one();
                }
                fn push_width(&mut self, base: u32, add: u32) {
                    self.0.push_width(base, add);
                }
                fn push_opacity(&mut self, bits: u32) {
                    self.0.push_opacity(bits);
                }
                fn convert_text(&mut self, text: &[u8]) -> [u32; 8] {
                    self.0.convert_text(text)
                }
                fn resolve_cached(&mut self, text: &[u8]) -> [u32; 8] {
                    self.0.resolve_cached(text)
                }
                fn submit(&mut self, a: u32, b: u32, w: &[u32; 8]) -> u32 {
                    self.0.submit(a, b, w)
                }
                fn query_height(&mut self, w: &[u32; 8]) -> f32 {
                    self.0.query_height(w)
                }
                fn line_height(&mut self) -> f32 {
                    self.0.line_height()
                }
                fn sink_height(&mut self, bits: u32) {
                    self.0.sink_height(bits);
                }
                fn sink_line(&mut self, bits: u32) {
                    self.0.sink_line(bits);
                }
                fn sink_scaled(&mut self, bits: u32) {
                    self.0.sink_scaled(bits);
                }
                fn pick_ext_a(&mut self) -> bool {
                    self.0.pick_ext_a()
                }
                fn pick_ext_b(&mut self) -> bool {
                    self.0.pick_ext_b()
                }
                fn end_frame(&mut self) {
                    self.0.end_frame();
                }
            }
            let mut swap = Swap(world);
            s.resolve_handle(&mut swap, key);
        }

        /// Kind 0 resolved with the wrong size.
        pub fn select_handle<W: FontWorld>(s: &mut FontString, world: &mut W, kind: u32) {
            if kind == 0 {
                let pair = world.resolve_sized(8);
                // Store through a second call to keep the shape.
                let _ = pair;
                world.notify_position([0, 0, 0, 0]);
            } else {
                s.select_handle(world, kind);
            }
        }
    }

    #[test]
    fn diff_snapshot_row() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x5A86);
        let mut caught = 0;
        for slot in 0..3u32 {
            for trial in 0..30u32 {
                let mut fx = Fixture::build(&mut rng, 3);
                // Vary the width mode and the live words under test.
                fx.obj.w8(WMODE, [0, 1, 0xFF][(trial % 3) as usize]);
                if trial % 4 == 0 {
                    fx.obj.w32(CORNER_A, F32_EDGE[(trial % 14) as usize]);
                    fx.obj
                        .w32(support::SCALE, F32_EDGE[((trial + 5) % 14) as usize]);
                }
                // Text length shapes: empty, short, long, full.
                let live: Vec<u8> = match trial % 4 {
                    0 => vec![0],
                    1 => rng.cstr(3),
                    2 => rng.cstr(200),
                    _ => {
                        let mut v = vec![0x41; 255];
                        v.push(0);
                        v
                    }
                };
                fx.obj.wbytes(TEXT, &live);
                let mut s = lift_of(&fx.obj, 3);
                // Zero the destination row text so the copy length shows.
                let dst0 = ROW_TEXT + slot as usize * 256;
                for b in &mut fx.obj.buf[dst0..dst0 + 256] {
                    *b = 0xCC;
                }
                let before = fx.obj.buf.to_vec();
                rt::set_script(&[]);
                rt::set_global(G_SLOT86, slot);
                let r = unsafe { fn_00db7080::rw_00db7080(fx.this()) };
                assert_eq!(r, 0);
                s.snapshot_row(slot);
                // Row fields match the lift's row.
                let base = slot as usize * ROW_STRIDE;
                let row = &s.rows()[slot as usize];
                assert_eq!(fx.obj.r32(base + R_CA), row.corner_a.to_bits());
                assert_eq!(fx.obj.r32(base + R_CB), row.corner_b.to_bits());
                assert_eq!(fx.obj.r32(base + R_POS), row.pos[0]);
                assert_eq!(fx.obj.r32(base + R_POS + 4), row.pos[1]);
                assert_eq!(fx.obj.r32(base + R_COL), row.colour);
                assert_eq!(fx.obj.r32(base + R_TAG), row.tag);
                assert_eq!(fx.obj.r8(base + R_SCR), row.use_scratch);
                assert_eq!(fx.obj.r32(base + R_WB), row.width_base.to_bits());
                assert_eq!(fx.obj.r32(base + R_WA), row.width_add.to_bits());
                assert_eq!(fx.obj.r8(base + R_RDY), 1);
                assert!(row.ready);
                // Row text matches byte for byte.
                let dst = ROW_TEXT + slot as usize * 256;
                assert_eq!(&fx.obj.buf[dst..dst + row.text.len()], row.text.as_slice());
                assert_eq!(&fx.obj.buf[dst..dst + live.len()], live.as_slice());
                // Nothing else moved except the row fields and the copy.
                assert_only_changed(
                    &before,
                    &fx.obj.buf,
                    &[
                        (base + R_CA, 4),
                        (base + R_CB, 4),
                        (base + R_POS, 8),
                        (base + R_WB, 8),
                        (base + R_COL, 4),
                        (base + R_TAG, 4),
                        (base + R_SCR, 1),
                        (base + R_RDY, 1),
                        (dst, live.len()),
                    ],
                );
                check_numbered(rt::take_numbered(), &[]);
                check_virtual(rt::take_virtual(), &[]);
                // Wrong lift: width base from the second corner.
                let wmode = before[WMODE];
                let ca = f32::from_bits(u32::from_le_bytes(
                    before[CORNER_A..CORNER_A + 4].try_into().unwrap(),
                ));
                let cb = f32::from_bits(u32::from_le_bytes(
                    before[CORNER_B..CORNER_B + 4].try_into().unwrap(),
                ));
                if wrong::snapshot_width_base(ca, cb, wmode).to_bits() != fx.obj.r32(base + R_WB) {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong snapshot never caught");
    }

    #[test]
    fn diff_set_position() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9119);
        let mut caught = 0;
        for i in 0..48u32 {
            let xb = if i < 14 {
                F32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            let yb = rng.u32();
            for hold in [false, true] {
                let mut fx = Fixture::build(&mut rng, 1);
                let mut s = lift_of(&fx.obj, 1);
                let mut fake = Fake::new();
                let before = fx.obj.buf.to_vec();
                rt::set_script(&[(1, StubKind::Cdecl4Snap, vec![0xA11CE])]);
                let flag = if hold { 1u32 } else { 0 };
                let r = unsafe { fn_00db73d0::rw_00db73d0(fx.this(), xb, yb, flag) };
                assert_eq!(r, 0);
                assert_eq!(fx.obj.r32(POS), xb);
                assert_eq!(fx.obj.r32(POS + 4), yb);
                s.set_position(&mut fake, f32::from_bits(xb), f32::from_bits(yb), hold);
                assert_eq!(s.pos(), [xb, yb]);
                assert_only_changed(&before, &fx.obj.buf, &[(POS, 8)]);
                let snap = [
                    fx.obj.r32(POS),
                    fx.obj.r32(POS + 4),
                    fx.obj.r32(POS + 8),
                    fx.obj.r32(POS + 12),
                ];
                if hold {
                    check_numbered(rt::take_numbered(), &[]);
                    assert!(rt::take_snaps().is_empty());
                    check_lift(&fake.log, &[]);
                } else {
                    check_numbered(
                        rt::take_numbered(),
                        &[(1, vec![2, 0, fx.this().wrapping_add(POS as u32), 0])],
                    );
                    assert_eq!(rt::take_snaps(), vec![snap.to_vec()]);
                    check_lift(&fake.log, &[("notify_position", snap.to_vec(), vec![])]);
                }
                check_virtual(rt::take_virtual(), &[]);
                // Wrong lift: inverted polarity, caught on the call log.
                let mut w = lift_of(&Image::from_vec(before.clone()), 1);
                let mut wfake = Fake::new();
                wrong::set_position(
                    &mut w,
                    &mut wfake,
                    f32::from_bits(xb),
                    f32::from_bits(yb),
                    hold,
                );
                if wfake.log.len() != fake.log.len() {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong position polarity never caught");
    }

    #[test]
    fn diff_resolve_handle() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9117);
        let mut caught = 0;
        for i in 0..40u32 {
            let key = if i < 12 {
                U32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            let found = [rng.u32(), rng.u32()];
            let mut fx = Fixture::build(&mut rng, 1);
            let found_img = Image::from_vec({
                let mut v = vec![0u8; 8];
                v[..4].copy_from_slice(&found[0].to_le_bytes());
                v[4..].copy_from_slice(&found[1].to_le_bytes());
                v
            });
            let mut s = lift_of(&fx.obj, 1);
            let mut fake = Fake::new();
            fake.answer_pairs(vec![found]);
            let before = fx.obj.buf.to_vec();
            rt::set_script(&[
                (1, StubKind::Cdecl2Snap2, vec![found_img.addr()]),
                (2, StubKind::Cdecl4Snap, vec![0]),
            ]);
            let r = unsafe { fn_00db7470::rw_00db7470(fx.this(), key) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r32(POS), found[0]);
            assert_eq!(fx.obj.r32(POS + 4), found[1]);
            s.resolve_handle(&mut fake, key);
            assert_eq!(s.pos(), found);
            assert_only_changed(&before, &fx.obj.buf, &[(POS, 8)]);
            let numbered = rt::take_numbered();
            assert_eq!(numbered.len(), 2, "two backend calls");
            assert_eq!(numbered[0].0, 1);
            assert_eq!(numbered[0].1[1], key, "registry key");
            assert_eq!(
                numbered[1],
                (2, vec![2, 0, fx.this().wrapping_add(POS as u32), 0])
            );
            let snaps = rt::take_snaps();
            assert_eq!(snaps[0], vec![0, 0], "zeroed registry scratch");
            let snap = [
                fx.obj.r32(POS),
                fx.obj.r32(POS + 4),
                fx.obj.r32(POS + 8),
                fx.obj.r32(POS + 12),
            ];
            assert_eq!(snaps[1], snap.to_vec());
            check_lift(
                &fake.log,
                &[
                    ("resolve_text", vec![key], vec![]),
                    ("notify_position", snap.to_vec(), vec![]),
                ],
            );
            check_virtual(rt::take_virtual(), &[]);
            // Wrong lift: swapped pair, caught on the stored words.
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            let mut wfake = Fake::new();
            wfake.answer_pairs(vec![found]);
            wrong::resolve_handle(&mut w, &mut wfake, key);
            if w.pos() != found {
                caught += 1;
            }
            let _ = found_img;
        }
        assert!(caught > 0, "wrong resolve never caught");
    }

    #[test]
    fn diff_select_handle() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9118);
        let mut caught = 0;
        for kind in [0u32, 2, 1, 5, u32::MAX] {
            for _ in 0..12 {
                let found = [rng.u32(), rng.u32()];
                let mut fx = Fixture::build(&mut rng, 1);
                let found_img = Image::from_vec({
                    let mut v = vec![0u8; 8];
                    v[..4].copy_from_slice(&found[0].to_le_bytes());
                    v[4..].copy_from_slice(&found[1].to_le_bytes());
                    v
                });
                let mut s = lift_of(&fx.obj, 1);
                let mut fake = Fake::new();
                fake.answer_pairs(vec![found]);
                let before = fx.obj.buf.to_vec();
                rt::set_script(&[
                    (1, StubKind::Cdecl2Snap2, vec![found_img.addr()]),
                    (2, StubKind::Cdecl4Snap, vec![0]),
                ]);
                let r = unsafe { fn_00db7410::rw_00db7410(fx.this(), kind) };
                assert_eq!(r, 0);
                s.select_handle(&mut fake, kind);
                assert_eq!(s.pos(), found);
                assert_eq!(fx.obj.r32(POS), found[0]);
                assert_eq!(fx.obj.r32(POS + 4), found[1]);
                assert_only_changed(&before, &fx.obj.buf, &[(POS, 8)]);
                let size = if kind == 0 { 2 } else { 8 };
                let numbered = rt::take_numbered();
                assert_eq!(numbered.len(), 2);
                assert_eq!(numbered[0].0, 1);
                assert_eq!(numbered[0].1[1], size, "kind {kind} size");
                let snaps = rt::take_snaps();
                assert_eq!(snaps[0], vec![0, 0], "zeroed scratch");
                let snap = [
                    fx.obj.r32(POS),
                    fx.obj.r32(POS + 4),
                    fx.obj.r32(POS + 8),
                    fx.obj.r32(POS + 12),
                ];
                assert_eq!(snaps[1], snap.to_vec());
                check_lift(
                    &fake.log,
                    &[
                        ("resolve_sized", vec![size], vec![]),
                        ("notify_position", snap.to_vec(), vec![]),
                    ],
                );
                check_virtual(rt::take_virtual(), &[]);
                // Wrong lift: kind 0 resolved with size 8.
                let mut w = lift_of(&Image::from_vec(before.clone()), 1);
                let mut wfake = Fake::new();
                wfake.answer_pairs(vec![found]);
                wrong::select_handle(&mut w, &mut wfake, kind);
                let wsize = wfake
                    .log
                    .first()
                    .map(|c| c.words.clone())
                    .unwrap_or_default();
                if kind == 0 && wsize != vec![2] {
                    caught += 1;
                }
                let _ = found_img;
            }
        }
        assert!(caught > 0, "wrong kind size never caught");
    }

    #[test]
    fn diff_set_text() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x9120);
        let mut caught = 0;
        for i in 0..32u32 {
            let with_text = i % 2 == 0;
            let refresh = i % 4 < 2;
            let mut fx = Fixture::build(&mut rng, 1);
            let src_bytes: Vec<u8> = (0..256).map(|_| rng.u8()).collect();
            let src_img = Image::from_vec(src_bytes.clone());
            let text_arg = if with_text { src_img.addr() } else { 0 };
            let flag = if refresh { 1u32 } else { 0 };
            let mut text256 = [0u8; 256];
            text256.copy_from_slice(&src_bytes);
            let arg: Option<&[u8; 256]> = if with_text { Some(&text256) } else { None };
            let mut s = lift_of(&fx.obj, 1);
            let mut fake = Fake::new();
            let before = fx.obj.buf.to_vec();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![0xBEEF]),
                (2, StubKind::Cdecl3, vec![0xCAFE]),
            ]);
            let r = unsafe { fn_00db74c0::rw_00db74c0(fx.this(), text_arg, flag) };
            assert_eq!(r, 0);
            s.set_text(&mut fake, arg, refresh);
            // Only the forced terminator moves, and only with text.
            if with_text {
                assert_eq!(fx.obj.r8(support::TEXT_NUL), 0);
                assert_eq!(s.text()[255], 0);
                assert_only_changed(&before, &fx.obj.buf, &[(support::TEXT_NUL, 1)]);
            } else {
                assert_only_changed(&before, &fx.obj.buf, &[]);
            }
            let mut expect_numbered = Vec::new();
            if refresh {
                expect_numbered.push((1, vec![fx.this()]));
            }
            if with_text {
                expect_numbered.push((
                    2,
                    vec![fx.this().wrapping_add(TEXT as u32), text_arg, 0x100],
                ));
            }
            check_numbered(rt::take_numbered(), &expect_numbered);
            assert!(rt::take_snaps().is_empty());
            let mut expect_lift = Vec::new();
            if refresh {
                expect_lift.push(("refresh_parent", vec![], vec![]));
            }
            if with_text {
                expect_lift.push(("copy_text", vec![], src_bytes.clone()));
            }
            check_lift(&fake.log, &expect_lift);
            check_virtual(rt::take_virtual(), &[]);
            // Wrong lift: flipped refresh polarity, caught on the log.
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            let mut wfake = Fake::new();
            w.set_text(&mut wfake, arg, !refresh);
            let wrefreshed = wfake.log.iter().any(|c| c.name == "refresh_parent");
            if wrefreshed != refresh {
                caught += 1;
            }
            let _ = src_img;
        }
        assert!(caught > 0, "wrong refresh polarity never caught");
    }
}
