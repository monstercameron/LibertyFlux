//! Differential cases: the string's render passes.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! world call in order, floats bit for bit. Each case also runs a
//! deliberately wrong lift, which must be caught at least once. 32-bit
//! target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_fontstr_diff::rewrites::*;
    use lf_fontstr_diff::rt::{self, StubKind};
    use lf_input_frontend::font_string::MeasureInputs;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        CORNER_A, CORNER_B, F32_EDGE, Fake, G_DEFAULT, G_DIV, G_EXT_A, G_EXT_B, G_EXT_C, G_EXT_D,
        G_MUL, G_SCALE85, G_SLOT87, G_UIALT, G_UIMODE, Image, MODE, OBJ_SIZE, POS, PUSH_A, PUSH_B,
        R_CA, R_CB, R_COL, R_POS, R_RDY, R_SCR, R_TAG, R_WA, R_WB, ROW_STRIDE, ROW_TEXT, Rng,
        SCALE, SIZE, SLOT_ADV, SLOT_GUARD, SLOT_MET_A, SLOT_MET_B, SLOT_MET_C, SLOT_MET_D,
        SLOT_NOTIFY_A, SLOT_NOTIFY_B, SLOT_RENDER, SLOT_SINK_H, SLOT_SINK_L, SLOT_SINK_PRI,
        SLOT_SINK_SCL, SLOT_VIS, STYLE, STYLED, Stubs, TAG, TAG_B, TEXT, TEXT_NUL, U32_EDGE,
        VTable, WMODE, assert_only_changed, check_lift, check_numbered, check_virtual, lift_of,
        quiet_snan, row_of,
    };

    /// One fixture: the 32-bit image, its vtable, and the cache block.
    struct Fixture {
        obj: Image,
        own_vt: VTable,
        stubs: Stubs,
        cache: Image,
    }

    impl Fixture {
        fn build(rng: &mut Rng, nrows: usize) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let own_vt = VTable::random(0x150 / 4, rng);
            let stubs = Stubs::new();
            let cache = Image::random(64, rng);
            obj.w32(0, own_vt.addr());
            let live_len = 8 + (rng.u32() % 24) as usize;
            let live = rng.cstr(live_len);
            obj.wbytes(TEXT, &live);
            for s in 0..nrows {
                let len = (rng.u32() % 40) as usize;
                let t = rng.cstr(len);
                obj.wbytes(ROW_TEXT + s * 256, &t);
            }
            Self {
                obj,
                own_vt,
                stubs,
                cache,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        fn plant(&mut self, slot: usize, target: u32) {
            self.own_vt.set(slot, target);
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        /// Refresh first corner with the mode-0 arm adding instead of
        /// subtracting.
        pub fn refresh_first(mode: u32, m1: f32, m3: f32, scale: f32) -> f32 {
            if mode == 0 {
                m1 + m3 * scale
            } else if mode == 1 {
                m3 * scale + m1
            } else {
                m1
            }
        }

        /// Reset tag one too high.
        pub fn reset_tag() -> u32 {
            8
        }

        /// Emit style without the draw-mode override.
        pub fn emit_style(tag: u32) -> u32 {
            tag
        }

        /// Measure's scaled height with the extent pair swapped.
        pub fn scaled_swapped(base: f32, height: f32, div: f32, ext_a: u32, ext_b: u32) -> f32 {
            let aspect = (ext_b as i32 as f32) / (ext_a as i32 as f32);
            (base * height) / (div / aspect)
        }
    }

    #[test]
    fn diff_refresh() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x8E85);
        let mut caught = 0;
        // Scales from the checker's cycle plus edges and randoms.
        // (One value per line, as words with their meanings.)
        let scales: [u32; 10] = [
            0x3F80_0000, // 1
            0x4000_0000, // 2
            0xC000_0000, // -2
            0x4040_0000, // 3
            0x3F00_0000, // 0.5
            0x8000_0000, // -0
            0x7FC0_0001, // quiet NaN, payload 1
            0xFF80_0000, // -inf
            0x0000_0001, // smallest subnormal
            0x7F7F_FFFF, // largest finite
        ];
        for mode in [0u32, 1, 2, 7, 0xFFFF_FFFF] {
            for trial in 0..24u32 {
                let metrics: [u32; 4] = if trial < 14 {
                    [
                        F32_EDGE[trial as usize],
                        F32_EDGE[(trial as usize + 3) % 14],
                        F32_EDGE[(trial as usize + 7) % 14],
                        F32_EDGE[(trial as usize + 11) % 14],
                    ]
                } else {
                    [rng.u32(), rng.u32(), rng.u32(), rng.u32()]
                };
                let scale = if trial < 10 {
                    scales[trial as usize]
                } else {
                    rng.u32()
                };
                let render_ans = if trial % 3 == 0 {
                    U32_EDGE[(trial % 12) as usize]
                } else {
                    rng.u32()
                };
                let mut fx = Fixture::build(&mut rng, 1);
                fx.obj.w32(MODE, mode);
                fx.plant(SLOT_MET_A, fx.stubs.metric_a);
                fx.plant(SLOT_MET_B, fx.stubs.metric_b);
                fx.plant(SLOT_MET_C, fx.stubs.metric_c);
                fx.plant(SLOT_MET_D, fx.stubs.metric_d);
                fx.plant(SLOT_RENDER, fx.stubs.render);
                let mut s = lift_of(&fx.obj, 1);
                let mut fake = Fake::new();
                fake.answer("metric_a", vec![metrics[0]]);
                fake.answer("metric_b", vec![metrics[1]]);
                fake.answer("metric_c", vec![metrics[2]]);
                fake.answer("metric_d", vec![metrics[3]]);
                fake.answer("render", vec![render_ans]);
                let before = fx.obj.buf.to_vec();
                rt::set_script(&[]);
                rt::set_virtual(&[
                    ("metric_a", vec![metrics[0]]),
                    ("metric_b", vec![metrics[1]]),
                    ("metric_c", vec![metrics[2]]),
                    ("metric_d", vec![metrics[3]]),
                    ("render", vec![render_ans]),
                ]);
                rt::set_global(G_SCALE85, scale);
                let r = unsafe { fn_00db6eb0::rw_00db6eb0(fx.this()) };
                let got = s.refresh(&mut fake, f32::from_bits(scale));
                assert_eq!(r, render_ans);
                assert_eq!(got, r);
                assert_eq!(fx.obj.r32(CORNER_A), s.corners().0.to_bits());
                assert_eq!(fx.obj.r32(CORNER_B), s.corners().1.to_bits());
                assert_only_changed(&before, &fx.obj.buf, &[(CORNER_A, 8)]);
                let this = fx.this();
                check_virtual(
                    rt::take_virtual(),
                    &[
                        ("metric_a", vec![this]),
                        ("metric_b", vec![this]),
                        ("metric_c", vec![this]),
                        ("metric_d", vec![this]),
                        ("render", vec![this]),
                    ],
                );
                check_numbered(rt::take_numbered(), &[]);
                check_lift(
                    &fake.log,
                    &[
                        ("metric_a", vec![], vec![]),
                        ("metric_b", vec![], vec![]),
                        ("metric_c", vec![], vec![]),
                        ("metric_d", vec![], vec![]),
                        ("render", vec![], vec![]),
                    ],
                );
                // Wrong lift: mode-0 arm adds.
                let w = wrong::refresh_first(
                    mode,
                    f32::from_bits(metrics[0]),
                    f32::from_bits(metrics[2]),
                    f32::from_bits(scale),
                );
                if w.to_bits() != fx.obj.r32(CORNER_A) {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong refresh never caught");
    }

    #[test]
    fn diff_reset() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x7115);
        let mut caught = 0;
        for i in 0..48u32 {
            let veto = i % 2 == 1;
            // Guard answers pin the low-byte test: 0x100 vetoes nothing.
            let guard_ans = match i % 6 {
                0 => 0,
                1 => 1,
                2 => 0x100,
                3 => 0x1FF,
                4 => 0xFFFF_FF00,
                _ => 0xFFFF_FFFF,
            };
            let vetoed = guard_ans & 0xFF != 0;
            let size = if i < 14 {
                F32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            let default = if i % 3 == 0 {
                [0x3F80_0000, 0x4000_0000, 0, 0xC000_0000][(i / 3 % 4) as usize]
            } else {
                rng.u32()
            };
            let src_word = rng.u32();
            let tag_a = if i % 5 == 0 { i % 3 } else { rng.u32() };
            let tag_b = rng.u32();
            let mut fx = Fixture::build(&mut rng, 1);
            fx.plant(SLOT_GUARD, fx.stubs.guard);
            fx.plant(SLOT_SINK_PRI, fx.stubs.sink_pri);
            fx.plant(SLOT_NOTIFY_A, fx.stubs.notify_a);
            fx.plant(SLOT_NOTIFY_B, fx.stubs.notify_b);
            let src_img = Image::from_vec({
                let mut v = vec![0u8; 16];
                v[..4].copy_from_slice(&src_word.to_le_bytes());
                for (k, b) in v[4..].iter_mut().enumerate() {
                    *b = (k as u8).wrapping_mul(7);
                }
                v
            });
            let mut s = lift_of(&fx.obj, 1);
            let mut fake = Fake::new();
            fake.answer("guard_veto", vec![guard_ans]);
            fake.answer("sink_primary", vec![0x5EED]);
            let before = fx.obj.buf.to_vec();
            rt::set_script(&[(5, StubKind::Cdecl4Snap, vec![0xA11CE])]);
            rt::set_virtual(&[
                ("guard", vec![guard_ans]),
                ("sink_primary", vec![0x5EED]),
                ("notify_a", vec![0]),
                ("notify_b", vec![0]),
            ]);
            rt::set_global(G_DEFAULT, default);
            let r =
                unsafe { fn_00db6f70::rw_00db6f70(fx.this(), size, src_img.addr(), tag_a, tag_b) };
            s.reset(
                &mut fake,
                f32::from_bits(size),
                src_word,
                tag_a,
                tag_b,
                f32::from_bits(default),
            );
            assert_eq!(r, 0);
            let this = fx.this();
            if vetoed {
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_virtual(rt::take_virtual(), &[("guard", vec![this])]);
                check_numbered(rt::take_numbered(), &[]);
                assert!(rt::take_snaps().is_empty());
                check_lift(&fake.log, &[("guard_veto", vec![], vec![])]);
            } else {
                assert_eq!(fx.obj.r32(SIZE), size);
                assert_eq!(fx.obj.r32(MODE), tag_a);
                assert_eq!(fx.obj.r32(TAG_B), tag_b);
                assert_eq!(fx.obj.r32(TAG), 7);
                assert_eq!(fx.obj.r32(STYLE), src_word);
                assert_eq!(fx.obj.r32(SCALE), 0x3F80_0000);
                assert_eq!(fx.obj.r32(PUSH_A), 0);
                assert_eq!(fx.obj.r32(PUSH_B), 0x3F80_0000);
                for off in [
                    STYLED,
                    support::F209,
                    support::F20A,
                    WMODE,
                    support::F20C,
                    support::F20D,
                ] {
                    assert_eq!(fx.obj.r8(off), 0);
                }
                assert_eq!(s.size().to_bits(), size);
                assert_eq!(s.mode(), tag_a);
                assert_eq!(s.tags(), (tag_b, 7));
                assert_eq!(s.style(), src_word);
                assert_eq!(s.scale().to_bits(), 0x3F80_0000);
                assert_eq!(s.pushed(), (f32::from_bits(0), 1.0));
                assert_eq!(s.flags(), [0, 0, 0, 0, 0, 0]);
                assert_only_changed(
                    &before,
                    &fx.obj.buf,
                    &[
                        (SIZE, 4),
                        (MODE, 4),
                        (TAG_B, 4),
                        (TAG, 4),
                        (STYLE, 4),
                        (SCALE, 4),
                        (STYLED, 6),
                        (PUSH_A, 4),
                        (PUSH_B, 4),
                    ],
                );
                check_virtual(
                    rt::take_virtual(),
                    &[
                        ("guard", vec![this]),
                        ("sink_primary", vec![this, default]),
                        ("notify_a", vec![this, 1]),
                        ("notify_b", vec![this, 1]),
                    ],
                );
                check_numbered(
                    rt::take_numbered(),
                    &[(5, vec![2, 0, this.wrapping_add(POS as u32), 0])],
                );
                let snap = [
                    fx.obj.r32(POS),
                    fx.obj.r32(POS + 4),
                    fx.obj.r32(POS + 8),
                    fx.obj.r32(POS + 12),
                ];
                assert_eq!(rt::take_snaps(), vec![snap.to_vec()]);
                check_lift(
                    &fake.log,
                    &[
                        ("guard_veto", vec![], vec![]),
                        ("sink_primary", vec![default], vec![]),
                        ("notify_a", vec![], vec![]),
                        ("notify_position", snap.to_vec(), vec![]),
                        ("notify_b", vec![], vec![]),
                    ],
                );
                // Wrong lift: tag 8.
                if wrong::reset_tag() != fx.obj.r32(TAG) {
                    caught += 1;
                }
            }
            let _ = (src_img, veto);
        }
        assert!(caught > 0, "wrong reset never caught");
    }

    #[test]
    fn diff_emit_row() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xE817);
        let mut caught = 0;
        let mut cases = 0;
        for slot in 0..3u32 {
            for trial in 0..36u32 {
                // Branch matrix inputs.
                let tag_zero = trial % 3 == 0;
                let ready = match trial % 4 {
                    0 => 0u8,
                    1 => 1,
                    2 => 0x42,
                    _ => 1,
                };
                let use_scratch = if trial % 2 == 0 { 1u8 } else { 0 };
                let styled = if trial % 5 == 0 { 1u8 } else { 0 };
                let (ui_mode, ui_alt) = match trial % 5 {
                    0 => (0u8, 0u8),
                    1 => (0x6A, 0),
                    2 => (0, 1),
                    3 => (0x6B, 0),
                    _ => (0x6A, 0xFF),
                };
                let mut fx = Fixture::build(&mut rng, 3);
                let base = slot as usize * ROW_STRIDE;
                fx.obj.w8(base + R_RDY, ready);
                fx.obj.w8(base + R_SCR, use_scratch);
                fx.obj.w8(STYLED, styled);
                let text_addr = ROW_TEXT + slot as usize * 256;
                if tag_zero {
                    fx.obj.w8(text_addr, 0);
                } else if fx.obj.r8(text_addr) == 0 {
                    fx.obj.w8(text_addr, b'Q');
                }
                // Row tag: usually not 2, so the override shows.
                if trial % 7 != 0 && fx.obj.r32(base + R_TAG) == 2 {
                    fx.obj.w32(base + R_TAG, 3);
                }
                let group: [u32; 8] = [
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                    rng.u32(),
                ];
                let cached_img = Image::from_vec({
                    let mut v = vec![0u8; 32];
                    for (k, w) in group.iter().enumerate() {
                        v[k * 4..k * 4 + 4].copy_from_slice(&w.to_le_bytes());
                    }
                    v
                });
                let submit_ans = if trial % 3 == 0 {
                    U32_EDGE[(trial % 12) as usize]
                } else {
                    rng.u32()
                };
                let mut s = lift_of(&fx.obj, 3);
                let mut fake = Fake::new();
                fake.answer_groups(vec![group]);
                fake.answer("submit", vec![submit_ans]);
                let before = fx.obj.buf.to_vec();
                rt::set_script(&[
                    (1, StubKind::Cdecl1, vec![0]),
                    (2, StubKind::Cdecl2, vec![0]),
                    (3, StubKind::Cdecl1, vec![0]),
                    (4, StubKind::Cdecl1, vec![0]),
                    (5, StubKind::Cdecl1, vec![0]),
                    (6, StubKind::Cdecl1, vec![0]),
                    (7, StubKind::Cdecl2, vec![0]),
                    (8, StubKind::Cdecl1, vec![0]),
                    (9, StubKind::Cdecl2Write8, vec![0]),
                    (10, StubKind::Thiscall2, vec![cached_img.addr()]),
                    (11, StubKind::Cdecl5Snap8, vec![submit_ans]),
                    (12, StubKind::Cdecl0, vec![0]),
                ]);
                rt::set_out_words(&[(9, vec![group])]);
                rt::set_global(G_SLOT87, slot);
                rt::set_global(G_UIMODE, ui_mode as u32);
                rt::set_global(G_UIALT, ui_alt as u32);
                rt::set_cache_addr(fx.cache.addr());
                let r = unsafe { fn_00db7170::rw_db7170(fx.this()) };
                let got = s.emit_row(&mut fake, slot, ui_mode, ui_alt);
                let this = fx.this();
                let row = row_of(&Image::from_vec(before.clone()), slot as usize);
                let draws = row.text[0] != 0 && ready != 0;
                if !draws {
                    assert_eq!(r, slot << 8);
                    assert_eq!(got, slot << 8);
                    assert_only_changed(&before, &fx.obj.buf, &[]);
                    check_numbered(rt::take_numbered(), &[(12, vec![])]);
                    assert!(rt::take_snaps().is_empty());
                    check_virtual(rt::take_virtual(), &[]);
                    check_lift(&fake.log, &[("end_frame", vec![], vec![])]);
                } else {
                    let style = if (ui_mode == 0x6A || ui_alt != 0) && styled == 0 {
                        2
                    } else {
                        row.tag
                    };
                    assert_eq!(r, submit_ans);
                    assert_eq!(got, submit_ans);
                    assert_eq!(fx.obj.r8(base + R_RDY), 0);
                    assert!(!s.rows()[slot as usize].ready);
                    assert_only_changed(&before, &fx.obj.buf, &[(base + R_RDY, 1)]);
                    // Numbered calls: pointer arguments behind the text
                    // and scratch are asserted structurally below.
                    let numbered = rt::take_numbered();
                    let row_text_addr = this.wrapping_add(text_addr as u32);
                    assert_eq!(numbered.len(), 11, "emit backend calls");
                    assert_eq!(numbered[0], (1, vec![style]));
                    assert_eq!(numbered[1], (2, vec![row.pos[0], row.pos[1]]));
                    assert_eq!(
                        numbered[2],
                        (
                            3,
                            vec![u32::from_le_bytes(
                                before[PUSH_A..PUSH_A + 4].try_into().unwrap()
                            )]
                        )
                    );
                    assert_eq!(numbered[3], (4, vec![0xFF00_0000]));
                    assert_eq!(numbered[4], (5, vec![row.colour]));
                    assert_eq!(numbered[5], (6, vec![1]));
                    assert_eq!(
                        numbered[6],
                        (7, vec![row.width_base.to_bits(), row.width_add.to_bits()])
                    );
                    assert_eq!(
                        numbered[7],
                        (
                            8,
                            vec![u32::from_le_bytes(
                                before[PUSH_B..PUSH_B + 4].try_into().unwrap()
                            )]
                        )
                    );
                    if use_scratch != 0 {
                        assert_eq!(numbered[8].0, 9);
                        assert_eq!(numbered[8].1[0], row_text_addr);
                        assert_ne!(numbered[8].1[1], 0, "scratch address");
                        assert_eq!(numbered[9].0, 11);
                        assert_ne!(numbered[9].1[2], 0, "converted words");
                        assert_eq!(rt::take_snaps(), vec![group.to_vec(), group.to_vec()]);
                    } else {
                        assert_eq!(numbered[8], (10, vec![fx.cache.addr(), row_text_addr]));
                        assert_eq!(numbered[9].0, 11);
                        assert_eq!(numbered[9].1[2], cached_img.addr(), "cached words");
                        assert_eq!(rt::take_snaps(), vec![group.to_vec()]);
                    }
                    assert_eq!(
                        &numbered[9].1[0..2],
                        &[row.corner_a.to_bits(), row.corner_b.to_bits()]
                    );
                    assert_eq!(&numbered[9].1[3..5], &[0xFFFF_FFFF, 0xFFFF_FFFF]);
                    assert_eq!(numbered[10], (12, vec![]), "closing frame call");
                    check_virtual(rt::take_virtual(), &[]);
                    let text = row.text.clone();
                    let mut expect_lift: Vec<(&str, Vec<u32>, Vec<u8>)> = vec![
                        ("push_style", vec![style], vec![]),
                        ("push_position", row.pos.to_vec(), vec![]),
                        (
                            "push_a",
                            vec![u32::from_le_bytes(
                                before[PUSH_A..PUSH_A + 4].try_into().unwrap(),
                            )],
                            vec![],
                        ),
                        ("push_colour", vec![0xFF00_0000], vec![]),
                        ("push_row_word", vec![row.colour], vec![]),
                        ("push_one", vec![], vec![]),
                        (
                            "push_width",
                            vec![row.width_base.to_bits(), row.width_add.to_bits()],
                            vec![],
                        ),
                        (
                            "push_opacity",
                            vec![u32::from_le_bytes(
                                before[PUSH_B..PUSH_B + 4].try_into().unwrap(),
                            )],
                            vec![],
                        ),
                    ];
                    if use_scratch != 0 {
                        expect_lift.push(("convert_text", vec![], text));
                    } else {
                        expect_lift.push(("resolve_cached", vec![], text));
                    }
                    let mut submit_words = vec![row.corner_a.to_bits(), row.corner_b.to_bits()];
                    submit_words.extend_from_slice(&group);
                    expect_lift.push(("submit", submit_words, vec![]));
                    expect_lift.push(("end_frame", vec![], vec![]));
                    check_lift(&fake.log, &expect_lift);
                    // Wrong lift: no draw-mode override.
                    if wrong::emit_style(row.tag) != style {
                        caught += 1;
                    }
                }
                cases += 1;
                let _ = cached_img;
            }
        }
        assert!(cases > 100, "emit grid too small: {cases}");
        assert!(caught > 0, "wrong emit style never caught");
    }

    #[test]
    fn diff_measure() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xEA83);
        let mut caught = 0;
        let mut cases = 0;
        // Display extents: pairs that differ, so the aspect shows.
        let extent_sets: [[u32; 4]; 4] = [
            [800, 600, 1024, 768],
            [0, 1, 1, 0],
            [1920, 1080, 640, 720],
            [1, 0xFFFF_FFFF, 7, 0x8000_0000],
        ];
        for trial in 0..64u32 {
            let wmode = [0u8, 1, 0xFF][(trial % 3) as usize];
            let vis_ans = [0u32, 1, 0x100, 0x1FF][(trial % 4) as usize];
            let visible = vis_ans & 0xFF != 0;
            let styled = if trial % 3 == 0 { 1u8 } else { 0 };
            let (ui_mode, ui_alt) = match trial % 4 {
                0 => (0u8, 0u8),
                1 => (0x6A, 0),
                2 => (0, 2),
                _ => (1, 0),
            };
            let pick_a = [0u32, 1, 0x100, 0x201][(trial % 4) as usize];
            let pick_b = [1u32, 0, 0x1FF, 0x300][(trial % 4) as usize];
            let extents = extent_sets[(trial % 4) as usize];
            let height = if trial < 14 {
                F32_EDGE[trial as usize]
            } else {
                rng.u32()
            };
            let line = rng.u32();
            let adv = rng.u32();
            let scale_mul = if trial % 2 == 0 {
                [0x3F80_0000, 0x4000_0000, 0, 0x8000_0000][(trial / 2 % 4) as usize]
            } else {
                rng.u32()
            };
            let scale_base = rng.u32();
            let scale_div = if trial % 5 == 0 { 0 } else { rng.u32() };
            let group: [u32; 8] = [
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
                rng.u32(),
            ];
            let sink_ans = rng.u32();
            let mut fx = Fixture::build(&mut rng, 1);
            fx.obj.w8(WMODE, wmode);
            fx.obj.w8(STYLED, styled);
            fx.plant(SLOT_VIS, fx.stubs.vis);
            fx.plant(SLOT_ADV, fx.stubs.adv);
            fx.plant(SLOT_SINK_H, fx.stubs.sink_h);
            fx.plant(SLOT_SINK_L, fx.stubs.sink_l);
            fx.plant(SLOT_SINK_SCL, fx.stubs.sink_s);
            fx.plant(SLOT_SINK_PRI, fx.stubs.sink_pri);
            let cached_img = Image::from_vec({
                let mut v = vec![0u8; 32];
                for (k, w) in group.iter().enumerate() {
                    v[k * 4..k * 4 + 4].copy_from_slice(&w.to_le_bytes());
                }
                v
            });
            let mut s = lift_of(&fx.obj, 1);
            let mut fake = Fake::new();
            fake.answer("visible", vec![vis_ans]);
            fake.answer("advance", vec![adv]);
            fake.answer_groups(vec![group]);
            fake.answer("query_height", vec![height]);
            fake.answer("line_height", vec![line]);
            fake.answer("pick_ext_a", vec![pick_a]);
            fake.answer("pick_ext_b", vec![pick_b]);
            fake.answer("sink_primary", vec![sink_ans]);
            let inputs = MeasureInputs {
                ui_mode,
                ui_mode_alt: ui_alt,
                scale_mul: f32::from_bits(scale_mul),
                scale_base: f32::from_bits(scale_base),
                scale_div: f32::from_bits(scale_div),
                extents,
            };
            let before = fx.obj.buf.to_vec();
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![0, 0]),
                (2, StubKind::Cdecl2, vec![0]),
                (5, StubKind::Cdecl1, vec![0]),
                (6, StubKind::Cdecl2, vec![0]),
                (7, StubKind::Cdecl1, vec![0]),
                (8, StubKind::Cdecl1, vec![0]),
                (9, StubKind::Thiscall2, vec![cached_img.addr()]),
                (10, StubKind::Cdecl2F32Snap8, vec![height]),
                (11, StubKind::Cdecl2Write8, vec![0]),
                (12, StubKind::Cdecl0F32, vec![line]),
                (16, StubKind::Cdecl0, vec![pick_a]),
                (17, StubKind::Cdecl0, vec![pick_b]),
                (19, StubKind::Cdecl0, vec![0]),
            ]);
            rt::set_out_words(&[(11, vec![group])]);
            rt::set_virtual(&[
                ("vis", vec![vis_ans]),
                ("adv", vec![adv]),
                ("sink_h", vec![0]),
                ("sink_l", vec![0]),
                ("sink_s", vec![0]),
                ("sink_primary", vec![sink_ans]),
            ]);
            rt::set_global(G_UIMODE, ui_mode as u32);
            rt::set_global(G_UIALT, ui_alt as u32);
            rt::set_global(G_MUL, scale_mul);
            rt::set_global(G_DEFAULT, scale_base);
            rt::set_global(G_DIV, scale_div);
            rt::set_global(G_EXT_A, extents[0]);
            rt::set_global(G_EXT_B, extents[1]);
            rt::set_global(G_EXT_C, extents[2]);
            rt::set_global(G_EXT_D, extents[3]);
            rt::set_cache_addr(fx.cache.addr());
            let r = unsafe { fn_00db7500::rw_db7500(fx.this()) };
            let got = s.measure(&mut fake, &inputs);
            assert_eq!(r, sink_ans);
            assert_eq!(got, sink_ans);
            // The measure pass writes nothing to the object directly.
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let this = fx.this();
            let live_text: Vec<u8> = {
                let mut end = TEXT;
                while before[end] != 0 {
                    end += 1;
                }
                before[TEXT..=end].to_vec()
            };
            // Virtual calls in order.
            let virtual_calls = rt::take_virtual();
            let mut expect_virtual: Vec<(&str, Vec<u32>)> = vec![("vis", vec![this])];
            if visible {
                expect_virtual.push(("adv", vec![this]));
            }
            // The sink bits travel from both sides; compare them below.
            assert_eq!(virtual_calls[0], ("vis".to_string(), vec![this]));
            let sink_calls = if visible {
                assert_eq!(virtual_calls[1], ("adv".to_string(), vec![this]));
                &virtual_calls[2..]
            } else {
                &virtual_calls[1..]
            };
            assert_eq!(sink_calls.len(), 4, "four sink slots");
            assert_eq!(sink_calls[0].0, "sink_h");
            assert_eq!(sink_calls[1].0, "sink_l");
            assert_eq!(sink_calls[2].0, "sink_s");
            assert_eq!(sink_calls[3].0, "sink_primary");
            // Numbered calls in order.
            let numbered = rt::take_numbered();
            let mut ni = 0;
            assert_eq!(numbered[ni], (1, vec![this]));
            ni += 1;
            if wmode != 0 {
                assert_eq!(
                    numbered[ni],
                    (
                        2,
                        vec![
                            0,
                            u32::from_le_bytes(before[SCALE..SCALE + 4].try_into().unwrap())
                        ]
                    )
                );
                ni += 1;
            }
            if visible {
                assert_eq!(numbered[ni], (1, vec![this]));
                ni += 1;
            }
            let style = if (ui_mode == 0x6A || ui_alt != 0) && styled == 0 {
                2
            } else {
                u32::from_le_bytes(before[TAG..TAG + 4].try_into().unwrap())
            };
            let pos0 = u32::from_le_bytes(before[POS..POS + 4].try_into().unwrap());
            let pos1 = u32::from_le_bytes(before[POS + 4..POS + 8].try_into().unwrap());
            let push_a = u32::from_le_bytes(before[PUSH_A..PUSH_A + 4].try_into().unwrap());
            let push_b = u32::from_le_bytes(before[PUSH_B..PUSH_B + 4].try_into().unwrap());
            assert_eq!(numbered[ni], (5, vec![style]));
            ni += 1;
            assert_eq!(numbered[ni], (6, vec![pos0, pos1]));
            ni += 1;
            assert_eq!(numbered[ni], (7, vec![push_a]));
            ni += 1;
            assert_eq!(numbered[ni], (8, vec![push_b]));
            ni += 1;
            let text_addr = this.wrapping_add(TEXT as u32);
            if styled == 0 {
                assert_eq!(numbered[ni], (9, vec![fx.cache.addr(), text_addr]));
                ni += 1;
                assert_eq!(numbered[ni].0, 10);
                assert_eq!(numbered[ni].1[0], cached_img.addr());
                assert_eq!(numbered[ni].1[1], 1);
                ni += 1;
                assert_eq!(rt::take_snaps(), vec![group.to_vec()]);
            } else {
                assert_eq!(numbered[ni].0, 11);
                assert_eq!(numbered[ni].1[0], text_addr);
                assert_ne!(numbered[ni].1[1], 0, "scratch address");
                ni += 1;
                assert_eq!(numbered[ni].0, 10);
                assert_ne!(numbered[ni].1[0], 0, "converted words");
                assert_eq!(numbered[ni].1[1], 1);
                ni += 1;
                assert_eq!(rt::take_snaps(), vec![group.to_vec(), group.to_vec()]);
            }
            assert_eq!(numbered[ni], (12, vec![]));
            ni += 1;
            assert_eq!(numbered[ni], (16, vec![]));
            ni += 1;
            assert_eq!(numbered[ni], (17, vec![]));
            ni += 1;
            assert_eq!(numbered[ni], (19, vec![]));
            ni += 1;
            assert_eq!(ni, numbered.len(), "no further backend calls");
            // Lift log in order.
            let mut expect_lift: Vec<(&str, Vec<u32>, Vec<u8>)> =
                vec![("refresh_base", vec![], vec![])];
            if wmode != 0 {
                expect_lift.push((
                    "push_scale",
                    vec![u32::from_le_bytes(
                        before[SCALE..SCALE + 4].try_into().unwrap(),
                    )],
                    vec![],
                ));
            }
            expect_lift.push(("visible", vec![], vec![]));
            if visible {
                expect_lift.push(("refresh_base", vec![], vec![]));
                expect_lift.push(("advance", vec![], vec![]));
            }
            expect_lift.push(("push_style", vec![style], vec![]));
            expect_lift.push(("push_position", vec![pos0, pos1], vec![]));
            expect_lift.push(("push_a", vec![push_a], vec![]));
            expect_lift.push(("push_opacity", vec![push_b], vec![]));
            if styled == 0 {
                expect_lift.push(("resolve_cached", vec![], live_text));
            } else {
                expect_lift.push(("convert_text", vec![], live_text));
            }
            // Both sides saw the answers through float-stack returns.
            let qheight = quiet_snan(f32::from_bits(height)).to_bits();
            let qline = quiet_snan(f32::from_bits(line)).to_bits();
            expect_lift.push(("query_height", group.to_vec(), vec![]));
            expect_lift.push(("line_height", vec![], vec![]));
            expect_lift.push(("sink_height", vec![qheight], vec![]));
            expect_lift.push(("sink_line", vec![qline], vec![]));
            // The scaled triple and the final sink: both sides computed
            // them; the virtual log holds the rewrite's bits, the lift
            // log the lift's. They must agree bit for bit.
            let lift_scaled = fake
                .log
                .iter()
                .find(|c| c.name == "sink_scaled")
                .expect("lift sinks the scaled triple")
                .words[0];
            let lift_final = fake
                .log
                .iter()
                .find(|c| c.name == "sink_primary")
                .expect("lift sinks the final height")
                .words[0];
            assert_eq!(sink_calls[0].1, vec![this, qheight], "sunk height");
            assert_eq!(sink_calls[1].1, vec![this, qline], "sunk line");
            assert_eq!(sink_calls[2].1, vec![this, lift_scaled], "scaled triple");
            assert_eq!(sink_calls[3].1, vec![this, lift_final], "final sink");
            expect_lift.push(("sink_scaled", vec![lift_scaled], vec![]));
            expect_lift.push(("pick_ext_a", vec![], vec![]));
            expect_lift.push(("pick_ext_b", vec![], vec![]));
            expect_lift.push(("sink_primary", vec![lift_final], vec![]));
            expect_lift.push(("end_frame", vec![], vec![]));
            check_lift(&fake.log, &expect_lift);
            let _ = expect_virtual;
            // Wrong lift: extent pair swapped.
            let ext_a = extents[if pick_a & 0xFF != 0 { 1 } else { 0 }];
            let ext_b = extents[if pick_b & 0xFF != 0 { 3 } else { 2 }];
            let w = wrong::scaled_swapped(
                f32::from_bits(scale_base),
                f32::from_bits(qheight),
                f32::from_bits(scale_div),
                ext_a,
                ext_b,
            );
            if w.to_bits() != lift_final {
                caught += 1;
            }
            cases += 1;
            let _ = cached_img;
        }
        assert!(cases >= 64, "measure grid too small: {cases}");
        assert!(caught > 0, "wrong aspect never caught");
    }
}
