//! Differential cases: the fetch-and-forward readers.
//!
//! One token is fetched (and discarded or checked), then a value is read
//! through a forwarded slot. Each lifted generic against its verified
//! rewrites on the same generated inputs, comparing results and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_files_memory::tokenizer::{ForwardSlot, TokenFetch, Tokenizer};
    use lf_tokendiff::rewrites::*;
    use lf_tokendiff::rt::{self, FetchScript, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F32_EDGE, Image, Rng, TokenCall, TokenFake, U32_EDGE};

    const OBJ_VT: usize = 0x00;
    const OBJ_SIZE: usize = 0x224;

    /// A fetch-forward fixture: object with a planted vtable.
    struct Fixture {
        obj: Image,
        vtable: support::VTable,
    }

    impl Fixture {
        fn build() -> Self {
            let mut obj = Image::zeroed(OBJ_SIZE);
            let vtable = support::tokenizer_vtable();
            obj.w32(OBJ_VT, vtable.addr());
            Self { obj, vtable }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        fn lift(&self) -> Tokenizer {
            Tokenizer::new(0)
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_files_memory::tokenizer::{ForwardSlot, TokenWorld, Tokenizer};

        /// Reports one past the fetched length.
        pub fn len_plus_one<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) -> u32 {
            let _ = t;
            w.fetch_token(0x40).len.wrapping_add(1)
        }

        /// Skips the discard-fetch before the flagged read.
        pub fn int_no_fetch<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) -> u32 {
            let _ = t;
            w.read_flagged_int(1)
        }

        /// Skips the discard-fetch before the float read.
        pub fn float_no_fetch<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) -> f32 {
            let _ = t;
            w.read_float(1)
        }

        /// Forwards one past the value.
        pub fn value_plus_one<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            slot: ForwardSlot,
            value: u32,
        ) -> u32 {
            let _ = t;
            w.fetch_token(0x40);
            w.forward_value(slot, value.wrapping_add(1))
        }

        /// Compares only when the fetch reports no token (inverted).
        pub fn check_inverted<W: TokenWorld>(t: &mut Tokenizer, w: &mut W, expected: &[u8]) -> u32 {
            let _ = t;
            let fetch = w.fetch_token(0x40);
            if fetch.len == 0 {
                w.compare_text(expected, &fetch.bytes);
            }
            w.read_flagged_int(1)
        }

        /// Compares only when the fetch reports no token (inverted),
        /// float tail.
        pub fn check_float_inverted<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            expected: &[u8],
        ) -> f32 {
            let _ = t;
            let fetch = w.fetch_token(0x40);
            if fetch.len == 0 {
                w.compare_text(expected, &fetch.bytes);
            }
            w.read_float(1)
        }

        /// Compares only when the fetch reports no token (inverted),
        /// value tail.
        pub fn check_value_inverted<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            expected: &[u8],
            slot: ForwardSlot,
            value: u32,
        ) -> u32 {
            let _ = t;
            let fetch = w.fetch_token(0x40);
            if fetch.len == 0 {
                w.compare_text(expected, &fetch.bytes);
            }
            w.forward_value(slot, value)
        }
    }

    /// Fetch scripts: lengths with and without tokens. Bytes stop at
    /// 63 so the zeroed frame terminates them: a full 64-byte token
    /// would over-read the frame into layout-dependent stack bytes on
    /// the rewrite side (the comparer reads to NUL), which no
    /// differential case can pin.
    fn fetch_scripts(rng: &mut Rng) -> Vec<(u32, Vec<u8>)> {
        let mut v = vec![
            (0u32, vec![]),
            (1, vec![b'a']),
            (3, vec![b'a', b'b', b'c']),
            (0x40, vec![b'z'; 0x3F]),
            (5, vec![b'h', b'e', b'l', b'l', b'o']),
        ];
        for _ in 0..4 {
            let n = rng.below(0x40);
            v.push((n, vec![b'q'; (n.min(0x3F)) as usize]));
        }
        v.push((0, vec![b'k']));
        v
    }

    /// Expected strings (bare bytes; images add the terminator).
    fn expected_strings() -> Vec<Vec<u8>> {
        vec![vec![], vec![b'a'], vec![b'a', b'b', b'c'], vec![b'x'; 32]]
    }

    #[test]
    fn forward_fetch_len_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFC30);
        let mut caught = 0;
        let mut cases = 0;
        for (len, bytes) in fetch_scripts(&mut rng) {
            let fx = Fixture::build();
            rt::set_script(&[]);
            rt::push_fetch(vec![FetchScript {
                len,
                bytes: bytes.clone(),
            }]);
            let ignored = rng.u32();
            let got = unsafe { fn_0066FC30::rw_0066fc30(fx.this(), ignored) };
            let mut tok = fx.lift();
            let mut fake = TokenFake::new();
            fake.fetch.push_back(TokenFetch {
                len,
                bytes: bytes.clone(),
            });
            let lift = tok.fetch_len(&mut fake);
            assert_eq!(got, lift, "len {len}");
            assert_eq!(
                rt::take_virtual(),
                vec![(
                    "fetch".to_string(),
                    vec![fx.this(), 0x40],
                    vec![bytes.clone()]
                )],
                "rewrite calls"
            );
            assert_eq!(fake.log, vec![TokenCall::Fetch(0x40)], "lift calls");
            let mut tok2 = fx.lift();
            let mut fake2 = TokenFake::new();
            fake2.fetch.push_back(TokenFetch { len, bytes });
            if wrong::len_plus_one(&mut tok2, &mut fake2) != lift {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong len never caught ({cases} cases)");
    }

    #[test]
    fn forward_then_int_float_match() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFC50);
        let mut caught_int = 0;
        let mut caught_float = 0;
        let mut cases = 0;
        for (len, bytes) in fetch_scripts(&mut rng) {
            for &answer in U32_EDGE {
                // vf13: the flagged integer tail.
                {
                    let fx = Fixture::build();
                    rt::set_script(&[]);
                    rt::set_virtual(&[("fwd_int", vec![answer])]);
                    rt::push_fetch(vec![FetchScript {
                        len,
                        bytes: bytes.clone(),
                    }]);
                    let got = unsafe { fn_0066FC50::rw_0066fc50(fx.this(), rng.u32()) };
                    let mut tok = fx.lift();
                    let mut fake = TokenFake::new();
                    fake.fetch.push_back(TokenFetch {
                        len,
                        bytes: bytes.clone(),
                    });
                    fake.flagged.push_back(answer);
                    let lift = tok.fetch_then_int(&mut fake);
                    assert_eq!(got, lift, "int len {len}");
                    assert_eq!(
                        rt::take_virtual(),
                        vec![
                            (
                                "fetch".to_string(),
                                vec![fx.this(), 0x40],
                                vec![bytes.clone()]
                            ),
                            ("fwd_int".to_string(), vec![fx.this(), 1], vec![]),
                        ],
                        "rewrite calls"
                    );
                    assert_eq!(
                        fake.log,
                        vec![TokenCall::Fetch(0x40), TokenCall::FlaggedInt(1)],
                        "lift calls"
                    );
                    let mut tok2 = fx.lift();
                    let mut fake2 = TokenFake::new();
                    fake2.flagged.push_back(answer);
                    let w = wrong::int_no_fetch(&mut tok2, &mut fake2);
                    if w != lift || fake2.log != fake.log {
                        caught_int += 1;
                    }
                    cases += 1;
                }
                // vf14: the float tail, bits in eax.
                {
                    let fans = support::quiet_snan(answer);
                    let fx = Fixture::build();
                    rt::set_script(&[]);
                    rt::set_virtual(&[("read_float", vec![fans])]);
                    rt::push_fetch(vec![FetchScript {
                        len,
                        bytes: bytes.clone(),
                    }]);
                    let got = unsafe { fn_0066FC80::rw_0066fc80(fx.this(), rng.u32()) };
                    let mut tok = fx.lift();
                    let mut fake = TokenFake::new();
                    fake.fetch.push_back(TokenFetch {
                        len,
                        bytes: bytes.clone(),
                    });
                    fake.floats.push_back(fans);
                    let lift = tok.fetch_then_float(&mut fake);
                    assert_eq!(got, lift.to_bits(), "float len {len}");
                    assert_eq!(
                        rt::take_virtual(),
                        vec![
                            (
                                "fetch".to_string(),
                                vec![fx.this(), 0x40],
                                vec![bytes.clone()]
                            ),
                            ("read_float".to_string(), vec![fx.this(), 1], vec![]),
                        ],
                        "rewrite calls"
                    );
                    assert_eq!(
                        fake.log,
                        vec![TokenCall::Fetch(0x40), TokenCall::ReadFloat(1)],
                        "lift calls"
                    );
                    let mut tok2 = fx.lift();
                    let mut fake2 = TokenFake::new();
                    fake2.floats.push_back(answer);
                    let w = wrong::float_no_fetch(&mut tok2, &mut fake2);
                    if w.to_bits() != lift.to_bits() || fake2.log != fake.log {
                        caught_float += 1;
                    }
                    cases += 1;
                }
            }
        }
        // A float-tail run over the float edge bits.
        for &bits in F32_EDGE {
            let bits = support::quiet_snan(bits);
            let fx = Fixture::build();
            rt::set_script(&[]);
            rt::set_virtual(&[("read_float", vec![bits])]);
            rt::push_fetch(vec![FetchScript {
                len: 2,
                bytes: vec![b'o', b'k'],
            }]);
            let got = unsafe { fn_0066FC80::rw_0066fc80(fx.this(), 0) };
            assert_eq!(got, bits, "float edge {bits:#x}");
            cases += 1;
        }
        assert!(
            caught_int > 0,
            "wrong int tail never caught ({cases} cases)"
        );
        assert!(
            caught_float > 0,
            "wrong float tail never caught ({cases} cases)"
        );
    }

    #[test]
    fn forward_then_value_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFCB0);
        let mut caught = 0;
        let mut cases = 0;
        // (method slot, rewrite): vf19 vf18 vf17 vf16 vf15.
        let methods: &[(ForwardSlot, u32)] = &[
            (ForwardSlot::V2c, 19),
            (ForwardSlot::V28, 18),
            (ForwardSlot::V24, 17),
            (ForwardSlot::V20, 16),
            (ForwardSlot::V1c, 15),
        ];
        for &(slot, which) in methods {
            for (len, bytes) in fetch_scripts(&mut rng) {
                let value = rng.u32();
                let answer = rng.u32();
                let fx = Fixture::build();
                rt::set_script(&[]);
                rt::set_virtual(&[(support::slot_name(slot), vec![answer])]);
                rt::push_fetch(vec![FetchScript {
                    len,
                    bytes: bytes.clone(),
                }]);
                let got = unsafe {
                    match which {
                        19 => fn_0066FCB0::rw_0066fcb0(fx.this(), rng.u32(), value),
                        18 => fn_0066FCE0::rw_0066fce0(fx.this(), rng.u32(), value),
                        17 => fn_0066FD10::rw_0066fd10(fx.this(), rng.u32(), value),
                        16 => fn_0066FD40::rw_0066fd40(fx.this(), rng.u32(), value),
                        _ => fn_0066FD70::rw_0066fd70(fx.this(), rng.u32(), value),
                    }
                };
                let mut tok = fx.lift();
                let mut fake = TokenFake::new();
                fake.fetch.push_back(TokenFetch {
                    len,
                    bytes: bytes.clone(),
                });
                fake.forward.push_back(answer);
                let lift = tok.fetch_then_value(&mut fake, slot, value);
                assert_eq!(got, lift, "vf{which} len {len}");
                assert_eq!(
                    rt::take_virtual(),
                    vec![
                        (
                            "fetch".to_string(),
                            vec![fx.this(), 0x40],
                            vec![bytes.clone()]
                        ),
                        (
                            support::slot_name(slot).to_string(),
                            vec![fx.this(), value, 1],
                            vec![],
                        ),
                    ],
                    "vf{which} rewrite calls"
                );
                assert_eq!(
                    fake.log,
                    vec![TokenCall::Fetch(0x40), TokenCall::Forward(slot, value)],
                    "lift calls"
                );
                let mut tok2 = fx.lift();
                let mut fake2 = TokenFake::new();
                fake2.fetch.push_back(TokenFetch { len, bytes });
                fake2.forward.push_back(answer);
                let w = wrong::value_plus_one(&mut tok2, &mut fake2, slot, value);
                if w != lift || fake2.log != fake.log {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(caught > 0, "wrong value never caught ({cases} cases)");
    }

    #[test]
    fn forward_check_int_float_match() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFDA0);
        let mut caught_int = 0;
        let mut caught_float = 0;
        let mut cases = 0;
        for (len, bytes) in fetch_scripts(&mut rng) {
            for expected in expected_strings() {
                for &answer in &[0u32, 1, 0xDEAD_BEEF] {
                    let exp_img = Image::of(&[expected.clone(), vec![0]].concat());
                    // vf20: the flagged integer tail.
                    {
                        let fx = Fixture::build();
                        rt::set_script(&[(2, StubKind::CdeclStrStr, vec![0x7777])]);
                        rt::set_virtual(&[("fwd_int", vec![answer])]);
                        rt::push_fetch(vec![FetchScript {
                            len,
                            bytes: bytes.clone(),
                        }]);
                        let got = unsafe { fn_0066FDA0::rw_0066fda0(fx.this(), exp_img.addr()) };
                        let mut tok = fx.lift();
                        let mut fake = TokenFake::new();
                        fake.fetch.push_back(TokenFetch {
                            len,
                            bytes: bytes.clone(),
                        });
                        fake.flagged.push_back(answer);
                        let lift = tok.fetch_check_then_int(&mut fake, &expected);
                        assert_eq!(got, lift, "int len {len}");
                        let numbered = rt::take_numbered();
                        if len != 0 {
                            assert_eq!(numbered.len(), 1, "compares iff token");
                            assert_eq!(
                                numbered[0].snaps,
                                vec![expected.clone(), bytes.clone()],
                                "compare inputs"
                            );
                        } else {
                            assert!(numbered.is_empty(), "no compare on empty");
                        }
                        assert_eq!(rt::take_virtual().len(), 2);
                        let mut expect_log = vec![TokenCall::Fetch(0x40)];
                        if len != 0 {
                            expect_log.push(TokenCall::Compare(expected.clone(), bytes.clone()));
                        }
                        expect_log.push(TokenCall::FlaggedInt(1));
                        assert_eq!(fake.log, expect_log, "lift calls");
                        let mut tok2 = fx.lift();
                        let mut fake2 = TokenFake::new();
                        fake2.fetch.push_back(TokenFetch {
                            len,
                            bytes: bytes.clone(),
                        });
                        fake2.flagged.push_back(answer);
                        let w = wrong::check_inverted(&mut tok2, &mut fake2, &expected);
                        if w != lift || fake2.log != fake.log {
                            caught_int += 1;
                        }
                        cases += 1;
                    }
                    // vf21: the float tail.
                    {
                        let fx = Fixture::build();
                        rt::set_script(&[(2, StubKind::CdeclStrStr, vec![0x7777])]);
                        rt::set_virtual(&[("read_float", vec![answer])]);
                        rt::push_fetch(vec![FetchScript {
                            len,
                            bytes: bytes.clone(),
                        }]);
                        let got = unsafe { fn_0066FDE0::rw_0066fde0(fx.this(), exp_img.addr()) };
                        let mut tok = fx.lift();
                        let mut fake = TokenFake::new();
                        fake.fetch.push_back(TokenFetch {
                            len,
                            bytes: bytes.clone(),
                        });
                        fake.floats.push_back(answer);
                        let lift = tok.fetch_check_then_float(&mut fake, &expected);
                        assert_eq!(got, lift.to_bits(), "float len {len}");
                        let numbered = rt::take_numbered();
                        assert_eq!(numbered.len(), usize::from(len != 0));
                        let mut expect_log = vec![TokenCall::Fetch(0x40)];
                        if len != 0 {
                            expect_log.push(TokenCall::Compare(expected.clone(), bytes.clone()));
                        }
                        expect_log.push(TokenCall::ReadFloat(1));
                        assert_eq!(fake.log, expect_log, "lift calls");
                        let mut tok2 = fx.lift();
                        let mut fake2 = TokenFake::new();
                        fake2.fetch.push_back(TokenFetch {
                            len,
                            bytes: bytes.clone(),
                        });
                        fake2.floats.push_back(answer);
                        let w = wrong::check_float_inverted(&mut tok2, &mut fake2, &expected);
                        if w.to_bits() != lift.to_bits() || fake2.log != fake.log {
                            caught_float += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert!(
            caught_int > 0,
            "wrong check-int never caught ({cases} cases)"
        );
        assert!(
            caught_float > 0,
            "wrong check-float never caught ({cases} cases)"
        );
    }

    #[test]
    fn forward_check_value_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFE20);
        let mut caught = 0;
        let mut cases = 0;
        // (method slot, rewrite): vf26 vf25 vf24 vf23 vf22.
        let methods: &[(ForwardSlot, u32)] = &[
            (ForwardSlot::V2c, 26),
            (ForwardSlot::V28, 25),
            (ForwardSlot::V24, 24),
            (ForwardSlot::V20, 23),
            (ForwardSlot::V1c, 22),
        ];
        for &(slot, which) in methods {
            for (len, bytes) in fetch_scripts(&mut rng) {
                let expected = vec![b'k', b'e', b'y'];
                let exp_img = Image::of(&[b'k', b'e', b'y', 0]);
                let value = rng.u32();
                let answer = rng.u32();
                let fx = Fixture::build();
                rt::set_script(&[(2, StubKind::CdeclStrStr, vec![0x7777])]);
                rt::set_virtual(&[(support::slot_name(slot), vec![answer])]);
                rt::push_fetch(vec![FetchScript {
                    len,
                    bytes: bytes.clone(),
                }]);
                let got = unsafe {
                    match which {
                        26 => fn_0066FE20::rw_0066fe20(fx.this(), exp_img.addr(), value),
                        25 => fn_0066FE70::rw_0066fe70(fx.this(), exp_img.addr(), value),
                        24 => fn_0066FEC0::rw_0066fec0(fx.this(), exp_img.addr(), value),
                        23 => fn_0066FF10::rw_0066ff10(fx.this(), exp_img.addr(), value),
                        _ => fn_0066FF60::rw_0066ff60(fx.this(), exp_img.addr(), value),
                    }
                };
                let mut tok = fx.lift();
                let mut fake = TokenFake::new();
                fake.fetch.push_back(TokenFetch {
                    len,
                    bytes: bytes.clone(),
                });
                fake.forward.push_back(answer);
                let lift = tok.fetch_check_then_value(&mut fake, &expected, slot, value);
                assert_eq!(got, lift, "vf{which} len {len}");
                let numbered = rt::take_numbered();
                assert_eq!(
                    numbered.len(),
                    usize::from(len != 0),
                    "vf{which} compare iff token"
                );
                if len != 0 {
                    assert_eq!(
                        numbered[0].snaps,
                        vec![expected.clone(), bytes.clone()],
                        "compare inputs"
                    );
                }
                assert_eq!(
                    rt::take_virtual(),
                    vec![
                        (
                            "fetch".to_string(),
                            vec![fx.this(), 0x40],
                            vec![bytes.clone()]
                        ),
                        (
                            support::slot_name(slot).to_string(),
                            vec![fx.this(), value, 1],
                            vec![],
                        ),
                    ],
                    "vf{which} rewrite calls"
                );
                let mut expect_log = vec![TokenCall::Fetch(0x40)];
                if len != 0 {
                    expect_log.push(TokenCall::Compare(expected.clone(), bytes.clone()));
                }
                expect_log.push(TokenCall::Forward(slot, value));
                assert_eq!(fake.log, expect_log, "lift calls");
                let mut tok2 = fx.lift();
                let mut fake2 = TokenFake::new();
                fake2.fetch.push_back(TokenFetch { len, bytes });
                fake2.forward.push_back(answer);
                let w = wrong::check_value_inverted(&mut tok2, &mut fake2, &expected, slot, value);
                if w != lift || fake2.log != fake.log {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(caught > 0, "wrong check-value never caught ({cases} cases)");
    }
}
