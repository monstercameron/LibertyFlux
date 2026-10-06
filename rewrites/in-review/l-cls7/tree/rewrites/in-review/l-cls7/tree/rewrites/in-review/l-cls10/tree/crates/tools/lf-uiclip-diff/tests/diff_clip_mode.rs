//! Differential cases: the clip's display-mode switch.
//!
//! The lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order, with a deliberately wrong lift that must
//! be caught. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_input_frontend::ui_clip::{MatchOut, TransformRecord};
    use lf_uiclip_diff::rewrites::*;
    use lf_uiclip_diff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Mint, Rng, Stubs, VTable, FLAG, MODE, OBJ_SIZE, PART, PART2, SLOT_FWD,
        SLOT_TRANSFORM, STORED, assert_only_changed, check_lift, check_numbered,
        check_virtual_names,
    };

    struct Fixture {
        obj: Image,
        part_obj: Image,
        part_vt: VTable,
        part2_obj: Image,
        part2_vt: VTable,
        stubs: Stubs,
        table_img: Image,
        entries: Vec<Image>,
        records: Vec<Image>,
        srcs: Vec<Image>,
        h1_ptr: Image,
        e1_ptr: Image,
        r1: Image,
        h2_ptr: Image,
        e2_ptr: Image,
        r2: Image,
    }

    impl Fixture {
        // Builds the clip plus the fabricated entry chains. `seeds` the
        // service-byte images; record contents come from `rng`.
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let mut part_obj = Image::random(0x10, rng);
            let part_vt = VTable::random(0x240 / 4, rng);
            let mut part2_obj = Image::random(0x10, rng);
            let part2_vt = VTable::random(0x240 / 4, rng);
            let stubs = Stubs::new();
            part_obj.w32(0, part_vt.addr());
            part2_obj.w32(0, part2_vt.addr());
            obj.w32(PART, part_obj.addr());
            obj.w32(PART2, part2_obj.addr());
            // Entry table: four entry pointers; each entry points at a
            // record whose transform area sits past sixteen prefix bytes.
            let mut entries = Vec::new();
            let mut records = Vec::new();
            for _ in 0..4 {
                entries.push(Image::random(4, rng));
                records.push(Image::random(0x28, rng));
            }
            for (e, r) in entries.iter_mut().zip(records.iter()) {
                e.w32(0, r.addr());
            }
            let mut table_img = Image::zeros(16);
            for (i, e) in entries.iter().enumerate() {
                table_img.w32(i * 4, e.addr());
            }
            let mut srcs = Vec::new();
            for _ in 0..6 {
                srcs.push(Image::random(24, rng));
            }
            // The two direct chains behind the second part's handles.
            let mut h1_ptr = Image::zeros(4);
            let mut e1_ptr = Image::zeros(4);
            let r1 = Image::random(0x28, rng);
            e1_ptr.w32(0, r1.addr());
            h1_ptr.w32(0, e1_ptr.addr());
            let mut h2_ptr = Image::zeros(4);
            let mut e2_ptr = Image::zeros(8);
            let r2 = Image::random(0x28, rng);
            e2_ptr.w32(4, r2.addr());
            h2_ptr.w32(0, e2_ptr.addr());
            Self {
                obj,
                part_obj,
                part_vt,
                part2_obj,
                part2_vt,
                stubs,
                table_img,
                entries,
                records,
                srcs,
                h1_ptr,
                e1_ptr,
                r1,
                h2_ptr,
                e2_ptr,
                r2,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    fn record_of(img: &Image) -> TransformRecord {
        let mut prefix = [0u8; 16];
        let mut transform = [0u8; 24];
        prefix.copy_from_slice(&img.buf[..16]);
        transform.copy_from_slice(&img.buf[16..40]);
        TransformRecord { prefix, transform }
    }

    fn src_bytes(img: &Image) -> [u8; 24] {
        img.buf[..24].try_into().unwrap()
    }

    mod wrong {
        use lf_core::Handle32;
        use lf_input_frontend::ui_clip::{ClipWorld, PartTag};

        // Replays the mode switch but zeroes the mode-2 selector pairs.
        pub fn set_display_mode<W: ClipWorld>(
            world: &mut W,
            part: Handle32<PartTag>,
            part2: Handle32<PartTag>,
            current: u8,
            mode: u8,
        ) -> u8 {
            if current == mode {
                return mode;
            }
            if mode == 0 || mode == 2 || mode == 1 {
                let handle = world.fetch_handle(part);
                let matched = world.match_entries(handle);
                if mode == 1 {
                    for index in 0..4 {
                        let bytes = world.transform_bytes(0, 0);
                        world.entry_record(matched.table, index).transform.copy_from_slice(&bytes);
                        world.release_service();
                    }
                    for sel in [0x40e0_0000u32, 0x4140_0000u32] {
                        let handle = world.fetch_handle(part2);
                        let entry = world.direct_entry(handle);
                        let bytes = world.transform_bytes(0, sel);
                        world.direct_record(entry).transform.copy_from_slice(&bytes);
                        world.release_service();
                    }
                } else if mode == 0 {
                    for index in 0..4 {
                        let bytes = world.transform_bytes(0, 0);
                        world.entry_record(matched.table, index).transform.copy_from_slice(&bytes);
                        world.release_service();
                    }
                    world.forward_to_part(part, 0);
                } else {
                    // Wrong: all-zero selectors.
                    for index in 0..4 {
                        let bytes = world.transform_bytes(0, 0);
                        world.entry_record(matched.table, index).transform.copy_from_slice(&bytes);
                        world.release_service();
                    }
                }
                if matched.gate & 0xffff_0000 != 0 {
                    world.teardown_entries(matched.table);
                }
            }
            mode
        }
    }

    #[test]
    fn diff_set_display_mode() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1330);
        let mut mint = Mint::new();
        let mut caught = 0;
        // (current mode, requested mode, gate word)
        let mut cases: Vec<(u8, u8, u32)> = vec![
            (0, 0, 0),
            (1, 1, 0),
            (2, 2, 0),
            (5, 5, 0),
            (0, 1, 0),
            (0, 1, 0x1_0000),
            (1, 0, 0),
            (1, 0, 0x8000_0000),
            (2, 0, 0xFFFF),
            (0, 2, 0),
            (0, 2, 0x1234_5678),
            (1, 2, 0),
            (3, 0, 0),
            (3, 1, 0xFFFF_0000),
            (3, 2, 0),
            (0, 3, 0),
            (4, 5, 0),
            (2, 7, 0xFFFF_FFFF),
        ];
        for _ in 0..12 {
            cases.push((rng.u8(), rng.u8(), rng.u32()));
        }
        for (current, mode, gate) in cases {
            eprintln!("case current={current} mode={mode} gate={gate:#x}");
            let mut fx = Fixture::build(&mut rng);
            fx.obj.w8(MODE, current);
            fx.part_vt.set(SLOT_TRANSFORM, fx.stubs.transform);
            fx.part_vt.set(SLOT_FWD, fx.stubs.fwd);
            fx.part2_vt.set(SLOT_TRANSFORM, fx.stubs.transform);
            let part = mint.next();
            let part2 = mint.next();
            let table_cookie = mint.next();
            let direct1 = mint.next();
            let direct2 = mint.next();
            let mut clip = lf_input_frontend::ui_clip::BasicClip::new(
                fx.obj.r8(FLAG),
                current,
                f32::from_bits(fx.obj.r32(STORED)),
                None,
                Some(part),
                Some(part2),
                None,
                Vec::new(),
            );
            let handle = rng.u32();
            let table_ptr = fx.table_img.addr();
            let nsvc = if mode == 1 && current != mode { 6 } else { 4 };
            let src_addrs: Vec<u32> = fx.srcs.iter().take(nsvc).map(|s| s.addr()).collect();
            rt::set_script(&[
                (2, StubKind::Thiscall2Out2, vec![0]),
                (3, StubKind::Thiscall3, src_addrs.clone()),
                (4, StubKind::Thiscall1, vec![0; 6]),
                (6, StubKind::Cdecl1, vec![0]),
            ]);
            rt::set_out_pairs(&[(2, vec![[table_ptr, gate]])]);
            rt::set_virtual(&[
                ("transform", vec![handle, fx.h1_ptr.addr(), fx.h2_ptr.addr()]),
                ("fwd", vec![0]),
            ]);
            let before = fx.obj.buf.to_vec();
            let got: u32 = unsafe { fn_00dd98d0::rw_00dd98d0(fx.this(), u32::from(mode)) };
            eprintln!("  rewrite ok");
            assert_eq!(got, 0);
            assert_eq!(fx.obj.r8(MODE), mode);
            assert_only_changed(&before, &fx.obj.buf, &[(MODE, 1)]);
            let nlog = rt::take_numbered();
            let vlog = rt::take_virtual();

            let refreshes = current != mode && (mode == 0 || mode == 1 || mode == 2);
            if !refreshes {
                check_numbered(nlog, &[]);
                check_virtual_names(&vlog, &[]);
            } else {
                // One matcher call, then service/release pairs, then the
                // gated teardown. Stack addresses are skipped; every
                // service scratch word must be the same buffer.
                assert_eq!(nlog[0].0, 2, "matcher first");
                assert_eq!(nlog[0].1[1], handle, "matcher handle");
                let mut ni = 1;
                let mut scratch = None;
                let nslots = if mode == 1 { 6 } else { 4 };
                for _ in 0..nslots {
                    assert_eq!(nlog[ni].0, 3, "service call");
                    let buf = nlog[ni].1[0];
                    scratch.get_or_insert(buf);
                    assert_eq!(scratch, Some(buf), "one scratch buffer");
                    ni += 1;
                    assert_eq!(nlog[ni], (4, vec![buf]), "release call");
                    ni += 1;
                }
                // Selector pairs on the service calls.
                let sels: Vec<(u32, u32)> = if mode == 1 {
                    vec![
                        (0, 0),
                        (0, 0),
                        (0, 0),
                        (0, 0),
                        (0, 0x40e0_0000),
                        (0, 0x4140_0000),
                    ]
                } else if mode == 0 {
                    vec![(0, 0), (0, 0), (0, 0), (0, 0)]
                } else {
                    vec![(0, 0xc140_0000), (0x4140_0000, 0), (0, 0), (0xc140_0000, 0)]
                };
                for (k, (s0, s1)) in sels.iter().enumerate() {
                    let args = &nlog[1 + k * 2].1;
                    assert_eq!((args[1], args[2]), (*s0, *s1), "slot {k} selectors");
                }
                if gate & 0xffff_0000 != 0 {
                    assert_eq!(nlog[ni], (6, vec![table_ptr]), "teardown call");
                    ni += 1;
                }
                assert_eq!(nlog.len(), ni, "no further numbered calls");
                // Virtual calls: the handle fetch, the mode-one direct
                // fetches, and the mode-zero notify.
                let mut want_v: Vec<(&str, Vec<u32>)> =
                    vec![("transform", vec![fx.part_obj.addr()])];
                if mode == 1 {
                    want_v.push(("transform", vec![fx.part2_obj.addr()]));
                    want_v.push(("transform", vec![fx.part2_obj.addr()]));
                }
                if mode == 0 {
                    want_v.push(("fwd", vec![fx.part_obj.addr(), 0]));
                }
                check_virtual_names(&vlog, &want_v);
            }

            // The lifted run over the same record bytes.
            let mut fake = Fake::new();
            fake.answer("fetch_handle", vec![handle, fx.h1_ptr.addr(), fx.h2_ptr.addr()]);
            fake.answer_matches(vec![MatchOut {
                table: table_cookie,
                gate,
            }]);
            let xforms: Vec<[u8; 24]> =
                fx.srcs.iter().take(nsvc).map(src_bytes).collect();
            fake.answer_xforms(xforms);
            fake.answer_directs(vec![direct1.get(), direct2.get()]);
            fake.add_table(
                table_cookie,
                fx.records.iter().map(record_of).collect(),
            );
            fake.add_direct(direct1, record_of(&fx.r1));
            fake.add_direct(direct2, record_of(&fx.r2));
            fake.answer("forward_to_part", vec![0]);
            clip.set_display_mode(&mut fake, mode);
            assert_eq!(clip.mode(), mode);
            // Records agree on every byte, prefixes untouched.
            for (i, rec) in fx.records.iter().enumerate() {
                let have = &fake.table(table_cookie)[i];
                assert_eq!(&rec.buf[..16], &have.prefix, "record {i} prefix");
                assert_eq!(&rec.buf[16..40], &have.transform, "record {i} bytes");
            }
            if refreshes && mode == 1 {
                let d1 = fake.direct(direct1);
                assert_eq!(&fx.r1.buf[..16], &d1.prefix, "direct 1 prefix");
                assert_eq!(&fx.r1.buf[16..40], &d1.transform, "direct 1 bytes");
                let d2 = fake.direct(direct2);
                assert_eq!(&fx.r2.buf[..16], &d2.prefix, "direct 2 prefix");
                assert_eq!(&fx.r2.buf[16..40], &d2.transform, "direct 2 bytes");
            }
            let mut expect_l: Vec<(&str, Vec<u32>, Vec<u8>)> = Vec::new();
            if refreshes {
                expect_l.push(("fetch_handle", vec![part.get()], vec![]));
                expect_l.push(("match_entries", vec![handle], vec![]));
                let sels: Vec<(u32, u32)> = if mode == 1 {
                    vec![(0, 0), (0, 0), (0, 0), (0, 0)]
                } else if mode == 0 {
                    vec![(0, 0), (0, 0), (0, 0), (0, 0)]
                } else {
                    vec![(0, 0xc140_0000), (0x4140_0000, 0), (0, 0), (0xc140_0000, 0)]
                };
                for (index, (s0, s1)) in sels.iter().enumerate() {
                    expect_l.push(("transform_bytes", vec![*s0, *s1], vec![]));
                    expect_l.push((
                        "entry_record",
                        vec![table_cookie.get(), index as u32],
                        vec![],
                    ));
                    expect_l.push(("release_service", vec![], vec![]));
                }
                if mode == 1 {
                    for sel in [0x40e0_0000u32, 0x4140_0000u32] {
                        expect_l.push(("fetch_handle", vec![part2.get()], vec![]));
                        // (The direct-entry cookie answers are queued in
                        // order; the handle words below name the scripted
                        // chain heads.)
                        let hw = if sel == 0x40e0_0000 {
                            fx.h1_ptr.addr()
                        } else {
                            fx.h2_ptr.addr()
                        };
                        expect_l.push(("direct_entry", vec![hw], vec![]));
                        expect_l.push(("transform_bytes", vec![0, sel], vec![]));
                        let dc = if sel == 0x40e0_0000 { direct1 } else { direct2 };
                        expect_l.push(("direct_record", vec![dc.get()], vec![]));
                        expect_l.push(("release_service", vec![], vec![]));
                    }
                }
                if mode == 0 {
                    expect_l.push(("forward_to_part", vec![part.get(), 0], vec![]));
                }
                if gate & 0xffff_0000 != 0 {
                    expect_l.push(("teardown_entries", vec![table_cookie.get()], vec![]));
                }
            }
            check_lift(&fake.log, &expect_l);

            // Wrong lift: zeroed mode-2 selectors.
            let mut fake = Fake::new();
            fake.answer("fetch_handle", vec![handle, fx.h1_ptr.addr(), fx.h2_ptr.addr()]);
            fake.answer_matches(vec![MatchOut {
                table: table_cookie,
                gate,
            }]);
            let xforms: Vec<[u8; 24]> =
                fx.srcs.iter().take(nsvc).map(src_bytes).collect();
            fake.answer_xforms(xforms);
            fake.answer_directs(vec![direct1.get(), direct2.get()]);
            fake.add_table(table_cookie, fx.records.iter().map(record_of).collect());
            fake.add_direct(direct1, record_of(&fx.r1));
            fake.add_direct(direct2, record_of(&fx.r2));
            fake.answer("forward_to_part", vec![0]);
            let back = wrong::set_display_mode(&mut fake, part, part2, current, mode);
            let same_log = fake.log.len() == expect_l.len()
                && fake.log.iter().zip(expect_l.iter()).all(
                    |(have, (name, words, bytes))| {
                        have.name == *name && have.words == *words && have.bytes == *bytes
                    },
                );
            if back != mode || !same_log {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong mode lift never caught");
    }
}
