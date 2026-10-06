//! Differential cases: the clip's state methods.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_uiclip_diff::rewrites::*;
    use lf_uiclip_diff::rt::{self};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        F32_EDGE, FLAG, Fake, Image, MODE, Mint, OBJ_SIZE, PART, PART2, Rng, SLOT_ACTION, SLOT_FWD,
        SLOT_PRED, SLOT_PROBE, STORED, SUBMIT, Stubs, U32_EDGE, VTable, assert_only_changed,
        check_lift, check_numbered, check_virtual_names, cookie, lift_of, words,
    };

    /// A test clip: the 32-bit image plus its collaborator blocks and
    /// tables. Kept alive together so addresses stay valid.
    struct Fixture {
        obj: Image,
        own_vt: VTable,
        submit_obj: Image,
        part_obj: Image,
        part2_obj: Image,
        part_vt: VTable,
        part2_vt: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let own_vt = VTable::random(0x240 / 4, rng);
            let mut submit_obj = Image::random(0x200, rng);
            let mut part_obj = Image::random(0x10, rng);
            let mut part2_obj = Image::random(0x10, rng);
            let part_vt = VTable::random(0x240 / 4, rng);
            let part2_vt = VTable::random(0x240 / 4, rng);
            let stubs = Stubs::new();
            obj.w32(0, own_vt.addr());
            submit_obj.w32(0, 0);
            part_obj.w32(0, part_vt.addr());
            part2_obj.w32(0, part2_vt.addr());
            obj.w32(SUBMIT, submit_obj.addr());
            obj.w32(PART, part_obj.addr());
            obj.w32(PART2, part2_obj.addr());
            Self {
                obj,
                own_vt,
                submit_obj,
                part_obj,
                part2_obj,
                part_vt,
                part2_vt,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_core::Handle32;
        use lf_input_frontend::ui_clip::{BasicClip, ClipWorld, PartTag};

        pub fn mode(clip: &BasicClip) -> u8 {
            clip.mode().wrapping_add(1)
        }

        pub fn stored(clip: &BasicClip) -> f32 {
            -clip.stored()
        }

        pub fn submit_word_into<W: ClipWorld>(clip: &BasicClip, world: &mut W, out: &mut u32) {
            let _ = clip;
            *out = 0xDEAD_BEEF;
            let _ = world;
        }

        pub fn store_submit_word<W: ClipWorld>(clip: &BasicClip, world: &mut W, value: u32) -> u32 {
            let _ = clip;
            let _ = world;
            value.wrapping_add(1)
        }

        pub fn probe_and_act<W: ClipWorld>(clip: &BasicClip, world: &mut W) -> u32 {
            let _ = clip;
            let ans = world.probe();
            // Wrong byte tested.
            if ans & 0xff00 != 0 {
                world.run_action()
            } else {
                ans
            }
        }

        pub fn forward_conditional<W: ClipWorld>(clip: &BasicClip, world: &mut W, arg: u32) -> u32 {
            // Mirrors the true shape but drops the mode-2 arm, and mints
            // its own part cookies instead of the clip's.
            let mut fwd = 0;
            if arg & 0xff == 1 {
                let pred = world.part_predicate(mint_a());
                if !pred || clip.mode() == 1 {
                    fwd = 1;
                }
            }
            world.forward_to_part(mint_b(), fwd)
        }

        fn mint_a() -> Handle32<PartTag> {
            Handle32::new(0x00C0_0001).unwrap()
        }
        fn mint_b() -> Handle32<PartTag> {
            Handle32::new(0x00C0_0002).unwrap()
        }
    }

    #[test]
    fn diff_mode_byte() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xC110);
        let mut caught = 0;
        // Every byte value, then random objects.
        for mode in 0..=255u32 {
            let mut fx = Fixture::build(&mut rng);
            fx.obj.w8(MODE, mode as u8);
            let clip = lift_of(&fx.obj, None, None, None, None, Vec::new());
            let before = fx.obj.buf.to_vec();
            let got = unsafe { fn_00dd9220::rw_00dd9220(fx.this()) };
            assert_eq!(got & 0xff, mode, "mode {mode}");
            assert_eq!(clip.mode(), mode as u8);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual_names(&rt::take_virtual(), &[]);
            if wrong::mode(&clip) != mode as u8 {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong mode lift never caught");
    }

    #[test]
    fn diff_sink_word() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x5EED);
        let mut caught = 0;
        for i in 0..40 {
            let mut fx = Fixture::build(&mut rng);
            // Mostly live addresses, sometimes null.
            let sink_addr = if i % 5 == 0 { 0 } else { fx.submit_obj.addr() };
            fx.obj.w32(support::SINK, sink_addr);
            let clip = lift_of(&fx.obj, None, None, None, cookie(sink_addr), Vec::new());
            let before = fx.obj.buf.to_vec();
            let got = unsafe { fn_00dd9260::rw_00dd9260(fx.this()) };
            assert_eq!(got, sink_addr);
            assert_eq!(words(clip.sink()), sink_addr);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual_names(&rt::take_virtual(), &[]);
            // Wrong lift: always answers null.
            if sink_addr != 0 {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong sink lift never caught");
    }

    #[test]
    fn diff_stored_float() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xF1A0);
        let mut caught = 0;
        let mut cases: Vec<u32> = F32_EDGE.to_vec();
        for _ in 0..40 {
            cases.push(rng.u32());
        }
        for bits in cases {
            let mut fx = Fixture::build(&mut rng);
            fx.obj.w32(STORED, bits);
            let clip = lift_of(&fx.obj, None, None, None, None, Vec::new());
            let before = fx.obj.buf.to_vec();
            let got: f32 = unsafe { fn_00dd9290::rw_00dd9290(fx.this()) };
            // The float-stack return quiets signalling NaNs on both
            // sides; everything else passes bit for bit.
            let want = lf_input_frontend::ui_clip::quiet_snan(f32::from_bits(bits)).to_bits();
            assert_eq!(got.to_bits(), want, "stored bits {bits:#x}");
            assert_eq!(clip.stored().to_bits(), want);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual_names(&rt::take_virtual(), &[]);
            if wrong::stored(&clip).to_bits() != bits {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong stored lift never caught");
    }

    #[test]
    fn diff_submit_word_copy() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1470);
        let mut mint = Mint::new();
        let mut caught = 0;
        let mut cases: Vec<u32> = U32_EDGE.to_vec();
        for _ in 0..24 {
            cases.push(rng.u32());
        }
        for word in cases {
            let mut fx = Fixture::build(&mut rng);
            fx.submit_obj.w32(0x1E0, word);
            let submit = mint.next();
            let clip = lift_of(&fx.obj, None, None, None, None, Vec::new());
            // Rebuild with the minted cookie.
            let clip = lf_input_frontend::ui_clip::BasicClip::new(
                clip.flag(),
                clip.mode(),
                clip.stored(),
                Some(submit),
                None,
                None,
                None,
                Vec::new(),
            );
            let mut out_img = Image::random(4, &mut rng);
            let before_obj = fx.obj.buf.to_vec();
            let before_sub = fx.submit_obj.buf.to_vec();
            let got = unsafe { fn_00dd92a0::rw_00dd92a0(fx.this(), out_img.addr()) };
            assert_eq!(got, out_img.addr());
            assert_eq!(out_img.r32(0), word);
            assert_only_changed(&before_obj, &fx.obj.buf, &[]);
            assert_only_changed(&before_sub, &fx.submit_obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual_names(&rt::take_virtual(), &[]);

            let mut fake = Fake::new();
            fake.answer("submit_word", vec![word]);
            let mut out = 0xA5A5_A5A5u32;
            clip.submit_word_into(&mut fake, &mut out);
            assert_eq!(out, word);
            check_lift(&fake.log, &[("submit_word", vec![submit.get()], vec![])]);

            let mut fake = Fake::new();
            fake.answer("submit_word", vec![word]);
            let mut out = 0u32;
            wrong::submit_word_into(&clip, &mut fake, &mut out);
            if out != word {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong copy lift never caught");
    }

    #[test]
    fn diff_store_submit_word() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1300);
        let mut mint = Mint::new();
        let mut caught = 0;
        let mut cases: Vec<u32> = U32_EDGE.to_vec();
        for _ in 0..24 {
            cases.push(rng.u32());
        }
        for word in cases {
            let mut fx = Fixture::build(&mut rng);
            let submit = mint.next();
            let clip = lf_input_frontend::ui_clip::BasicClip::new(
                fx.obj.r8(FLAG),
                fx.obj.r8(MODE),
                f32::from_bits(fx.obj.r32(STORED)),
                Some(submit),
                None,
                None,
                None,
                Vec::new(),
            );
            let mut src = Image::random(4, &mut rng);
            src.w32(0, word);
            let before_obj = fx.obj.buf.to_vec();
            let before_sub = fx.submit_obj.buf.to_vec();
            let got = unsafe { fn_00dda930::rw_00dda930(fx.this(), src.addr()) };
            assert_eq!(got, word);
            assert_eq!(fx.submit_obj.r32(0x1E0), word);
            assert_only_changed(&before_obj, &fx.obj.buf, &[]);
            assert_only_changed(&before_sub, &fx.submit_obj.buf, &[(0x1E0, 4)]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual_names(&rt::take_virtual(), &[]);

            let mut fake = Fake::new();
            let back = clip.store_submit_word(&mut fake, word);
            assert_eq!(back, word);
            check_lift(
                &fake.log,
                &[("set_submit_word", vec![submit.get(), word], vec![])],
            );

            let mut fake = Fake::new();
            let back = wrong::store_submit_word(&clip, &mut fake, word);
            if back != word {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong store lift never caught");
    }

    #[test]
    fn diff_probe_and_act() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1230);
        let mut caught = 0;
        // Probe answers that separate the low byte from its neighbours.
        let probes: Vec<u32> = vec![
            0,
            1,
            0xFF,
            0x100,
            0xFF00,
            0x1_0000,
            0xFFFF_FFFF,
            0xFFFF_FF00,
            0x80,
            0x8000,
        ];
        for probe in probes {
            for action in [0u32, 1, 0x1234_5678] {
                let mut fx = Fixture::build(&mut rng);
                fx.own_vt.set(SLOT_PROBE, fx.stubs.probe);
                fx.own_vt.set(SLOT_ACTION, fx.stubs.action);
                let this = fx.this();
                let clip = lift_of(&fx.obj, None, None, None, None, Vec::new());
                rt::set_script(&[]);
                rt::set_virtual(&[("probe", vec![probe]), ("action", vec![action])]);
                let before = fx.obj.buf.to_vec();
                let got = unsafe { fn_00de3e90::rw_00de3e90(this) };
                let want = if probe & 0xff != 0 { action } else { probe };
                assert_eq!(got, want, "probe {probe:#x}");
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_numbered(rt::take_numbered(), &[]);
                if probe & 0xff != 0 {
                    check_virtual_names(
                        &rt::take_virtual(),
                        &[("probe", vec![this]), ("action", vec![this, 0])],
                    );
                } else {
                    check_virtual_names(&rt::take_virtual(), &[("probe", vec![this])]);
                }

                let mut fake = Fake::new();
                fake.answer("probe", vec![probe]);
                fake.answer("run_action", vec![action]);
                let back = clip.probe_and_act(&mut fake);
                assert_eq!(back, want);
                if probe & 0xff != 0 {
                    check_lift(
                        &fake.log,
                        &[("probe", vec![], vec![]), ("run_action", vec![], vec![])],
                    );
                } else {
                    check_lift(&fake.log, &[("probe", vec![], vec![])]);
                }

                let mut fake = Fake::new();
                fake.answer("probe", vec![probe]);
                fake.answer("run_action", vec![action]);
                let back = wrong::probe_and_act(&clip, &mut fake);
                let wrong_calls_action = probe & 0xff00 != 0;
                let right_calls_action = probe & 0xff != 0;
                if back != want || wrong_calls_action != right_calls_action {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong probe lift never caught");
    }

    #[test]
    fn diff_forward_conditional() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1310);
        let mut mint = Mint::new();
        let mut caught = 0;
        for mode in [0u8, 1, 2, 3, 0xFF] {
            for arg_lo in [0u32, 1, 2, 0xFF] {
                for pred in [0u32, 1, 0x100, 0xFF00] {
                    for ans in [0u32, 0xABCDEF] {
                        let mut fx = Fixture::build(&mut rng);
                        fx.obj.w8(MODE, mode);
                        fx.part2_vt.set(SLOT_PRED, fx.stubs.pred);
                        fx.part_vt.set(SLOT_FWD, fx.stubs.fwd);
                        let arg = (rng.u32() & 0xFFFF_FF00) | arg_lo;
                        let part = mint.next();
                        let part2 = mint.next();
                        let clip = lf_input_frontend::ui_clip::BasicClip::new(
                            fx.obj.r8(FLAG),
                            mode,
                            f32::from_bits(fx.obj.r32(STORED)),
                            None,
                            Some(part),
                            Some(part2),
                            None,
                            Vec::new(),
                        );
                        rt::set_script(&[]);
                        rt::set_virtual(&[("pred", vec![pred]), ("fwd", vec![ans])]);
                        let before = fx.obj.buf.to_vec();
                        let got = unsafe { fn_00dd9860::rw_00dd9860(fx.this(), arg) };
                        assert_eq!(got, ans);
                        assert_only_changed(&before, &fx.obj.buf, &[]);
                        check_numbered(rt::take_numbered(), &[]);
                        let want_fwd = if arg_lo == 1 && (pred as u8 == 0 || mode == 1 || mode == 2)
                        {
                            1
                        } else {
                            0
                        };
                        // (arg_lo == 1 with other fwd values: the low byte
                        // alone decides the predicate path.)
                        let want_fwd = if arg & 0xff == 1 { want_fwd } else { 0 };
                        if arg & 0xff == 1 {
                            check_virtual_names(
                                &rt::take_virtual(),
                                &[
                                    ("pred", vec![fx.part2_obj.addr()]),
                                    ("fwd", vec![fx.part_obj.addr(), want_fwd]),
                                ],
                            );
                        } else {
                            check_virtual_names(
                                &rt::take_virtual(),
                                &[("fwd", vec![fx.part_obj.addr(), 0])],
                            );
                        }

                        let mut fake = Fake::new();
                        fake.answer("part_predicate", vec![pred]);
                        fake.answer("forward_to_part", vec![ans]);
                        let back = clip.forward_conditional(&mut fake, arg);
                        assert_eq!(back, ans);
                        if arg & 0xff == 1 {
                            check_lift(
                                &fake.log,
                                &[
                                    ("part_predicate", vec![part2.get()], vec![]),
                                    ("forward_to_part", vec![part.get(), want_fwd], vec![]),
                                ],
                            );
                        } else {
                            check_lift(
                                &fake.log,
                                &[("forward_to_part", vec![part.get(), 0], vec![])],
                            );
                        }

                        // Wrong lift: drops the mode-2 arm and mints its
                        // own cookies, so both its answers and its cookie
                        // words are compared.
                        let mut fake = Fake::new();
                        fake.answer("part_predicate", vec![pred]);
                        fake.answer("forward_to_part", vec![ans]);
                        let back = wrong::forward_conditional(&clip, &mut fake, arg);
                        let wrong_fwd = if arg & 0xff == 1 && (pred as u8 == 0 || mode == 1) {
                            1
                        } else {
                            0
                        };
                        let cookie_hit = fake.log.iter().any(|c| {
                            c.words.first() == Some(&part.get())
                                || c.words.first() == Some(&part2.get())
                        });
                        if back != ans || wrong_fwd != want_fwd || !cookie_hit {
                            caught += 1;
                        }
                    }
                }
            }
        }
        assert!(caught > 0, "wrong forward lift never caught");
    }
}
