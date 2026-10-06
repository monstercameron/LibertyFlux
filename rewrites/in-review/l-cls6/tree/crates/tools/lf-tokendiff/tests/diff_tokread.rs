//! Differential cases: the tokenizer read path.
//!
//! The delimiter reader, the scalar readers and the float-vector readers,
//! each against its verified rewrite on the same generated inputs,
//! comparing results, every written byte, and every collaborator call in
//! order. Each case also runs a deliberately wrong lift, which must be
//! caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_files_memory::tokenizer::{TokenFetch, TokenStream, Tokenizer};
    use lf_tokendiff::rewrites::*;
    use lf_tokendiff::rt::{self, ByteScript, FetchScript, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        F32_EDGE, F64_EDGE, Image, Rng, TokenCall, TokenFake, U32_EDGE, VTable,
    };

    // Tokenizer object field offsets, as the verified rewrites use them.
    const OBJ_VT: usize = 0x00;
    const OBJ_FIRST: usize = 0x04;
    const OBJ_LINE: usize = 0x08;
    const OBJ_STREAM: usize = 0x0C;
    const OBJ_LIMIT: usize = 0x10;
    const OBJ_MODE: usize = 0x14;
    const OBJ_PBCOUNT: usize = 0x18;
    const OBJ_PBCELLS: usize = 0x1C;
    const OBJ_AUX: usize = 0x21C;
    const OBJ_LEVEL: usize = 0x220;
    const OBJ_SIZE: usize = 0x224;

    // Stream object field offsets.
    const ST_BUF: usize = 0x08;
    const ST_POS: usize = 0x10;
    const ST_MID: usize = 0x14;
    const ST_END: usize = 0x18;
    const STREAM_SIZE: usize = 0x1C;

    /// A tokenizer fixture: object, stream, stream bytes and vtable.
    struct Fixture {
        obj: Image,
        stream: Image,
        data: Image,
        vtable: VTable,
    }

    impl Fixture {
        fn build(data_bytes: &[u8]) -> Self {
            let obj = Image::zeroed(OBJ_SIZE);
            let stream = Image::zeroed(STREAM_SIZE);
            let data = Image::of(data_bytes);
            let vtable = support::tokenizer_vtable();
            let mut fx = Self {
                obj,
                stream,
                data,
                vtable,
            };
            fx.obj.w32(OBJ_VT, fx.vtable.addr());
            fx.obj.w32(OBJ_STREAM, fx.stream.addr());
            fx.stream.w32(ST_BUF, fx.data.addr());
            fx
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        fn stream_addr(&self) -> u32 {
            self.stream.addr()
        }

        /// The lifted tokenizer owning the same words the images hold.
        fn lift(&self) -> Tokenizer {
            let count = self.obj.r32(OBJ_PBCOUNT) as usize;
            Tokenizer {
                first: self.obj.r32(OBJ_FIRST),
                line: self.obj.r32(OBJ_LINE),
                stream: TokenStream {
                    data: self.data.buf.to_vec(),
                    pos: self.stream.r32(ST_POS),
                    mid: self.stream.r32(ST_MID),
                    end: self.stream.r32(ST_END),
                },
                limit: self.obj.r32(OBJ_LIMIT),
                mode: self.obj.r32(OBJ_MODE),
                pushback: self.obj.buf[OBJ_PBCELLS..OBJ_PBCELLS + count].to_vec(),
                aux: self.obj.r32(OBJ_AUX),
                level: self.obj.r32(OBJ_LEVEL),
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_files_memory::tokenizer::{TokenRead, TokenWorld, Tokenizer};

        /// Inverts the whitespace-skipping flag: a plausible
        /// flag-bit mistake, wrong whenever leading whitespace is
        /// present.
        pub fn skip_inverted<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            size: u32,
            delim: u8,
            skip: bool,
        ) -> TokenRead {
            t.read_token(w, size, delim, !skip)
        }

        /// Parses any token starting with `-`, even when the fetch
        /// reports no token.
        pub fn int_minus_any_len<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            required: bool,
        ) -> u32 {
            let fetch = w.fetch_token(0x20);
            let first = fetch.bytes.first().copied().unwrap_or(0);
            let _ = t;
            if first == b'-' || first.wrapping_sub(b'0') <= 9 {
                w.parse_int(&fetch.bytes)
            } else if required {
                0
            } else {
                0xFFFF_FFFF
            }
        }

        /// Never parses a leading `.`.
        pub fn double_no_dot<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) -> f64 {
            let fetch = w.fetch_token(0x20);
            let first = fetch.bytes.first().copied().unwrap_or(0);
            let _ = t;
            if (fetch.len != 0 && first == b'-') || first.wrapping_sub(b'0') <= 9 {
                w.parse_float(&fetch.bytes)
            } else {
                0.0
            }
        }

        /// Stores the floats rotated by one.
        pub fn floats_rotated<W: TokenWorld, const N: usize>(
            t: &mut Tokenizer,
            w: &mut W,
            flag: u32,
        ) -> [f32; N] {
            let got: [f32; N] = t.read_floats(w, flag);
            core::array::from_fn(|i| got[(i + 1) % N])
        }
    }

    /// One delimiter-read case.
    struct ReadCase {
        pushback: Vec<u8>,
        stream: Vec<u8>,
        pos: u32,
        end: u32,
        refill: Vec<(u8, u32)>,
        size: u32,
        delim: u8,
        skip: bool,
        line: u32,
    }

    fn read_shapes() -> Vec<ReadCase> {
        let mut v = Vec::new();
        // Empty stream, failing refill: failed with nothing stored.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![],
            pos: 0,
            end: 0,
            refill: vec![(0, 0)],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Delimiter immediately.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b',', b'x'],
            pos: 0,
            end: 2,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Leading whitespace skipped, trailing trimmed.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b' ', b'\t', b'a', b'b', b' ', b'\r', b','],
            pos: 0,
            end: 7,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Whitespace kept when skipping is off.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b' ', b'a', b' ', b','],
            pos: 0,
            end: 4,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: false,
            line: 1,
        });
        // Newlines count lines even while skipped or trimming.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b'\n', b'a', b'\n', b','],
            pos: 0,
            end: 4,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 41,
        });
        // A newline delimiter still counts its line.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b'a', b'\n', b'b'],
            pos: 0,
            end: 3,
            refill: vec![],
            size: 32,
            delim: b'\n',
            skip: true,
            line: 7,
        });
        // Semicolons run the skipper and are stored.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b'a', b';', b'b', b','],
            pos: 0,
            end: 4,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Pushback drains last-in-first-out before the stream.
        v.push(ReadCase {
            pushback: vec![b'x', b'y'],
            stream: vec![b','],
            pos: 0,
            end: 1,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Pushback -1 (byte 0xFF) fails the read after storing.
        v.push(ReadCase {
            pushback: vec![0xFF, b'q'],
            stream: vec![b','],
            pos: 0,
            end: 1,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Pushback -1 with nothing stored: -1, nothing written.
        v.push(ReadCase {
            pushback: vec![0xFF],
            stream: vec![b','],
            pos: 0,
            end: 1,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // A high-byte delimiter matches a sign-extended pushback byte.
        v.push(ReadCase {
            pushback: vec![0x80],
            stream: vec![b','],
            pos: 0,
            end: 1,
            refill: vec![],
            size: 32,
            delim: 0x80,
            skip: true,
            line: 1,
        });
        // Refill bytes join the token; a failing refill ends it.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![],
            pos: 0,
            end: 0,
            refill: vec![(b'h', 1), (b'i', 1), (0, 0)],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // A refill answer of 2 (not 1) fails the read.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![],
            pos: 0,
            end: 0,
            refill: vec![(b'h', 2)],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Sizes below 2 store nothing but the terminator.
        for size in [0u32, 1] {
            v.push(ReadCase {
                pushback: vec![b'a'],
                stream: vec![b'b', b','],
                pos: 0,
                end: 2,
                refill: vec![],
                size,
                delim: b',',
                skip: true,
                line: 1,
            });
        }
        // Truncation at size - 1, with the terminator.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b'a', b'b', b'c', b'd', b'e', b','],
            pos: 0,
            end: 6,
            refill: vec![],
            size: 4,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Huge sizes: the sign-extended limit/reserve logic.
        for size in [0x7FFF_FFFFu32, 0x8000_0000, 0xFFFF_FFFF] {
            v.push(ReadCase {
                pushback: vec![],
                stream: vec![b'a', b','],
                pos: 0,
                end: 2,
                refill: vec![],
                size,
                delim: b',',
                skip: true,
                line: 1,
            });
        }
        // NUL bytes: skipped leading, trimming trailing.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![0, b'a', 0, b','],
            pos: 0,
            end: 4,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        // Stream position starting mid-buffer.
        v.push(ReadCase {
            pushback: vec![],
            stream: vec![b'z', b'z', b'k', b','],
            pos: 2,
            end: 4,
            refill: vec![],
            size: 32,
            delim: b',',
            skip: true,
            line: 1,
        });
        v
    }

    #[test]
    fn token_read_until_delimiter_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x6F7A);
        let mut caught = 0;
        let mut cases = 0;
        let mut run = |case: ReadCase, catch: &mut u32| {
            let mut fx = Fixture::build(&case.stream);
            fx.obj.w32(OBJ_LINE, case.line);
            fx.obj.w32(OBJ_PBCOUNT, case.pushback.len() as u32);
            for (i, byte) in case.pushback.iter().enumerate() {
                fx.obj.buf[OBJ_PBCELLS + i] = *byte;
            }
            fx.stream.w32(ST_POS, case.pos);
            fx.stream.w32(ST_MID, case.end);
            // The caller buffer: the rewrite writes at most `size`
            // bytes for small sizes, one byte for huge ones.
            let alloc = if case.size <= 0x1000 {
                (case.size as usize).max(1) + 8
            } else {
                8
            };
            let mut buf = Image::zeroed(alloc);
            let scripts: Vec<ByteScript> = case
                .refill
                .iter()
                .map(|(b, ret)| ByteScript {
                    bytes: vec![*b],
                    ret: *ret,
                })
                .collect();
            rt::set_script(&[
                (1, StubKind::Refill, Vec::new()),
                (2, StubKind::Thiscall1, vec![0; 64]),
            ]);
            rt::set_byte_scripts(1, scripts.clone());
            // Snapshot the lift state before the rewrite mutates the images.
            let pre = fx.lift();
            let got = unsafe {
                fn_0066F7A0::rw_0066f7a0(fx.this(), buf.addr(), case.size, u32::from(case.delim), u32::from(case.skip))
            };
            let numbered = rt::take_numbered();
            let mut tok = pre.clone();
            let mut fake = TokenFake::new();
            for (b, ret) in &case.refill {
                fake.refill.push_back(if *ret == 1 { Some(*b) } else { None });
            }
            let lift = tok.read_token(&mut fake, case.size, case.delim, case.skip);
            assert_eq!(got, lift.len as u32, "len (stream {:?})", case.stream);
            if lift.len < 0 {
                assert_eq!(lift.bytes, Vec::<u8>::new(), "no bytes on -1");
            } else {
                let n = lift.len as usize + 1;
                assert_eq!(buf.buf[..n], lift.bytes[..], "written bytes");
                assert_eq!(lift.bytes.last(), Some(&0), "terminator");
            }
            // Effects: line, stream cursor, pushback count.
            assert_eq!(fx.obj.r32(OBJ_LINE), tok.line, "line");
            assert_eq!(fx.stream.r32(ST_POS), tok.stream.pos, "pos");
            assert_eq!(fx.obj.r32(OBJ_PBCOUNT), tok.pushback.len() as u32, "pushback");
            // Calls in order: refills and comment skips.
            let mut expect_log = Vec::new();
            let mut refill_idx = 0;
            for call in &numbered {
                match call.id {
                    1 => {
                        assert_eq!(call.args, vec![fx.stream_addr(), 1], "refill args");
                        assert_eq!(call.snaps, vec![vec![scripts[refill_idx].bytes[0]]], "refill byte");
                        refill_idx += 1;
                        expect_log.push(TokenCall::Refill);
                    }
                    2 => {
                        assert_eq!(call.args, vec![fx.this()], "skip args");
                        expect_log.push(TokenCall::SkipComment);
                    }
                    other => panic!("unexpected callee {other}"),
                }
            }
            assert_eq!(fake.log, expect_log, "lift calls");
            assert!(rt::take_virtual().is_empty(), "no virtual calls");
            // Wrong lift: the inverted flag stores leading
            // whitespace it should drop (or drops what it should
            // store). Pad with failing refills: the wrong path may
            // read further.
            let mut fake2 = TokenFake::new();
            for (b, ret) in case.refill.iter().map(|(b, r)| (*b, *r)).chain(
                core::iter::repeat((0u8, 0u32)).take(8),
            ) {
                fake2.refill.push_back(if ret == 1 { Some(b) } else { None });
            }
            let mut tok2 = pre.clone();
            if wrong::skip_inverted(&mut tok2, &mut fake2, case.size, case.delim, case.skip) != lift
            {
                *catch += 1;
            }
            cases += 1;
        };
        for case in read_shapes() {
            run(case, &mut caught);
        }
        // Random small cases over a byte alphabet with early delimiters.
        let alphabet: &[u8] = &[b'a', b' ', b'\t', b'\n', b'\r', 0, b';', b',', 0xFF, 0x80];
        for _ in 0..48 {
            let n = 1 + rng.below(6) as usize;
            let mut stream: Vec<u8> = (0..n).map(|_| alphabet[rng.below(10) as usize]).collect();
            if !stream.contains(&b',') && rng.u32() & 1 != 0 {
                stream.push(b',');
            }
            let end = stream.len() as u32;
            let pb_len = rng.below(4) as usize;
            let pushback: Vec<u8> = (0..pb_len).map(|_| alphabet[rng.below(10) as usize]).collect();
            let n_refill = rng.below(3) as usize;
            let mut refill = Vec::new();
            for _ in 0..n_refill {
                refill.push((alphabet[rng.below(10) as usize], 1));
            }
            refill.push((0, 0));
            run(
                ReadCase {
                    pushback,
                    stream,
                    pos: 0,
                    end,
                    refill,
                    size: 2 + rng.below(10),
                    delim: b',',
                    skip: rng.u32() & 1 != 0,
                    line: 1 + rng.below(5),
                },
                &mut caught,
            );
        }
        assert!(caught > 0, "wrong skip flag never caught ({cases} cases)");
    }

    /// Token scripts for the scalar readers: (reported len, bytes).
    /// Bytes carry no interior NUL, matching the registry narrowing.
    fn scalar_tokens() -> Vec<(u32, Vec<u8>)> {
        let mut v: Vec<(u32, Vec<u8>)> = vec![
            (0, vec![]),
            (0, vec![b'5']),
            (0, vec![b'-']),
            (0, vec![b'a']),
            (1, vec![b'-']),
            (2, vec![b'-', b'3']),
            (3, vec![b'-', b'1', b'2']),
            (1, vec![b'0']),
            (2, vec![b'9', b'9']),
            (3, vec![b'a', b'b', b'c']),
            (1, vec![b'.']),
            (3, vec![b'.', b'5', b'x']),
            (1, vec![b' ']),
            (4, vec![b'+', b'1', b'2', b'3']),
            (2, vec![0xFF, b'1']),
        ];
        // A 31-byte token: fills the scalar buffer without its NUL.
        v.push((31, vec![b'7'; 31]));
        v
    }

    #[test]
    fn token_read_int_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x6F9D);
        let mut caught = 0;
        let mut cases = 0;
        for (len, bytes) in scalar_tokens() {
            for &required in &[true, false] {
                for &answer in &[0u32, 1, 0xFFFF_FFFF, 0x8000_0000] {
                    let fx = Fixture::build(&[]);
                    rt::set_script(&[(2, StubKind::CdeclStr, vec![answer])]);
                    rt::push_fetch(vec![FetchScript {
                        len,
                        bytes: bytes.clone(),
                    }]);
                    let got = unsafe { fn_0066F9D0::rw_0066f9d0(fx.this(), u32::from(required)) };
                    let mut tok = fx.lift();
                    let mut fake = TokenFake::new();
                    fake.fetch.push_back(TokenFetch {
                        len,
                        bytes: bytes.clone(),
                    });
                    fake.ints.push_back(answer);
                    let lift = tok.read_int(&mut fake, required);
                    assert_eq!(got, lift, "token {bytes:?} required {required}");
                    // Calls: one fetch with the scalar length, then the
                    // parser iff the classifier parsed.
                    let virtual_calls = rt::take_virtual();
                    assert_eq!(virtual_calls.len(), 1, "one fetch");
                    assert_eq!(virtual_calls[0].0, "fetch");
                    assert_eq!(virtual_calls[0].1, vec![fx.this(), 0x20]);
                    assert_eq!(virtual_calls[0].2, vec![bytes.clone()]);
                    let numbered = rt::take_numbered();
                    let parsed = !numbered.is_empty();
                    if parsed {
                        assert_eq!(numbered.len(), 1);
                        assert_eq!(numbered[0].id, 2);
                        assert_eq!(numbered[0].snaps, vec![bytes.clone()], "parser input");
                    }
                    let mut expect_log = vec![TokenCall::Fetch(0x20)];
                    if parsed {
                        expect_log.push(TokenCall::ParseInt(bytes.clone()));
                    }
                    assert_eq!(fake.log, expect_log, "lift calls");
                    // Wrong lift.
                    let mut tok2 = fx.lift();
                    let mut fake2 = TokenFake::new();
                    fake2.fetch.push_back(TokenFetch {
                        len,
                        bytes: bytes.clone(),
                    });
                    fake2.ints.push_back(answer);
                    if wrong::int_minus_any_len(&mut tok2, &mut fake2, required) != lift {
                        caught += 1;
                    }
                    cases += 1;
                }
                let _ = &mut rng;
            }
        }
        assert!(caught > 0, "wrong int never caught ({cases} cases)");
    }

    #[test]
    fn token_read_double_matches() {
        let _guard = rt::script_lock();
        let mut caught = 0;
        let mut cases = 0;
        for (len, bytes) in scalar_tokens() {
            for &bits in F64_EDGE {
                let fx = Fixture::build(&[]);
                rt::set_script(&[(2, StubKind::CdeclF64Str, Vec::new())]);
                rt::set_answers64(2, vec![bits]);
                rt::push_fetch(vec![FetchScript {
                    len,
                    bytes: bytes.clone(),
                }]);
                let got = unsafe { fn_0066FA20::rw_0066fa20(fx.this(), 1) };
                let mut tok = fx.lift();
                let mut fake = TokenFake::new();
                fake.fetch.push_back(TokenFetch {
                    len,
                    bytes: bytes.clone(),
                });
                fake.doubles.push_back(bits);
                let lift = tok.read_double(&mut fake, true);
                assert_eq!(got.to_bits(), lift.to_bits(), "token {bytes:?} bits {bits:#x}");
                let virtual_calls = rt::take_virtual();
                assert_eq!(virtual_calls.len(), 1, "one fetch");
                assert_eq!(virtual_calls[0].1, vec![fx.this(), 0x20]);
                let numbered = rt::take_numbered();
                let parsed = !numbered.is_empty();
                if parsed {
                    assert_eq!(numbered[0].snaps, vec![bytes.clone()], "parser input");
                } else {
                    assert_eq!(lift, 0.0, "fallback is zero");
                }
                let mut expect_log = vec![TokenCall::Fetch(0x20)];
                if parsed {
                    expect_log.push(TokenCall::ParseFloat(bytes.clone()));
                }
                assert_eq!(fake.log, expect_log, "lift calls");
                let mut tok2 = fx.lift();
                let mut fake2 = TokenFake::new();
                fake2.fetch.push_back(TokenFetch {
                    len,
                    bytes: bytes.clone(),
                });
                fake2.doubles.push_back(bits);
                if wrong::double_no_dot(&mut tok2, &mut fake2).to_bits() != lift.to_bits() {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(caught > 0, "wrong double never caught ({cases} cases)");
    }

    #[test]
    fn token_read_floats_match() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x6FA8);
        let mut caught = 0;
        let mut cases = 0;
        // (rewrite id, count): the five vector readers.
        for &(which, n) in [(11u32, 2usize), (10, 3), (9, 4), (8, 3), (7, 4)] {
            for trial in 0..40 {
                let flag = if trial < U32_EDGE.len() {
                    U32_EDGE[trial]
                } else {
                    rng.u32()
                };
                let mut words = Vec::with_capacity(n);
                for i in 0..n {
                    words.push(if trial < F32_EDGE.len() {
                        F32_EDGE[(trial + i) % F32_EDGE.len()]
                    } else {
                        rng.u32()
                    });
                }
                // Distinct values so rotation is observable.
                words[0] ^= 0x1111_1111u32.wrapping_add(trial as u32);
                let fx = Fixture::build(&[]);
                rt::set_script(&[]);
                rt::set_virtual(&[("read_float", words.clone())]);
                let mut out = Image::zeroed(16);
                let got = unsafe {
                    match which {
                        11 => fn_0066FA80::rw_0066fa80(fx.this(), out.addr(), flag),
                        10 => fn_0066FAB0::rw_0066fab0(fx.this(), out.addr(), flag),
                        9 => fn_0066FAF0::rw_0066faf0(fx.this(), out.addr(), flag),
                        8 => fn_0066FB40::rw_0066fb40(fx.this(), out.addr(), flag),
                        _ => fn_0066FBA0::rw_0066fba0(fx.this(), out.addr(), flag),
                    }
                };
                let mut tok = fx.lift();
                let mut fake = TokenFake::new();
                fake.floats.extend(words.iter().copied());
                let lift: Vec<u32> = match n {
                    2 => tok.read_floats::<TokenFake, 2>(&mut fake, flag).iter().map(f32::to_bits).collect(),
                    3 => tok.read_floats::<TokenFake, 3>(&mut fake, flag).iter().map(f32::to_bits).collect(),
                    _ => tok.read_floats::<TokenFake, 4>(&mut fake, flag).iter().map(f32::to_bits).collect(),
                };
                for (i, word) in lift.iter().enumerate() {
                    assert_eq!(out.r32(i * 4), *word, "vf{which} lane {i}");
                    assert_eq!(*word, words[i], "vf{which} scripted lane {i}");
                }
                if which == 8 || which == 7 {
                    assert_eq!(got, out.addr(), "vf{which} answers out");
                } else {
                    assert_eq!(got, words[n - 1], "vf{which} answers last bits");
                }
                let virtual_calls = rt::take_virtual();
                assert_eq!(virtual_calls.len(), n, "vf{which} reads");
                for call in &virtual_calls {
                    assert_eq!(call.0, "read_float");
                    assert_eq!(call.1, vec![fx.this(), flag]);
                }
                assert_eq!(fake.log.len(), n);
                assert!(rt::take_numbered().is_empty());
                // Wrong lift: rotation must differ.
                let mut tok2 = fx.lift();
                let mut fake2 = TokenFake::new();
                fake2.floats.extend(words.iter().copied());
                let rotated: Vec<u32> = match n {
                    2 => wrong::floats_rotated::<TokenFake, 2>(&mut tok2, &mut fake2, flag).iter().map(f32::to_bits).collect(),
                    3 => wrong::floats_rotated::<TokenFake, 3>(&mut tok2, &mut fake2, flag).iter().map(f32::to_bits).collect(),
                    _ => wrong::floats_rotated::<TokenFake, 4>(&mut tok2, &mut fake2, flag).iter().map(f32::to_bits).collect(),
                };
                if rotated != lift {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(caught > 0, "wrong floats never caught ({cases} cases)");
    }
}
