//! Host tests for the lifted tokenizer and streaming device: the edge
//! cases a reader of the code would ask about, run on the 64-bit host.

use lf_files_memory::tokenizer::{
    ForwardSlot, StreamDevice, StreamEntry, StreamTables, StreamWorld, TokenFetch, TokenStream,
    TokenWorld, Tokenizer, registry,
};
use std::collections::VecDeque;

// A scripted stream world.
struct StreamFake {
    resolve: VecDeque<Option<StreamEntry>>,
    shifted: VecDeque<u32>,
    data: VecDeque<u32>,
    post: VecDeque<u32>,
    log: Vec<String>,
}

impl StreamFake {
    fn new() -> Self {
        Self {
            resolve: VecDeque::new(),
            shifted: VecDeque::new(),
            data: VecDeque::new(),
            post: VecDeque::new(),
            log: Vec::new(),
        }
    }
}

impl StreamWorld for StreamFake {
    fn resolve(&mut self, key: u32) -> Option<StreamEntry> {
        self.log.push(format!("resolve {key:#x}"));
        self.resolve.pop_front().unwrap()
    }
    fn shifted(&mut self, _entry: &StreamEntry) -> u32 {
        self.log.push("shifted".to_string());
        self.shifted.pop_front().unwrap()
    }
    fn data_word(&mut self, _entry: &StreamEntry) -> u32 {
        self.log.push("data".to_string());
        self.data.pop_front().unwrap()
    }
    fn close_channel_object(
        &mut self,
        _object: Option<lf_core::Handle32<lf_files_memory::tokenizer::ChannelTag>>,
    ) {
        self.log.push("close".to_string());
    }
    fn post_span(
        &mut self,
        _object: Option<lf_core::Handle32<lf_files_memory::tokenizer::ChannelTag>>,
        _sink: u32,
        lo: u32,
        hi: u32,
        _a3: u32,
        _a4: u32,
    ) -> u32 {
        self.log.push(format!("post {lo:#x} {hi:#x}"));
        self.post.pop_front().unwrap()
    }
}

// A scripted token world.
struct TokenFake {
    refill: VecDeque<Option<u8>>,
    fetch: VecDeque<TokenFetch>,
    ints: VecDeque<u32>,
    doubles: VecDeque<f64>,
    floats: VecDeque<f32>,
    flagged: VecDeque<u32>,
    forward: VecDeque<u32>,
    written: VecDeque<u32>,
    formatted: VecDeque<Vec<u8>>,
    blocks: VecDeque<u32>,
    log: Vec<String>,
}

impl TokenFake {
    fn new() -> Self {
        Self {
            refill: VecDeque::new(),
            fetch: VecDeque::new(),
            ints: VecDeque::new(),
            doubles: VecDeque::new(),
            floats: VecDeque::new(),
            flagged: VecDeque::new(),
            forward: VecDeque::new(),
            written: VecDeque::new(),
            formatted: VecDeque::new(),
            blocks: VecDeque::new(),
            log: Vec::new(),
        }
    }
}

