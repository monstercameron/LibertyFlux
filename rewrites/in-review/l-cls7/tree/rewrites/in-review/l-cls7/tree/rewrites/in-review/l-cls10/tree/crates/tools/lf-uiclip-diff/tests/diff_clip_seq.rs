//! Differential cases: the clip's flag loop and float paths.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::ui_clip::TripleKind;
    use lf_uiclip_diff::rewrites::*;
    use lf_uiclip_diff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Mint, Rng, Stubs, VTable, F32_EDGE, FLAG, MODE, OBJ_SIZE, PART2, PARTS,
        SLOT_COUNT, SLOT_FIN, SLOT_FWD, SLOT_MEASURE, SLOT_MID1, SLOT_MID2, SLOT_SINK_PUSH, STORED,
        SUBMIT, U32_EDGE, assert_only_changed, check_lift, check_numbered, check_virtual_names,
        lift_of,
    };

    struct Fixture {
        obj: Image,
        own_vt: VTable,
        parts_array: Image,
        elements: Vec<Image>,
        part2_obj: Image,
        part2_vt: VTable,
        submit_obj: Image,
        submit_vt: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng, nelts: usize) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let own_vt = VTable::random(0x240 / 4, rng);
            let mut parts_array = Image::zeros(nelts.max(1) * 4);
            let mut elements = Vec::new();
            for _ in 0..nelts {
                elements.push(Image::random(0xD0, rng));
            }
            for (i, e) in elements.iter().enumerate() {
                parts_array.w32(i * 4, e.addr());
            }
            let mut part2_obj = Image::random(0x10, rng);
            let part2_vt = VTable::random(0x240 / 4, rng);
            let mut submit_obj = Image::random(0x10, rng);
            let submit_vt = VTable::random(0xC0 / 4, rng);
            let stubs = Stubs::new();
            obj.w32(0, own_vt.addr());
            obj.w32(PARTS, parts_array.addr());
            part2_obj.w32(0, part2_vt.addr());
            submit_obj.w32(0, submit_vt.addr());
            obj.w32(PART2, part2_obj.addr());
            obj.w32(SUBMIT, submit_obj.addr());
            Self {
                obj,
                own_vt,
                parts_array,
                elements,
                part2_obj,
                part2_vt,
                submit_obj,
                submit_vt,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    mod wrong {
        use lf_input_frontend::ui_clip::{BasicClip, ClipWorld, TripleKind};

        // Off-by-one bound: flags one element too many.
        pub fn set_flag<W: ClipWorld>(clip: &mut BasicClip, world: &mut W, flag: u8) -> u32 {
            let mut count = world.part_count();
            if count != 0 {
                let mut i: u32 = 0;
                while i <= count {
                    if let Some(e) = clip.parts().get(i as usize) {
                        world.set_element_flag(*e, flag);
                    }
                    i += 1;
                    count = world.part_count();
                }
            }
            count
        }

        // Always scales, ignoring the argument's low byte.
        pub fn forward_and_push<W: ClipWorld>(clip: &BasicClip, world: &mut W, arg: u32) -> u32 {
            use lf_input_frontend::ui_clip::{MEASURE_BIAS, MEASURE_SCALE};
            let _ = clip;
            let _ = arg;
            let v = world.measure();
            let v = v * MEASURE_SCALE - MEASURE_BIAS;
            world.push_adjusted(
                lf_core::Handle32::new(0x00C0_00D1).unwrap(),
                v.to_bits(),
            )
        }

        // Swapped splitter constants.
        pub fn triple_bytes(x: f32) -> [u8; 3] {
            use lf_input_frontend::ui_clip::{TRIPLE_C0, TRIPLE_C1, TRIPLE_C2, truncate_raw};
            let b0 = (truncate_raw(x * TRIPLE_C1) & 0xff) as u8;
            let x1 = x - f32::from(b0) * TRIPLE_C0;
            let b1 = (truncate_raw(x1) & 0xff) as u8;
            let x2 = (x1 - f32::from(b1)) * TRIPLE_C2;
            let b2 = (truncate_raw(x2) & 0xff) as u8;
            [b0, b1, b2]
        }

        pub fn set_triple<W: ClipWorld>(
            clip: &mut BasicClip,
            world: &mut W,
            kind: TripleKind,
            input: f32,
        ) -> u32 {
            world.encode_triple(kind, triple_bytes(input));
            world.run_triple_mid(kind);
            world.finish_triple();
            world.frame_check()
        }
    }

    #[test]
    fn diff_set_flag() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1240);
        let mut mint = Mint::new();
        let mut caught = 0;
        // Count scripts: empty, short, growing, and the off-by-one rim.
        let scripts: Vec<Vec<u32>> = vec![
            vec![0],
            vec![1, 0],
            vec![2, 0, 0],
            vec![1, 3, 0, 0],
            vec![3, 3, 3, 3, 0],
            vec![2, 2, 2, 0],
        ];
        for counts in &scripts {
            for flag in [0u8, 1, 0xCB, 0xFF] {
                let mut fx = Fixture::build(&mut rng, 4);
                fx.own_vt.set(SLOT_COUNT, fx.stubs.count);
                let elt_cookies: Vec<_> = (0..4).map(|_| mint.next()).collect();
                let mut clip = lift_of(
                    &fx.obj,
                    None,
                    None,
                    None,
                    None,
                    elt_cookies.clone(),
                );
                let this = fx.this();
                rt::set_script(&[]);
                rt::set_virtual(&[("count", counts.clone())]);
                let before_obj = fx.obj.buf.to_vec();
                let before_elts: Vec<Vec<u8>> =
                    fx.elements.iter().map(|e| e.buf.to_vec()).collect();
                let got = unsafe { fn_00dda4e0::rw_00dda4e0(this, u32::from(flag)) };
                // Replay the script to find the expected path.
                let mut want_flagged = Vec::new();
                let mut want_count = counts[0];
                if want_count != 0 {
                    let mut i = 0u32;
                    let mut ci = 1usize;
                    while i < want_count {
                        want_flagged.push(i);
                        i += 1;
                        want_count = *counts.get(ci).unwrap_or(&0);
                        ci += 1;
                    }
                }
                assert_eq!(got, want_count, "counts {counts:?}");
                assert_eq!(fx.obj.r8(FLAG), flag);
                for (i, e) in fx.elements.iter().enumerate() {
                    if want_flagged.contains(&(i as u32)) {
                        assert_eq!(e.r8(FLAG), flag, "element {i}");
                    } else {
                        assert_eq!(e.buf.to_vec(), before_elts[i], "element {i} untouched");
                    }
                }
                assert_only_changed(&before_obj, &fx.obj.buf, &[(FLAG, 1)]);
                check_numbered(rt::take_numbered(), &[]);
                let ncalls = 1 + want_flagged.len();
                let expect_v: Vec<(&str, Vec<u32>)> =
                    (0..ncalls).map(|_| ("count", vec![this])).collect();
                check_virtual_names(&rt::take_virtual(), &expect_v);

                let mut fake = Fake::new();
                fake.answer("part_count", counts.clone());
                let back = clip.set_flag(&mut fake, flag);
                assert_eq!(back, want_count);
                assert_eq!(clip.flag(), flag);
                let mut expect_l: Vec<(&str, Vec<u32>, Vec<u8>)> =
                    vec![("part_count", vec![], vec![])];
                for i in want_flagged.iter() {
                    expect_l.push((
                        "set_element_flag",
                        vec![elt_cookies[*i as usize].get(), u32::from(flag)],
                        vec![],
                    ));
                    expect_l.push(("part_count", vec![], vec![]));
                }
                check_lift(&fake.log, &expect_l);

                let mut clip2 = clip.clone();
                let mut fake = Fake::new();
                fake.answer("part_count", counts.clone());
                let back = wrong::set_flag(&mut clip2, &mut fake, flag);
                let wrong_flagged = fake
                    .log
                    .iter()
                    .filter(|c| c.name == "set_element_flag")
                    .count();
                if back != want_count || wrong_flagged != want_flagged.len() {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong flag lift never caught");
    }

    #[test]
    fn diff_forward_and_push() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1290);
        let mut mint = Mint::new();
        let mut caught = 0;
        let mut measures: Vec<u32> = F32_EDGE.to_vec();
        for _ in 0..24 {
            measures.push(rng.u32());
        }
        for arg_lo in [0u32, 1, 0xFF] {
            for mbits in &measures {
                let mut fx = Fixture::build(&mut rng, 1);
                fx.part2_vt.set(SLOT_FWD, fx.stubs.fwd);
                fx.own_vt.set(SLOT_MEASURE, fx.stubs.measure);
                fx.submit_vt.set(SLOT_SINK_PUSH, fx.stubs.sink_push);
                let arg = (rng.u32() & 0xFFFF_FF00) | arg_lo;
                let fwd_ans = rng.u32();
                let push_ans = rng.u32();
                let part2 = mint.next();
                let submit = mint.next();
                let clip = lf_input_frontend::ui_clip::BasicClip::new(
                    fx.obj.r8(FLAG),
                    fx.obj.r8(MODE),
                    f32::from_bits(fx.obj.r32(STORED)),
                    Some(submit),
                    None,
                    Some(part2),
                    None,
                    Vec::new(),
                );
                rt::set_script(&[]);
                rt::set_virtual(&[
                    ("fwd", vec![fwd_ans]),
                    ("measure", vec![*mbits]),
                    ("sink_push", vec![push_ans]),
                ]);
                let before = fx.obj.buf.to_vec();
                let got = unsafe { fn_00dda8b0::rw_00dda8b0(fx.this(), arg) };
                assert_eq!(got, push_ans);
                assert_only_changed(&before, &fx.obj.buf, &[]);
                check_numbered(rt::take_numbered(), &[]);
                let vlog = rt::take_virtual();
                assert_eq!(vlog.len(), 3, "three virtual calls");
                check_virtual_names(
                    &vlog,
                    &[
                        ("fwd", vec![fx.part2_obj.addr(), arg]),
                        ("measure", vec![fx.this()]),
                        ("sink_push", vec![fx.submit_obj.addr(), vlog[2].1[1]]),
                    ],
                );
                let pushed_bits = vlog[2].1[1];

                let mut fake = Fake::new();
                fake.answer("forward_to_part", vec![fwd_ans]);
                fake.answer("measure", vec![*mbits]);
                fake.answer("push_adjusted", vec![push_ans]);
                let back = clip.forward_and_push(&mut fake, arg);
                assert_eq!(back, push_ans);
                check_lift(
                    &fake.log,
                    &[
                        ("forward_to_part", vec![part2.get(), arg], vec![]),
                        ("measure", vec![], vec![]),
                        ("push_adjusted", vec![submit.get(), pushed_bits], vec![]),
                    ],
                );

                let mut fake = Fake::new();
                fake.answer("measure", vec![*mbits]);
                let back = wrong::forward_and_push(&clip, &mut fake, arg);
                let wrong_bits = fake.log[1].words[1];
                if back != push_ans || wrong_bits != pushed_bits {
                    caught += 1;
                }
                // (The wrong lift also mints its own submit cookie; its
                // cookie word differs from the clip's on every case.)
            }
        }
        // Edge words through the argument high bytes as well.
        for arg in U32_EDGE {
            let mut fx = Fixture::build(&mut rng, 1);
            fx.part2_vt.set(SLOT_FWD, fx.stubs.fwd);
            fx.own_vt.set(SLOT_MEASURE, fx.stubs.measure);
            fx.submit_vt.set(SLOT_SINK_PUSH, fx.stubs.sink_push);
            let part2 = mint.next();
            let submit = mint.next();
            let clip = lf_input_frontend::ui_clip::BasicClip::new(
                0, 0, 0.0, Some(submit), None, Some(part2), None, Vec::new(),
            );
            rt::set_script(&[]);
            rt::set_virtual(&[
                ("fwd", vec![7]),
                ("measure", vec![0x4040_0000]),
                ("sink_push", vec![9]),
            ]);
            let got = unsafe { fn_00dda8b0::rw_00dda8b0(fx.this(), arg) };
            assert_eq!(got, 9);
            let vlog = rt::take_virtual();
            let mut fake = Fake::new();
            fake.answer("forward_to_part", vec![7]);
            fake.answer("measure", vec![0x4040_0000]);
            fake.answer("push_adjusted", vec![9]);
            let back = clip.forward_and_push(&mut fake, arg);
            assert_eq!(back, 9);
            assert_eq!(fake.log[2].words[1], vlog[2].1[1], "arg {arg:#x}");
        }
        assert!(caught > 0, "wrong push lift never caught");
    }

    fn diff_triple_kind(kind: TripleKind, table: u32, seed: u64) {
        let _guard = rt::script_lock();
        let mut rng = Rng(seed);
        let mut caught = 0;
        let mut inputs: Vec<u32> = F32_EDGE.to_vec();
        for _ in 0..40 {
            inputs.push(rng.u32());
        }
        for bits in inputs {
            let mut fx = Fixture::build(&mut rng, 1);
            let mid_slot = match kind {
                TripleKind::First => SLOT_MID1,
                TripleKind::Second => SLOT_MID2,
            };
            let mid_stub = match kind {
                TripleKind::First => fx.stubs.mid1,
                TripleKind::Second => fx.stubs.mid2,
            };
            let mid_name = match kind {
                TripleKind::First => "mid1",
                TripleKind::Second => "mid2",
            };
            fx.own_vt.set(mid_slot, mid_stub);
            fx.own_vt.set(SLOT_FIN, fx.stubs.fin);
            let mut clip = lift_of(&fx.obj, None, None, None, None, Vec::new());
            let cookie = rng.u32();
            rt::set_script(&[
                (0, StubKind::Cdecl5, vec![0]),
                (3, StubKind::Cdecl0, vec![cookie]),
            ]);
            rt::set_virtual(&[(mid_name, vec![0]), ("fin", vec![0])]);
            let before = fx.obj.buf.to_vec();
            let input = f32::from_bits(bits);
            let got = unsafe {
                match kind {
                    TripleKind::First => fn_00dd9ef0::rw_00dd9ef0(fx.this(), bits),
                    TripleKind::Second => fn_00dda430::rw_00dda430(fx.this(), bits),
                }
            };
            assert_eq!(got, cookie, "input {bits:#x}");
            assert_eq!(fx.obj.r32(STORED), bits);
            assert_only_changed(&before, &fx.obj.buf, &[(STORED, 4)]);
            let nlog = rt::take_numbered();
            assert_eq!(nlog.len(), 2, "two numbered calls");
            assert_eq!(nlog[0].0, 0);
            assert_eq!(nlog[0].1.len(), 5);
            assert_eq!(nlog[0].1[1], table, "engine table");
            let (b0, b1, b2) = (nlog[0].1[2], nlog[0].1[3], nlog[0].1[4]);
            assert_eq!(nlog[1], (3, vec![]));
            let vlog = rt::take_virtual();
            assert_eq!(vlog.len(), 2, "two virtual calls");
            // Both frame pointers name the same scratch word.
            assert_eq!(vlog[0].1[1], nlog[0].1[0], "shared scratch");
            check_virtual_names(
                &vlog,
                &[(mid_name, vec![fx.this(), vlog[0].1[1], 0]), ("fin", vec![
                    fx.this(), 1,
                ])],
            );

            let mut fake = Fake::new();
            fake.answer("frame_check", vec![cookie]);
            let back = clip.set_triple(&mut fake, kind, input);
            assert_eq!(back, cookie);
            assert_eq!(clip.stored().to_bits(), bits);
            let kind_w = match kind {
                TripleKind::First => 0,
                TripleKind::Second => 1,
            };
            check_lift(
                &fake.log,
                &[
                    ("encode_triple", vec![kind_w, b0, b1, b2], vec![]),
                    ("run_triple_mid", vec![kind_w], vec![]),
                    ("finish_triple", vec![], vec![]),
                    ("frame_check", vec![], vec![]),
                ],
            );

            let mut clip2 = clip.clone();
            let mut fake = Fake::new();
            fake.answer("frame_check", vec![cookie]);
            let back = wrong::set_triple(&mut clip2, &mut fake, kind, input);
            let wb = &fake.log[0].words;
            if back != cookie || wb[1] != b0 || wb[2] != b1 || wb[3] != b2 {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong triple lift never caught ({kind:?})");
    }

    #[test]
    fn diff_set_triple_first() {
        diff_triple_kind(TripleKind::First, 0xEFB9A4, 0x1320);
    }

    #[test]
    fn diff_set_triple_second() {
        diff_triple_kind(TripleKind::Second, 0xEFB9B4, 0x1370);
    }
}
