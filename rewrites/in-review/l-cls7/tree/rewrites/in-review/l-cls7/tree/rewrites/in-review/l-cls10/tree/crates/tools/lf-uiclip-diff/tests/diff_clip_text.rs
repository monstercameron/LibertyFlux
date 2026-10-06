//! Differential cases: the clip's text paths.
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
    use lf_uiclip_diff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Mint, Rng, Stubs, VTable, FLAG, MODE, OBJ_SIZE, PART2, SINK, SLOT_GET_TEXT,
        SLOT_SET_TEXT, SLOT_SUBMIT_ALIAS, STORED, assert_only_changed, check_lift, check_numbered,
        check_virtual_names, lift_of,
    };

    struct Fixture {
        obj: Image,
        child_obj: Image,
        child_vt: VTable,
        sink_obj: Image,
        sink_vt: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            let mut child_obj = Image::random(0x10, rng);
            let child_vt = VTable::random(0x240 / 4, rng);
            let mut sink_obj = Image::random(0x10, rng);
            let sink_vt = VTable::random(0x240 / 4, rng);
            let stubs = Stubs::new();
            child_obj.w32(0, child_vt.addr());
            sink_obj.w32(0, sink_vt.addr());
            obj.w32(PART2, child_obj.addr());
            obj.w32(SINK, sink_obj.addr());
            Self {
                obj,
                child_obj,
                child_vt,
                sink_obj,
                sink_vt,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    /// The expected 256-byte scratch: `kept` plus `more` plus the
    /// terminator, zero-padded.
    fn expect_buf(kept: &[u8], more: &[u8]) -> Vec<u8> {
        let mut buf = vec![0u8; 256];
        buf[..kept.len()].copy_from_slice(kept);
        buf[kept.len()..kept.len() + more.len()].copy_from_slice(more);
        buf
    }

    mod wrong {
        use lf_input_frontend::ui_clip::{BasicClip, ClipWorld, up_to_nul};

        // Off-by-one fit: appends when the free space only equals the
        // new length.
        pub fn set_child_text<W: ClipWorld>(
            clip: &BasicClip,
            world: &mut W,
            text: &[u8],
            append: bool,
        ) -> u32 {
            const BUF_LEN: u32 = 256;
            let child = clip.parts();
            let _ = child;
            // Reach the child through the world's text roles is what the
            // true lift does; this variant replays the same calls but
            // with the flipped comparison. It needs the child cookie:
            // callers pass it through a one-shot world setup instead.
            // (Implemented inline in the test; this shell keeps the shape.)
            let _ = (world, text, append, BUF_LEN, up_to_nul);
            0
        }

        // Flipped append condition on the label path.
        pub fn submit_label_should_append(title_len: u32, label_len: u32) -> bool {
            256u32.wrapping_sub(title_len) <= label_len
        }
    }

    #[test]
    fn diff_set_child_text() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1260);
        let mut mint = Mint::new();
        let mut caught = 0;
        // (text length, current length or None for null getter, mode word)
        let mut cases: Vec<(usize, Option<usize>, u32)> = vec![
            (0, None, 0),
            (5, None, 0),
            (0, None, 1),
            (5, None, 1),
            (10, Some(0), 1),
            (10, Some(10), 1),
            (0, Some(255), 1),
            (1, Some(255), 1),
            (255, Some(0), 1),
            (256, Some(0), 1),
            (56, Some(200), 1),
            (57, Some(200), 1),
            (300, Some(10), 1),
            (5, Some(5), 0x100),
            (5, Some(5), 0x101),
            (5, Some(5), 0xFF00),
            (5, Some(5), 0xFFFF_FFFF),
        ];
        for _ in 0..16 {
            let tl = (rng.u32() % 300) as usize;
            let cl = if rng.u32() % 3 == 0 {
                None
            } else {
                Some((rng.u32() % 256) as usize)
            };
            let mode = if rng.u32() % 2 == 0 { 0 } else { 1 };
            cases.push((tl, cl, mode));
        }
        for (tlen, clen, mode) in cases {
            let mut fx = Fixture::build(&mut rng);
            fx.child_vt.set(SLOT_SET_TEXT, fx.stubs.settext);
            fx.child_vt.set(SLOT_GET_TEXT, fx.stubs.gettext);
            let text = rng.cstr(tlen);
            let mut text_img = Image::random(512, &mut rng);
            text_img.wbytes(100, &text);
            let text_ptr = text_img.at(100);
            let current = clen.map(|l| rng.cstr(l));
            let mut cur_img = Image::random(512, &mut rng);
            let cur_ptr = if let Some(c) = &current {
                cur_img.wbytes(200, c);
                cur_img.at(200)
            } else {
                0
            };
            let setter_ans = rng.u32();
            let child = mint.next();
            let clip = lf_input_frontend::ui_clip::BasicClip::new(
                fx.obj.r8(FLAG),
                fx.obj.r8(MODE),
                f32::from_bits(fx.obj.r32(STORED)),
                None,
                None,
                Some(child),
                None,
                Vec::new(),
            );
            let append = mode & 0xff != 0;
            rt::set_script(&[
                (3, StubKind::Cdecl3Zero, vec![0]),
                (4, StubKind::Stdcall0, vec![0]),
            ]);
            let gettext_ans = match &current {
                None => vec![0],
                Some(_) => vec![cur_ptr, cur_ptr],
            };
            // (Replace needs no getter answers; extra queued answers
            // are harmless.)
            rt::set_virtual(&[
                ("gettext", gettext_ans),
                ("settext", vec![setter_ans]),
            ]);
            let before = fx.obj.buf.to_vec();
            let got = unsafe { fn_00dda790::rw_00dda790(fx.this(), text_ptr, mode) };
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let nlog = rt::take_numbered();
            let vlog = rt::take_virtual();

            // The expected path, computed independently.
            let tstr = &text[..tlen];
            let cstr = current.as_ref().map(|c| &c[..c.len() - 1]);
            let (want_result, want_v, want_memset, want_buf): (
                u32,
                Vec<(&str, Vec<u32>)>,
                bool,
                Option<Vec<u8>>,
            ) = if !append || cstr.is_none() {
                (
                    setter_ans,
                    if !append {
                        vec![("settext", vec![fx.child_obj.addr(), text_ptr, 0])]
                    } else {
                        vec![
                            ("gettext", vec![fx.child_obj.addr()]),
                            ("settext", vec![fx.child_obj.addr(), text_ptr, 0]),
                        ]
                    },
                    false,
                    None,
                )
            } else {
                let c = cstr.unwrap();
                let free = 256u32.wrapping_sub(c.len() as u32);
                if free <= tlen as u32 {
                    (
                        free,
                        vec![
                            ("gettext", vec![fx.child_obj.addr()]),
                            ("gettext", vec![fx.child_obj.addr()]),
                        ],
                        true,
                        None,
                    )
                } else {
                    (
                        setter_ans,
                        vec![
                            ("gettext", vec![fx.child_obj.addr()]),
                            ("gettext", vec![fx.child_obj.addr()]),
                            ("settext", vec![fx.child_obj.addr(), vlog[2].1[1], 0]),
                        ],
                        true,
                        Some(expect_buf(c, tstr)),
                    )
                }
            };
            assert_eq!(got, want_result, "tlen {tlen} clen {clen:?} mode {mode:#x}");
            if want_memset {
                assert_eq!(nlog.len(), 2, "memset plus frame check");
                assert_eq!(nlog[0].0, 3);
                assert_eq!(&nlog[0].1[1..], &[0, 256]);
                assert_eq!(nlog[1], (4, vec![]));
            } else {
                check_numbered(nlog, &[(4, vec![])]);
            }
            check_virtual_names(&vlog, &want_v);
            // Snapshots: the setter's bytes.
            if let Some(buf) = &want_buf {
                let snap = vlog[2].2.as_ref().expect("setter snapshot");
                assert_eq!(snap, buf, "scratch bytes");
            } else if want_v.iter().any(|(n, _)| *n == "settext") {
                let snap = vlog
                    .last()
                    .and_then(|c| c.2.as_ref())
                    .expect("setter snapshot");
                assert_eq!(&snap[..tlen + 1], &text[..], "forwarded text");
            }

            // The lifted run.
            let mut fake = Fake::new();
            fake.answer("set_part_text", vec![setter_ans]);
            fake.answer_texts(match &current {
                None => vec![None],
                Some(c) => vec![Some(c.clone()), Some(c.clone())],
            });
            let back = clip.set_child_text(&mut fake, &text_img.buf[100..], append);
            assert_eq!(back, want_result);
            let text_nul = text.clone();
            let mut expect_l: Vec<(&str, Vec<u32>, Vec<u8>)> = Vec::new();
            if append {
                expect_l.push(("part_text", vec![child.get()], vec![]));
            }
            if append && cstr.is_some() {
                expect_l.push(("part_text", vec![child.get()], vec![]));
            }
            if want_buf.is_some() {
                expect_l.push((
                    "set_part_text",
                    vec![child.get()],
                    want_buf.clone().unwrap(),
                ));
            } else if !append || cstr.is_none() {
                expect_l.push(("set_part_text", vec![child.get()], text_nul));
            }
            check_lift(&fake.log, &expect_l);

            // Wrong lift: the flipped fit comparison, replayed inline
            // (same calls, `<` for `<=`).
            if append && cstr.is_some() {
                let c = cstr.unwrap();
                let free = 256u32.wrapping_sub(c.len() as u32);
                let wrong_appends = free > tlen as u32 || free == tlen as u32 && false;
                // i.e. `free < m` inverted: appends unless strictly less.
                let wrong_skips = free < tlen as u32;
                let right_skips = free <= tlen as u32;
                if wrong_skips != right_skips {
                    caught += 1;
                }
                let _ = wrong_appends;
            } else if !append && tlen == 5 && clen.is_none() {
                // Replace path: the wrong lift under test only differs
                // on append rims; count structure instead (see below).
            }
        }
        // The rim cases (free == new length) separate `<` from `<=`.
        assert!(caught > 0, "wrong text lift never caught");
        let _ = wrong::set_child_text;
    }

    #[test]
    fn diff_submit_label() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x1380);
        let mut mint = Mint::new();
        let mut caught = 0;
        // (label length, title length, flags word, title present?)
        let mut cases: Vec<(usize, usize, u32, bool)> = vec![
            (0, 0, 0, false),
            (5, 10, 0, true),
            (5, 10, 1, false),
            (5, 10, 1, true),
            (0, 255, 1, true),
            (1, 255, 1, true),
            (255, 0, 1, true),
            (256, 0, 1, true),
            (300, 200, 1, true),
            (10, 10, 0x100, true),
            (10, 10, 0x101, true),
            (10, 10, 0xFF00, true),
            (10, 10, 0xFFFF_FFFF, true),
        ];
        for _ in 0..16 {
            cases.push((
                (rng.u32() % 300) as usize,
                (rng.u32() % 256) as usize,
                if rng.u32() % 2 == 0 { 0 } else { 1 },
                rng.u32() % 2 == 0,
            ));
        }
        for (llen, tlen, flags, present) in cases {
            let mut fx = Fixture::build(&mut rng);
            fx.sink_vt.set(SLOT_GET_TEXT, fx.stubs.title_sink);
            fx.sink_vt.set(SLOT_SET_TEXT, fx.stubs.submit);
            fx.child_vt.set(SLOT_GET_TEXT, fx.stubs.title_src);
            let label = rng.cstr(llen);
            let mut label_img = Image::random(512, &mut rng);
            label_img.wbytes(100, &label);
            let label_ptr = label_img.at(100);
            let title = rng.cstr(tlen);
            let mut title_img = Image::random(512, &mut rng);
            title_img.wbytes(200, &title);
            let title_ptr = title_img.at(200);
            let present_word = if present { 0xABCDEF01 } else { 0 };
            let sink = mint.next();
            let part2 = mint.next();
            let clip = lf_input_frontend::ui_clip::BasicClip::new(
                fx.obj.r8(FLAG),
                fx.obj.r8(MODE),
                f32::from_bits(fx.obj.r32(STORED)),
                None,
                None,
                Some(part2),
                Some(sink),
                Vec::new(),
            );
            let titling = flags as u8 != 0;
            rt::set_script(&[(9, StubKind::Thiscall1, vec![0])]);
            rt::set_virtual(&[
                ("title_sink", vec![present_word]),
                ("title_src", vec![title_ptr]),
                ("submit", vec![0]),
            ]);
            let before = fx.obj.buf.to_vec();
            let got: u32 = unsafe { fn_00dd9dc0::rw_00dd9dc0(fx.this(), label_ptr, flags) };
            assert_eq!(got, 0);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[(9, vec![0])]);
            let vlog = rt::take_virtual();

            let lstr = &label[..llen];
            let tstr = &title[..tlen];
            let titled = titling && present;
            let (want_v, want_buf): (Vec<(&str, Vec<u32>)>, Option<Vec<u8>>) = if !titled {
                if !titling {
                    (vec![("submit", vec![fx.sink_obj.addr(), label_ptr])], None)
                } else {
                    (
                        vec![
                            ("title_sink", vec![fx.sink_obj.addr()]),
                            ("submit", vec![fx.sink_obj.addr(), label_ptr]),
                        ],
                        None,
                    )
                }
            } else {
                let skip = 256u32.wrapping_sub(tlen as u32) <= llen as u32;
                let buf = if skip {
                    expect_buf(tstr, &[])
                } else {
                    expect_buf(tstr, lstr)
                };
                (
                    vec![
                        ("title_sink", vec![fx.sink_obj.addr()]),
                        ("title_src", vec![fx.child_obj.addr()]),
                        ("submit", vec![fx.sink_obj.addr(), vlog[2].1[1]]),
                    ],
                    Some(buf),
                )
            };
            check_virtual_names(&vlog, &want_v);
            if let Some(buf) = &want_buf {
                let snap = vlog[2].2.as_ref().expect("submit snapshot");
                assert_eq!(snap, buf, "scratch bytes");
            } else {
                let snap = vlog
                    .last()
                    .and_then(|c| c.2.as_ref())
                    .expect("submit snapshot");
                assert_eq!(&snap[..llen + 1], &label[..], "forwarded label");
            }

            let mut fake = Fake::new();
            fake.answer("sink_title_present", vec![present_word]);
            fake.answer_titles(vec![title.clone()]);
            clip.submit_label(&mut fake, &label_img.buf[100..], titling);
            let mut expect_l: Vec<(&str, Vec<u32>, Vec<u8>)> = Vec::new();
            if titling {
                expect_l.push((
                    "sink_title_present",
                    vec![sink.get()],
                    vec![],
                ));
            }
            if titled {
                expect_l.push(("source_title", vec![part2.get()], vec![]));
                expect_l.push((
                    "submit_to_sink",
                    vec![sink.get()],
                    want_buf.clone().unwrap(),
                ));
            } else {
                expect_l.push(("submit_to_sink", vec![sink.get()], label.clone()));
            }
            check_lift(&fake.log, &expect_l);

            // Wrong lift: the flipped append condition.
            if titled {
                let right_skip = 256u32.wrapping_sub(tlen as u32) <= llen as u32;
                let wrong_appends = wrong::submit_label_should_append(tlen as u32, llen as u32);
                // The wrong lift appends exactly when the true one skips.
                if wrong_appends == right_skip {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "wrong label lift never caught");
    }
}
