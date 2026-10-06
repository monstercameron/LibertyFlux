//! Differential cases: the tokenizer write path.
//!
//! Brace writers, line control, the label writer and the integer
//! formatter, each against its verified rewrite on the same generated
//! inputs, comparing results, every written byte, and every collaborator
//! call in order. Each case also runs a deliberately wrong lift, which
//! must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_files_memory::tokenizer::{TokenStream, Tokenizer};
    use lf_tokendiff::rewrites::*;
    use lf_tokendiff::rt::{self, ByteScript, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Image, Rng, TokenCall, TokenFake, U32_EDGE};

    const OBJ_VT: usize = 0x00;
    const OBJ_STREAM: usize = 0x0C;
    const OBJ_MODE: usize = 0x14;
    const OBJ_LEVEL: usize = 0x220;
    const OBJ_SIZE: usize = 0x224;

    const ST_BUF: usize = 0x08;
    const ST_POS: usize = 0x10;
    const ST_MID: usize = 0x14;
    const ST_END: usize = 0x18;
    const STREAM_SIZE: usize = 0x1C;

    /// A writer fixture: object, stream and stream bytes.
    struct Fixture {
        obj: Image,
        stream: Image,
        data: Image,
        vtable: support::VTable,
    }

    impl Fixture {
        fn build(data_len: usize, fill: u8) -> Self {
            let mut obj = Image::zeroed(OBJ_SIZE);
            let stream = Image::zeroed(STREAM_SIZE);
            let data = Image::of(&vec![fill; data_len]);
            let vtable = support::tokenizer_vtable();
            obj.w32(OBJ_VT, vtable.addr());
            let mut fx = Self {
                obj,
                stream,
                data,
                vtable,
            };
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

        fn lift(&self) -> Tokenizer {
            Tokenizer {
                first: 0,
                line: 1,
                stream: TokenStream {
                    data: self.data.buf.to_vec(),
                    pos: self.stream.r32(ST_POS),
                    mid: self.stream.r32(ST_MID),
                    end: self.stream.r32(ST_END),
                },
                limit: 0x20,
                mode: self.obj.r32(OBJ_MODE),
                pushback: Vec::new(),
                aux: 0,
                level: self.obj.r32(OBJ_LEVEL),
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_files_memory::tokenizer::{TokenWorld, Tokenizer};

        /// Opens a block without deepening the indent.
        pub fn open_no_deepen<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) {
            let level = t.level;
            w.write_indent(level);
            t.open_block(w);
            t.level = level;
        }

        /// Closes a block by deepening instead of shallowing.
        pub fn close_deepen<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) {
            t.level = t.level.wrapping_add(1);
            let level = t.level;
            w.write_indent(level);
            // The brace bytes still go out; only the level is wrong.
            let _ = level;
        }

        /// Indents only when a line is already open (inverted).
        pub fn start_inverted<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) {
            if t.mode == 1 {
                w.write_indent(t.level);
            }
            t.mode = 1;
        }

        /// Ends the line feed-first (swapped order).
        pub fn end_swapped<W: TokenWorld>(t: &mut Tokenizer, w: &mut W) {
            if t.mode != 2 {
                w.put_char(0x0A);
                w.put_char(0x0D);
            }
            t.mode = 2;
        }

        /// Writes the label without its tabs.
        pub fn label_no_tabs<W: TokenWorld>(
            t: &mut Tokenizer,
            w: &mut W,
            text: &[u8],
            count: u32,
        ) -> bool {
            t.mode = 0;
            let mut len = 0u32;
            while text[len as usize] != 0 {
                len = len.wrapping_add(1);
            }
            let wrote = w.write_text(&text[..len as usize]);
            wrote.wrapping_add(count) == len.wrapping_add(count)
        }

        /// Inverts the formatted-write success.
        pub fn int_inverted<W: TokenWorld>(t: &mut Tokenizer, w: &mut W, value: u32) -> bool {
            t.mode = 0;
            let bytes = w.format_value(value);
            let wrote = w.write_bytes(&bytes);
            wrote != bytes.len() as u32
        }
    }

    /// Stream shapes: (pos, mid, end) over an 8-byte buffer.
    fn stream_shapes() -> Vec<(u32, u32, u32)> {
        vec![
            (0, 0, 8),           // buffered with room
            (6, 0, 8),           // buffered, room runs out mid-write
            (8, 0, 8),           // buffered but full: slow path
            (0, 1, 8),           // bypass mode: slow path
            (0, 0xFFFF_FFFF, 8), // bypass, nonzero mode word
            (3, 0, 3),           // full at a nonzero cursor
        ]
    }

    /// Maps rewrite numbered calls to the expected fake log, checking
    /// addresses. Returns the mapped log.
    fn map_writer_calls(numbered: &[rt::NumberedCall], this: u32, stream: u32) -> Vec<TokenCall> {
        let mut log = Vec::new();
        for call in numbered {
            match call.id {
                1 => {
                    // indent(this, level), putc(stream, byte) or
                    // str_write(stream, len): told apart by the object
                    // argument and the snapshot.
                    if call.args[0] == this {
                        log.push(TokenCall::Indent(call.args[1]));
                    } else if call.snaps.is_empty() {
                        assert_eq!(call.args[0], stream, "putc stream");
                        log.push(TokenCall::PutChar(call.args[1] as u8));
                    } else {
                        assert_eq!(call.args[0], stream, "text stream");
                        log.push(TokenCall::WriteText(call.snaps[0].clone()));
                    }
                }
                2 => {
                    // slow byte / tab / block write.
                    assert_eq!(call.args[0], stream, "write stream");
                    if call.args[1] == 1 && call.snaps[0].len() == 1 {
                        log.push(TokenCall::ByteSlow(call.snaps[0][0]));
                    } else {
                        log.push(TokenCall::WriteBytes(call.snaps[0].clone()));
                    }
                }
                other => panic!("unexpected callee {other}"),
            }
        }
        log
    }

    #[test]
    fn write_blocks_match() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xFFB0);
        let mut caught_open = 0;
        let mut caught_close = 0;
        let mut cases = 0;
        for &(pos, mid, end) in &stream_shapes() {
            for &level in &[0u32, 1, 5, 0xFFFF_FFFF] {
                // vf27: open.
                {
                    let mut fx = Fixture::build(8, 0xAA);
                    fx.obj.w32(OBJ_LEVEL, level);
                    fx.stream.w32(ST_POS, pos);
                    fx.stream.w32(ST_MID, mid);
                    fx.stream.w32(ST_END, end);
                    rt::set_script(&[
                        (1, StubKind::Thiscall2, vec![0]),
                        (2, StubKind::ThiscallMem, vec![0, 0, 0]),
                    ]);
                    let pre = fx.lift();
                    unsafe { fn_0066FFB0::rw_0066ffb0(fx.this()) };
                    let numbered = rt::take_numbered();
                    let mut tok = pre.clone();
                    let mut fake = TokenFake::new();
                    tok.open_block(&mut fake);
                    assert_eq!(fx.obj.r32(OBJ_LEVEL), tok.level, "open level");
                    assert_eq!(fx.obj.r32(OBJ_LEVEL), level.wrapping_add(1));
                    assert_eq!(fx.stream.r32(ST_POS), tok.stream.pos, "open pos");
                    assert_eq!(fx.data.buf.to_vec(), tok.stream.data, "open bytes");
                    assert_eq!(
                        map_writer_calls(&numbered, fx.this(), fx.stream_addr()),
                        fake.log,
                        "open calls"
                    );
                    assert_eq!(fake.log[0], TokenCall::Indent(level), "indent first");
                    assert!(rt::take_virtual().is_empty());
                    let mut tok2 = pre.clone();
                    let mut fake2 = TokenFake::new();
                    wrong::open_no_deepen(&mut tok2, &mut fake2);
                    if tok2.level != tok.level || fake2.log != fake.log {
                        caught_open += 1;
                    }
                    cases += 1;
                }
                // vf28: close.
                {
                    let mut fx = Fixture::build(8, 0xAA);
                    fx.obj.w32(OBJ_LEVEL, level);
                    fx.stream.w32(ST_POS, pos);
                    fx.stream.w32(ST_MID, mid);
                    fx.stream.w32(ST_END, end);
                    rt::set_script(&[
                        (1, StubKind::Thiscall2, vec![0]),
                        (2, StubKind::ThiscallMem, vec![0, 0, 0]),
                    ]);
                    let pre = fx.lift();
                    unsafe { fn_00670060::rw_00670060(fx.this()) };
                    let numbered = rt::take_numbered();
                    let mut tok = pre.clone();
                    let mut fake = TokenFake::new();
                    tok.close_block(&mut fake);
                    assert_eq!(fx.obj.r32(OBJ_LEVEL), tok.level, "close level");
                    assert_eq!(fx.obj.r32(OBJ_LEVEL), level.wrapping_sub(1));
                    assert_eq!(fx.stream.r32(ST_POS), tok.stream.pos, "close pos");
                    assert_eq!(fx.data.buf.to_vec(), tok.stream.data, "close bytes");
                    assert_eq!(
                        map_writer_calls(&numbered, fx.this(), fx.stream_addr()),
                        fake.log,
                        "close calls"
                    );
                    assert_eq!(
                        fake.log[0],
                        TokenCall::Indent(level.wrapping_sub(1)),
                        "indent after shallow"
                    );
                    let mut tok2 = pre;
                    let mut fake2 = TokenFake::new();
                    wrong::close_deepen(&mut tok2, &mut fake2);
                    if tok2.level != tok.level {
                        caught_close += 1;
                    }
                    cases += 1;
                }
            }
        }
        // Random levels.
        for _ in 0..16 {
            let level = rng.u32();
            let mut fx = Fixture::build(8, 0xAA);
            fx.obj.w32(OBJ_LEVEL, level);
            fx.stream.w32(ST_POS, 0);
            fx.stream.w32(ST_MID, 0);
            fx.stream.w32(ST_END, 8);
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::ThiscallMem, vec![0, 0, 0]),
            ]);
            unsafe { fn_0066FFB0::rw_0066ffb0(fx.this()) };
            assert_eq!(fx.obj.r32(OBJ_LEVEL), level.wrapping_add(1));
            cases += 1;
        }
        assert!(caught_open > 0, "wrong open never caught ({cases} cases)");
        assert!(caught_close > 0, "wrong close never caught ({cases} cases)");
    }

    #[test]
    fn write_lines_match() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x7011);
        let mut caught_start = 0;
        let mut caught_end = 0;
        let mut cases = 0;
        for &mode in &[0u32, 1, 2, 3, 0xFFFF_FFFF] {
            for &level in &[0u32, 4, 0x8000_0000] {
                // vf29: start.
                {
                    let mut fx = Fixture::build(4, 0xAA);
                    fx.obj.w32(OBJ_MODE, mode);
                    fx.obj.w32(OBJ_LEVEL, level);
                    rt::set_script(&[(1, StubKind::Thiscall2, vec![0])]);
                    unsafe { fn_00670110::rw_00670110(fx.this()) };
                    let numbered = rt::take_numbered();
                    let mut tok = Tokenizer::new(0);
                    tok.mode = mode;
                    tok.level = level;
                    let mut fake = TokenFake::new();
                    tok.start_line(&mut fake);
                    assert_eq!(fx.obj.r32(OBJ_MODE), 1, "start state");
                    assert_eq!(tok.mode, 1);
                    let mut expect = Vec::new();
                    if mode != 1 {
                        assert_eq!(numbered.len(), 1, "indents iff closed");
                        assert_eq!(numbered[0].args, vec![fx.this(), level]);
                        expect.push(TokenCall::Indent(level));
                    } else {
                        assert!(numbered.is_empty(), "silent when open");
                    }
                    assert_eq!(fake.log, expect, "start calls");
                    let mut tok2 = Tokenizer::new(0);
                    tok2.mode = mode;
                    tok2.level = level;
                    let mut fake2 = TokenFake::new();
                    wrong::start_inverted(&mut tok2, &mut fake2);
                    if fake2.log != fake.log {
                        caught_start += 1;
                    }
                    cases += 1;
                }
                // vf30: end.
                {
                    let mut fx = Fixture::build(4, 0xAA);
                    fx.obj.w32(OBJ_MODE, mode);
                    rt::set_script(&[(1, StubKind::Thiscall2, vec![0, 0])]);
                    unsafe { fn_00670130::rw_00670130(fx.this()) };
                    let numbered = rt::take_numbered();
                    let mut tok = Tokenizer::new(0);
                    tok.mode = mode;
                    let mut fake = TokenFake::new();
                    tok.end_line(&mut fake);
                    assert_eq!(fx.obj.r32(OBJ_MODE), 2, "end state");
                    assert_eq!(tok.mode, 2);
                    let mut expect = Vec::new();
                    if mode != 2 {
                        assert_eq!(numbered.len(), 2, "two chars iff open");
                        assert_eq!(numbered[0].args, vec![fx.stream_addr(), 0x0D]);
                        assert_eq!(numbered[1].args, vec![fx.stream_addr(), 0x0A]);
                        expect.push(TokenCall::PutChar(0x0D));
                        expect.push(TokenCall::PutChar(0x0A));
                    } else {
                        assert!(numbered.is_empty(), "silent when ended");
                    }
                    assert_eq!(fake.log, expect, "end calls");
                    let mut tok2 = Tokenizer::new(0);
                    tok2.mode = mode;
                    let mut fake2 = TokenFake::new();
                    wrong::end_swapped(&mut tok2, &mut fake2);
                    if fake2.log != fake.log {
                        caught_end += 1;
                    }
                    cases += 1;
                }
            }
        }
        let _ = &mut rng;
        assert!(caught_start > 0, "wrong start never caught ({cases} cases)");
        assert!(caught_end > 0, "wrong end never caught ({cases} cases)");
    }

    #[test]
    fn write_label_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x701B);
        let mut caught = 0;
        let mut cases = 0;
        let texts: &[&[u8]] = &[b"a", b"hello", b"x y", b"0123456789abcdef"];
        for text in texts {
            for &count in &[0u32, 1, 3] {
                for &(pos, mid, end) in &stream_shapes() {
                    for &wrote in &[0u32, 1, 5, 0xFFFF_FFFF] {
                        let img = Image::of(&[text.to_vec(), vec![0]].concat());
                        let mut fx = Fixture::build(8, 0xAA);
                        fx.obj.w32(OBJ_MODE, rng.u32());
                        fx.stream.w32(ST_POS, pos);
                        fx.stream.w32(ST_MID, mid);
                        fx.stream.w32(ST_END, end);
                        rt::set_script(&[
                            (1, StubKind::ThiscallMem, vec![wrote]),
                            (2, StubKind::ThiscallMem, vec![0, 0, 0, 0]),
                        ]);
                        let pre = fx.lift();
                        let got = unsafe { fn_006701B0::rw_006701b0(fx.this(), img.addr(), count) };
                        let numbered = rt::take_numbered();
                        let mut full_text = text.to_vec();
                        full_text.push(0);
                        let mut tok = pre.clone();
                        let mut fake = TokenFake::new();
                        fake.written.push_back(wrote);
                        let lift = tok.write_label(&mut fake, &full_text, count);
                        assert_eq!(got, u32::from(lift), "label {text:?}");
                        assert_eq!(fx.obj.r32(OBJ_MODE), 0, "state cleared");
                        assert_eq!(tok.mode, 0);
                        assert_eq!(fx.stream.r32(ST_POS), tok.stream.pos, "pos");
                        assert_eq!(fx.data.buf.to_vec(), tok.stream.data, "bytes");
                        // Calls: one text write, then one slow call per
                        // tab that missed the buffer.
                        assert_eq!(numbered[0].id, 1);
                        assert_eq!(numbered[0].args, vec![fx.stream_addr(), text.len() as u32]);
                        assert_eq!(numbered[0].snaps, vec![text.to_vec()], "text bytes");
                        let mut expect_log = vec![TokenCall::WriteText(text.to_vec())];
                        // Tabs through the buffer are silent; slow tabs log.
                        let mut p = pos;
                        let mut slow_tabs = 0;
                        for _ in 0..count {
                            let slow = mid != 0 || (p as i32) >= (end as i32);
                            if slow {
                                expect_log.push(TokenCall::ByteSlow(9));
                                slow_tabs += 1;
                            } else {
                                p = p.wrapping_add(1);
                            }
                        }
                        assert_eq!(numbered.len(), 1 + slow_tabs, "tab calls");
                        for call in numbered.iter().skip(1) {
                            assert_eq!(call.id, 2);
                            assert_eq!(call.snaps, vec![vec![9u8]], "tab byte");
                        }
                        assert_eq!(fake.log, expect_log, "lift calls");
                        let mut tok2 = pre;
                        let mut fake2 = TokenFake::new();
                        fake2.written.push_back(wrote);
                        let w = wrong::label_no_tabs(&mut tok2, &mut fake2, &full_text, count);
                        if w != lift || fake2.log != fake.log || tok2.stream.data != tok.stream.data
                        {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert!(caught > 0, "wrong label never caught ({cases} cases)");
    }

    #[test]
    fn write_int_matches() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x7024);
        let mut caught = 0;
        let mut cases = 0;
        // (rewrite, formatted bytes pool).
        let formats: &[&[u8]] = &[b"0", b"42", b"-17", b"2147483647", b"", b"abc"];
        for (which, formatted) in [(39u32, formats), (40, formats)] {
            for value in U32_EDGE.iter().copied().take(6) {
                for bytes in formatted {
                    for &wrote in &[bytes.len() as u32, 0, 7] {
                        let mut fx = Fixture::build(4, 0xAA);
                        fx.obj.w32(OBJ_MODE, rng.u32());
                        rt::set_script(&[
                            (1, StubKind::CdeclFormat, Vec::new()),
                            (2, StubKind::ThiscallMem, vec![wrote]),
                        ]);
                        rt::set_byte_scripts(
                            1,
                            vec![ByteScript {
                                bytes: bytes.to_vec(),
                                ret: bytes.len() as u32,
                            }],
                        );
                        let got = unsafe {
                            if which == 39 {
                                fn_00670240::rw_00670240(fx.this(), value)
                            } else {
                                fn_00670290::rw_00670290(fx.this(), value)
                            }
                        };
                        let numbered = rt::take_numbered();
                        let mut tok = Tokenizer::new(0);
                        let mut fake = TokenFake::new();
                        fake.formatted.push_back(bytes.to_vec());
                        fake.blocks.push_back(wrote);
                        let lift = tok.write_int(&mut fake, value);
                        assert_eq!(got, u32::from(lift), "vf{which} value {value}");
                        assert_eq!(fx.obj.r32(OBJ_MODE), 0, "state cleared");
                        assert_eq!(tok.mode, 0);
                        // Calls: format(value) answering the bytes, then
                        // the block write of exactly those bytes.
                        assert_eq!(numbered.len(), 2, "two calls");
                        assert_eq!(numbered[0].id, 1);
                        assert_eq!(numbered[0].args, vec![value], "format value");
                        assert_eq!(numbered[0].snaps, vec![bytes.to_vec()], "formatted");
                        assert_eq!(numbered[1].id, 2);
                        assert_eq!(
                            numbered[1].args,
                            vec![fx.stream_addr(), bytes.len() as u32],
                            "write args"
                        );
                        assert_eq!(numbered[1].snaps, vec![bytes.to_vec()], "written");
                        assert_eq!(
                            fake.log,
                            vec![
                                TokenCall::Format(value),
                                TokenCall::WriteBytes(bytes.to_vec()),
                            ],
                            "lift calls"
                        );
                        let mut tok2 = Tokenizer::new(0);
                        let mut fake2 = TokenFake::new();
                        fake2.formatted.push_back(bytes.to_vec());
                        fake2.blocks.push_back(wrote);
                        if wrong::int_inverted(&mut tok2, &mut fake2, value) != lift {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        // Random values and byte shapes.
        for _ in 0..32 {
            let value = rng.u32();
            let n = rng.below(9) as usize;
            let bytes: Vec<u8> = (0..n).map(|_| b'0' + (rng.below(10) as u8)).collect();
            let wrote = if rng.u32() & 1 != 0 {
                n as u32
            } else {
                rng.u32()
            };
            let mut fx = Fixture::build(4, 0xAA);
            rt::set_script(&[
                (1, StubKind::CdeclFormat, Vec::new()),
                (2, StubKind::ThiscallMem, vec![wrote]),
            ]);
            rt::set_byte_scripts(
                1,
                vec![ByteScript {
                    bytes: bytes.clone(),
                    ret: bytes.len() as u32,
                }],
            );
            let got = unsafe { fn_00670240::rw_00670240(fx.this(), value) };
            let mut tok = Tokenizer::new(0);
            let mut fake = TokenFake::new();
            fake.formatted.push_back(bytes);
            fake.blocks.push_back(wrote);
            assert_eq!(got, u32::from(tok.write_int(&mut fake, value)), "random");
            cases += 1;
        }
        assert!(caught > 0, "wrong int never caught ({cases} cases)");
    }
}