impl TokenWorld for TokenFake {
    fn refill(&mut self) -> Option<u8> {
        self.log.push("refill".to_string());
        self.refill.pop_front().unwrap()
    }
    fn skip_comment(&mut self) {
        self.log.push("skip".to_string());
    }
    fn fetch_token(&mut self, len: u32) -> TokenFetch {
        self.log.push(format!("fetch {len:#x}"));
        self.fetch.pop_front().unwrap()
    }
    fn parse_int(&mut self, text: &[u8]) -> u32 {
        self.log.push(format!("int {text:?}"));
        self.ints.pop_front().unwrap()
    }
    fn parse_float(&mut self, text: &[u8]) -> f64 {
        self.log.push(format!("float {text:?}"));
        self.doubles.pop_front().unwrap()
    }
    fn read_float(&mut self, flag: u32) -> f32 {
        self.log.push(format!("read {flag:#x}"));
        self.floats.pop_front().unwrap()
    }
    fn read_flagged_int(&mut self, flag: u32) -> u32 {
        self.log.push(format!("flagged {flag}"));
        self.flagged.pop_front().unwrap()
    }
    fn forward_value(&mut self, slot: ForwardSlot, value: u32) -> u32 {
        self.log.push(format!("forward {slot:?} {value:#x}"));
        self.forward.pop_front().unwrap()
    }
    fn compare_text(&mut self, _expected: &[u8], _token: &[u8]) -> u32 {
        self.log.push("compare".to_string());
        0
    }
    fn write_indent(&mut self, level: u32) {
        self.log.push(format!("indent {level}"));
    }
    fn write_byte_slow(&mut self, byte: u8) {
        self.log.push(format!("slow {byte:#x}"));
    }
    fn put_char(&mut self, byte: u8) {
        self.log.push(format!("put {byte:#x}"));
    }
    fn write_text(&mut self, text: &[u8]) -> u32 {
        self.log.push(format!("text {text:?}"));
        self.written.pop_front().unwrap()
    }
    fn format_value(&mut self, value: u32) -> Vec<u8> {
        self.log.push(format!("format {value}"));
        self.formatted.pop_front().unwrap()
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> u32 {
        self.log.push(format!("bytes {bytes:?}"));
        self.blocks.pop_front().unwrap()
    }
}

fn entry(offset: i32) -> StreamEntry {
    StreamEntry {
        offset,
        first: 0xA,
        kind: 0,
        size: 0,
        flags: 0,
    }
}

#[test]
fn registry_counts() {
    assert_eq!(registry::count(registry::State::Proven), 37);
    assert_eq!(registry::count(registry::State::Missing), 3);
    assert_eq!(registry::count(registry::State::Lifted), 0);
    assert_eq!(registry::ROWS.len(), 40);
}

#[test]
fn device_key_wraps() {
    let dev = StreamDevice {
        base_key: 0xFFFF_FFFF,
        elem_kinds: vec![],
        close_first: false,
    };
    let mut w = StreamFake::new();
    w.resolve.push_back(None);
    assert!(!dev.is_active(&mut w, 1));
    assert_eq!(w.log, vec!["resolve 0x0"]);
}

#[test]
fn device_active_bits() {
    let dev = StreamDevice {
        base_key: 0,
        elem_kinds: vec![],
        close_first: false,
    };
    // Low two size bits alone do not count.
    for (size, flags, expect) in [
        (3u32, 0u16, false),
        (4, 0, true),
        (0, 1 << 11, true),
        (0, 1 << 10, false),
        (0, 1 << 12, false),
    ] {
        let mut w = StreamFake::new();
        let mut e = entry(0);
        e.size = size;
        e.flags = flags;
        w.resolve.push_back(Some(e));
        assert_eq!(dev.is_active(&mut w, 0), expect, "size {size:#x} flags {flags:#x}");
    }
}

#[test]
fn device_span_slot_math() {
    let dev = StreamDevice {
        base_key: 0,
        elem_kinds: vec![],
        close_first: false,
    };
    let tables = StreamTables {
        spans: vec![],
        slots_v1: vec![],
        slots_v2: vec![0x1000],
        channels: vec![],
    };
    // Negative offsets divide toward zero, then truncate to 16 bits.
    for (offset, slot) in [(0i32, 0u32), (24, 1), (25, 1), (-24, 0xFFFF), (-1, 0)] {
        let mut w = StreamFake::new();
        w.resolve.push_back(Some(entry(offset)));
        w.data.push_back(0x1000);
        let span = dev.data_span(&mut w, &tables, 0).unwrap();
        assert_eq!(span.offset, 0, "offset {offset}");
        assert_eq!(span.slot, slot, "slot {offset}");
    }
    // The data difference wraps before the shift.
    let mut w = StreamFake::new();
    w.resolve.push_back(Some(entry(48)));
    w.data.push_back(0);
    let span = dev.data_span(&mut w, &tables, 0).unwrap();
    assert_eq!(span.offset, 0xFFFF_F000u32.wrapping_shl(11));
    assert_eq!(span.slot, 2);
}

#[test]
fn device_post_carry() {
    let dev = StreamDevice {
        base_key: 0,
        elem_kinds: vec![0],
        close_first: true,
    };
    let tables = StreamTables {
        spans: vec![],
        slots_v1: vec![],
        slots_v2: vec![],
        channels: vec![lf_files_memory::tokenizer::StreamChannel {
            cursor_lo: 0xFFFF_FFFF,
            cursor_hi: 0xFFFF_FFFF,
            object: None,
            sink_arg: 1,
        }],
    };
    let mut w = StreamFake::new();
    w.post.push_back(0x77);
    assert_eq!(dev.post_span_for(&mut w, &tables, 0, 1, 0, 0, 0), 0x77);
    assert_eq!(w.log, vec!["close", "post 0x0 0x0"]);
}

#[test]
fn device_post_no_close_first() {
    let dev = StreamDevice {
        base_key: 0,
        elem_kinds: vec![0],
        close_first: false,
    };
    let tables = StreamTables {
        spans: vec![],
        slots_v1: vec![],
        slots_v2: vec![],
        channels: vec![lf_files_memory::tokenizer::StreamChannel {
            cursor_lo: 5,
            cursor_hi: 6,
            object: None,
            sink_arg: 7,
        }],
    };
    let mut w = StreamFake::new();
    w.post.push_back(9);
    assert_eq!(dev.post_span_for(&mut w, &tables, 0, 10, 20, 30, 40), 9);
    assert_eq!(w.log, vec!["post 0xf 0x1a"]);
}

#[test]
#[should_panic(expected = "unresolvable")]
fn device_span_pair_null_panics() {
    let dev = StreamDevice {
        base_key: 0,
        elem_kinds: vec![],
        close_first: false,
    };
    let mut w = StreamFake::new();
    w.resolve.push_back(None);
    let _ = dev.span_pair(&mut w, &StreamTables::default(), 0);
}

#[test]
fn tokenizer_new_matches_ctor() {
    let tok = Tokenizer::new(0x1234);
    assert_eq!(tok.first, 0x1234);
    assert_eq!(tok.line, 1);
    assert_eq!(tok.limit, 0x20);
    assert_eq!(tok.mode, 2);
    assert!(tok.pushback.is_empty());
    assert_eq!(tok.aux, 0);
    assert_eq!(tok.level, 0);
}

#[test]
fn token_read_edges() {
    // Size 0 and 1: terminator only, nothing consumed.
    for size in [0u32, 1] {
        let mut tok = Tokenizer {
            stream: TokenStream {
                data: vec![b'a', b','],
                pos: 0,
                mid: 2,
                end: 2,
            },
            pushback: vec![b'z'],
            ..Tokenizer::new(0)
        };
        let mut w = TokenFake::new();
        let out = tok.read_token(&mut w, size, b',', true);
        assert_eq!(out.len, 0, "size {size}");
        assert_eq!(out.bytes, vec![0]);
        assert!(w.log.is_empty(), "no calls for size {size}");
        assert_eq!(tok.stream.pos, 0);
        assert_eq!(tok.pushback, vec![b'z']);
    }
    // A lone delimiter: empty token.
    let mut tok = Tokenizer {
        stream: TokenStream {
            data: vec![b','],
            pos: 0,
            mid: 1,
            end: 1,
        },
        ..Tokenizer::new(0)
    };
    let out = tok.read_token(&mut TokenFake::new(), 32, b',', true);
    assert_eq!(out.len, 0);
    assert_eq!(out.bytes, vec![0]);
    // Failed with nothing stored: -1, nothing written.
    let mut tok = Tokenizer::new(0);
    let mut w = TokenFake::new();
    w.refill.push_back(None);
    let out = tok.read_token(&mut w, 32, b',', true);
    assert_eq!(out.len, -1);
    assert!(out.bytes.is_empty());
    // Failed after storing: partial token with terminator.
    let mut tok = Tokenizer {
        stream: TokenStream {
            data: vec![b'a', b'b'],
            pos: 0,
            mid: 2,
            end: 2,
        },
        ..Tokenizer::new(0)
    };
    let mut w = TokenFake::new();
    w.refill.push_back(None);
    let out = tok.read_token(&mut w, 32, b',', true);
    assert_eq!(out.len, 2);
    assert_eq!(out.bytes, vec![b'a', b'b', 0]);
}

#[test]
fn token_read_trims_all_five() {
    for ws in [0x20u8, 9, 0x0A, 0x0D, 0] {
        let mut tok = Tokenizer {
            stream: TokenStream {
                data: vec![b'a', ws, b','],
                pos: 0,
                mid: 3,
                end: 3,
            },
            ..Tokenizer::new(0)
        };
        let out = tok.read_token(&mut TokenFake::new(), 32, b',', true);
        assert_eq!(out.bytes, vec![b'a', 0], "trailing {ws:#x}");
        // A high byte is not whitespace.
        let mut tok = Tokenizer {
            stream: TokenStream {
                data: vec![b'a', 0x80, b','],
                pos: 0,
                mid: 3,
                end: 3,
            },
            ..Tokenizer::new(0)
        };
        let out = tok.read_token(&mut TokenFake::new(), 32, b',', true);
        assert_eq!(out.bytes, vec![b'a', 0x80, 0], "high byte kept");
    }
}

#[test]
fn token_read_pushback_lifo_and_line() {
    let mut tok = Tokenizer {
        stream: TokenStream {
            data: vec![b'\n', b','],
            pos: 0,
            mid: 2,
            end: 2,
        },
        pushback: vec![b'1', b'2'],
        line: 10,
        ..Tokenizer::new(0)
    };
    let out = tok.read_token(&mut TokenFake::new(), 32, b',', true);
    assert_eq!(out.bytes, vec![b'2', b'1', 0]);
    assert_eq!(tok.line, 11, "one newline counted");
    assert!(tok.pushback.is_empty());
}

#[test]
fn token_read_comment_runs_skipper() {
    let mut tok = Tokenizer {
        stream: TokenStream {
            data: vec![b';', b';', b','],
            pos: 0,
            mid: 3,
            end: 3,
        },
        ..Tokenizer::new(0)
    };
    let mut w = TokenFake::new();
    let out = tok.read_token(&mut w, 32, b',', true);
    assert_eq!(out.bytes, vec![b';', b';', 0], "semicolons are stored");
    assert_eq!(w.log, vec!["skip", "skip"]);
}

#[test]
#[should_panic]
fn token_read_past_buffer_panics() {
    let mut tok = Tokenizer {
        stream: TokenStream {
            data: vec![b'a'],
            pos: 5,
            mid: 6,
            end: 6,
        },
        ..Tokenizer::new(0)
    };
    let _ = tok.read_token(&mut TokenFake::new(), 32, b',', true);
}

#[test]
fn scalar_classification() {
    // (len, first byte, expect parse for int, for double).
    let cases: &[(u32, u8, bool, bool)] = &[
        (0, 0, false, false),
        (0, b'5', true, true),
        (0, b'-', false, false),
        (1, b'-', true, true),
        (1, b'.', false, true),
        (1, b'7', true, true),
        (1, b'a', false, false),
        (1, b'+', false, false),
        (1, 0xFF, false, false),
    ];
    for &(len, first, want_int, want_double) in cases {
        let bytes = if len == 0 && first == 0 { vec![] } else { vec![first] };
        let mut tok = Tokenizer::new(0);
        let mut w = TokenFake::new();
        w.fetch.push_back(TokenFetch {
            len,
            bytes: bytes.clone(),
        });
        w.ints.push_back(0x5555);
        assert_eq!(
            tok.read_int(&mut w, true),
            if want_int { 0x5555 } else { 0 },
            "int len {len} byte {first:#x}"
        );
        let mut tok = Tokenizer::new(0);
        let mut w = TokenFake::new();
        w.fetch.push_back(TokenFetch { len, bytes });
        w.doubles.push_back(1.5);
        assert_eq!(
            tok.read_double(&mut w, true),
            if want_double { 1.5 } else { 0.0 },
            "double len {len} byte {first:#x}"
        );
    }
    // Optional fallback is -1.
    let mut tok = Tokenizer::new(0);
    let mut w = TokenFake::new();
    w.fetch.push_back(TokenFetch {
        len: 1,
        bytes: vec![b'x'],
    });
    assert_eq!(tok.read_int(&mut w, false), 0xFFFF_FFFF);
}

#[test]
fn fetch_check_branch() {
    // No compare on an empty fetch.
    let mut tok = Tokenizer::new(0);
    let mut w = TokenFake::new();
    w.fetch.push_back(TokenFetch {
        len: 0,
        bytes: vec![],
    });
    w.flagged.push_back(3);
    assert_eq!(tok.fetch_check_then_int(&mut w, b"key"), 3);
    assert_eq!(w.log, vec!["fetch 0x40", "flagged 1"]);
    // Compare on a token.
    let mut tok = Tokenizer::new(0);
    let mut w = TokenFake::new();
    w.fetch.push_back(TokenFetch {
        len: 2,
        bytes: vec![b'o', b'k'],
    });
    w.forward.push_back(4);
    assert_eq!(
        tok.fetch_check_then_value(&mut w, b"key", ForwardSlot::V24, 9),
        4
    );
    assert_eq!(w.log[1], "compare");
}

#[test]
fn writer_state_machines() {
    // start_line indents unless open.
    for (mode, indents) in [(0u32, true), (1, false), (2, true), (99, true)] {
        let mut tok = Tokenizer::new(0);
        tok.mode = mode;
        tok.level = 3;
        let mut w = TokenFake::new();
        tok.start_line(&mut w);
        assert_eq!(tok.mode, 1);
        assert_eq!(w.log.len(), usize::from(indents), "mode {mode}");
    }
    // end_line writes CR LF unless ended.
    for (mode, writes) in [(0u32, true), (1, true), (2, false), (99, true)] {
        let mut tok = Tokenizer::new(0);
        tok.mode = mode;
        let mut w = TokenFake::new();
        tok.end_line(&mut w);
        assert_eq!(tok.mode, 2);
        assert_eq!(
            w.log,
            if writes { vec!["put 0xd".to_string(), "put 0xa".to_string()] } else { vec![] },
            "mode {mode}"
        );
    }
    // Blocks wrap the level.
    let mut tok = Tokenizer::new(0);
    tok.level = 0xFFFF_FFFF;
    tok.stream = TokenStream {
        data: vec![0; 8],
        pos: 0,
        mid: 0,
        end: 8,
    };
    tok.open_block(&mut TokenFake::new());
    assert_eq!(tok.level, 0);
    assert_eq!(&tok.stream.data[..3], &[b'{', 0x0D, 0x0A]);
    tok.close_block(&mut TokenFake::new());
    assert_eq!(tok.level, 0xFFFF_FFFF);
    assert_eq!(&tok.stream.data[3..6], &[b'}', 0x0D, 0x0A]);
}

#[test]
fn writer_put_rule() {
    // Buffered writes land in the buffer; the boundary goes slow.
    let mut tok = Tokenizer::new(0);
    tok.stream = TokenStream {
        data: vec![0xAA; 4],
        pos: 3,
        mid: 0,
        end: 4,
    };
    let mut w = TokenFake::new();
    tok.open_block(&mut w); // '{' buffered, CR LF slow
    assert_eq!(tok.stream.data, vec![0xAA, 0xAA, 0xAA, b'{']);
    assert_eq!(tok.stream.pos, 4);
    assert_eq!(w.log[1], "slow 0xd");
    assert_eq!(w.log[2], "slow 0xa");
    // A nonzero mode bypasses the buffer entirely.
    let mut tok = Tokenizer::new(0);
    tok.stream = TokenStream {
        data: vec![0xAA; 4],
        pos: 0,
        mid: 2,
        end: 4,
    };
    let mut w = TokenFake::new();
    tok.open_block(&mut w);
    assert_eq!(tok.stream.data, vec![0xAA; 4]);
    assert_eq!(tok.stream.pos, 0);
}

#[test]
fn label_success() {
    // wrote + count equals len + count: success (the count shifts
    // both sides equally, so wrapping adds no further case).
    let mut tok = Tokenizer::new(0);
    tok.stream = TokenStream {
        data: vec![],
        pos: 0,
        mid: 1,
        end: 0,
    };
    let mut w = TokenFake::new();
    w.written.push_back(1);
    assert!(tok.write_label(&mut w, b"a\0", 3));
    assert_eq!(w.log[1], "slow 0x9");
    assert_eq!(w.log.len(), 4, "one text write plus three tabs");
    // A short write fails.
    let mut tok = Tokenizer::new(0);
    tok.stream = TokenStream {
        data: vec![],
        pos: 0,
        mid: 1,
        end: 0,
    };
    let mut w = TokenFake::new();
    w.written.push_back(0);
    assert!(!tok.write_label(&mut w, b"a\0", 0));
}

#[test]
fn int_format_success() {
    let mut tok = Tokenizer::new(0);
    let mut w = TokenFake::new();
    w.formatted.push_back(vec![b'1', b'2']);
    w.blocks.push_back(2);
    assert!(tok.write_int(&mut w, 12));
    let mut w = TokenFake::new();
    w.formatted.push_back(vec![b'1', b'2']);
    w.blocks.push_back(1);
    assert!(!tok.write_int(&mut w, 12));
}
