//! The text tokenizer: reading and writing the game's text data files.
//!
//! Lifted from the verified rewrites of `rage::fiTokenizer`. The 32-bit
//! object reads a stream token by token: a pushback stack of already-seen
//! characters, a buffered stream with a refill role, a line counter, a
//! read/write state word and an indent level for the writer side. Here
//! those are plain fields; the stream object the original keeps beside
//! the tokenizer is owned inline, and every callee and virtual slot the
//! rewrites reach (the token fetcher, the number parsers, the float
//! reader, the forwarded slots, the stream reader and writers) is one
//! method of [`TokenWorld`].

/// A tokenizer stream: the buffered bytes with a cursor and two limit
/// words. Readers compare the cursor against `mid` (the buffered end);
/// writers test `mid` as a mode word (nonzero bypasses the buffer) and
/// compare the cursor against `end` (the buffer capacity).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenStream {
    /// The buffered bytes.
    pub data: Vec<u8>,
    /// The cursor: next byte to read or write.
    pub pos: u32,
    /// The read end (readers) or the bypass mode (writers).
    pub mid: u32,
    /// The write capacity (writers).
    pub end: u32,
}

/// A tokenizer, owning its stream, its pushback stack and its counters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tokenizer {
    /// The caller-supplied first word, kept as stored.
    pub first: u32,
    /// The current line number, counting newlines read.
    pub line: u32,
    /// The owned stream.
    pub stream: TokenStream,
    /// The read limit word, as the constructor leaves it.
    pub limit: u32,
    /// The read/write state word.
    pub mode: u32,
    /// The pushback stack, popped last-in-first-out.
    pub pushback: Vec<u8>,
    /// The auxiliary word, as the constructor leaves it.
    pub aux: u32,
    /// The writer indent level.
    pub level: u32,
}

/// The value-forward virtual slots: five slots taking (`value`, 1) whose
/// meanings the inventory does not establish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForwardSlot {
    /// The first value slot.
    V1c,
    /// The second value slot.
    V20,
    /// The third value slot.
    V24,
    /// The fourth value slot.
    V28,
    /// The fifth value slot.
    V2c,
}

/// One fetched token: the fetch's reported length and the bytes it left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenFetch {
    /// The length the fetch reported.
    pub len: u32,
    /// The token bytes the fetch left.
    pub bytes: Vec<u8>,
}

/// One delimiter read: the stored length (or -1 for a failed read that
/// stored nothing) and exactly the bytes written, terminator included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRead {
    /// The stored length, or -1 when the read failed before storing.
    pub len: i32,
    /// Exactly the bytes written (empty when `len` is -1).
    pub bytes: Vec<u8>,
}

/// What the tokenizer needs from the engine around it: the stream refill
/// and comment roles, the token fetcher, the number parsers, the float
/// reader, the forwarded slots, the string comparer, and the writer
/// roles (indent, slow byte, character, text, formatter, block writer).
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with caller buffers narrowed to returned values (the fetcher
/// answers the token, the formatter answers the formatted bytes) and
/// frame-cell addresses dropped.
pub trait TokenWorld {
    /// Refills one stream byte; `None` when the stream is exhausted.
    fn refill(&mut self) -> Option<u8>;
    /// Skips the comment opened by a semicolon.
    fn skip_comment(&mut self);
    /// Fetches one token into a buffer of `len` bytes.
    fn fetch_token(&mut self, len: u32) -> TokenFetch;
    /// Parses a token as an integer; answers the parser's answer.
    fn parse_int(&mut self, text: &[u8]) -> u32;
    /// Parses a token as a double; answers the parser's answer.
    fn parse_float(&mut self, text: &[u8]) -> f64;
    /// Reads one float with a flag, as the x87 return delivers it
    /// (a signalling NaN would arrive quietened; scripts exclude them).
    fn read_float(&mut self, flag: u32) -> f32;
    /// The flagged integer slot; answers its answer.
    fn read_flagged_int(&mut self, flag: u32) -> u32;
    /// One value-forward slot with (`value`, 1); answers its answer.
    fn forward_value(&mut self, slot: ForwardSlot, value: u32) -> u32;
    /// Compares a fetched token against an expected string; the result is
    /// ignored by every caller, but the call is observable.
    fn compare_text(&mut self, expected: &[u8], token: &[u8]) -> u32;
    /// Writes the indent for a level.
    fn write_indent(&mut self, level: u32);
    /// Writes one byte past the buffer (the slow path).
    fn write_byte_slow(&mut self, byte: u8);
    /// Writes one character through the stream.
    fn put_char(&mut self, byte: u8);
    /// Writes a text span; answers how much was written.
    fn write_text(&mut self, text: &[u8]) -> u32;
    /// Formats an integer; answers the formatted bytes.
    fn format_value(&mut self, value: u32) -> Vec<u8>;
    /// Writes a byte span through the stream; answers how much was written.
    fn write_bytes(&mut self, bytes: &[u8]) -> u32;
}

/// The newline character.
const NEWLINE: i32 = 0x0A;
/// The comment character.
const COMMENT: i32 = 0x3B;
/// The end-of-input marker, reachable from the pushback only.
const END_OF_INPUT: i32 = -1;
/// The token buffer length the scalar readers fetch with.
const SCALAR_TOKEN_LEN: u32 = 0x20;
/// The token buffer length the fetch-and-forward readers fetch with.
const FORWARD_TOKEN_LEN: u32 = 0x40;
/// The flag the fetch-and-forward readers forward with.
const FORWARD_FLAG: u32 = 1;
/// The line number a fresh tokenizer starts on.
const FIRST_LINE: u32 = 1;
/// The read limit a fresh tokenizer starts with.
const LIMIT_INIT: u32 = 0x20;
/// The read mode a fresh tokenizer starts in.
const MODE_INIT: u32 = 2;
/// The writer state meaning "a line is open".
const LINE_OPEN: u32 = 1;
/// The writer state meaning "the line is ended".
const LINE_ENDED: u32 = 2;
/// The tab character.
const TAB: u8 = 9;
/// The carriage-return character.
const CR: u8 = 0x0D;
/// The line-feed character.
const LF: u8 = 0x0A;

/// True for the five characters the reader skips and trims: space, tab,
/// newline, carriage return and NUL.
fn is_space_char(ch: i32) -> bool {
    matches!(ch, 0x20 | 9 | NEWLINE | 0x0D | 0)
}

impl Tokenizer {
    /// A fresh tokenizer over an empty stream, as the constructor leaves
    /// it: line 1, read limit `0x20`, read mode 2, empty pushback, cleared
    /// auxiliary word and indent level.
    #[must_use]
    pub fn new(first: u32) -> Self {
        Self {
            first,
            line: FIRST_LINE,
            limit: LIMIT_INIT,
            mode: MODE_INIT,
            ..Self::default()
        }
    }

    /// Reads characters until the delimiter byte, skipping leading
    /// whitespace, trimming trailing whitespace, and terminating with
    /// NUL: at most `size - 1` characters are stored, so sizes below 2
    /// store nothing but the terminator.
    ///
    /// Each character comes from the pushback stack first (sign-extended),
    /// then from the stream buffer (zero-extended), then from the refill
    /// role. A newline counts a line; a semicolon runs the comment role;
    /// end-of-input fails the read. The answer is the stored length, or
    /// -1 when the read failed before anything was stored (nothing is
    /// written then).
    ///
    /// # Panics
    ///
    /// When the stream cursor selects past the buffered bytes.
    pub fn read_token<W: TokenWorld>(
        &mut self,
        world: &mut W,
        size: u32,
        delimiter: u8,
        skip_ws: bool,
    ) -> TokenRead {
        let limit = (size as i32).wrapping_sub(1);
        let mut stored: Vec<u8> = Vec::new();
        let mut skipping = true;
        let mut failed = false;
        if limit > 0 {
            let delimiter = delimiter as i8 as i32;
            loop {
                let ch: i32;
                if let Some(byte) = self.pushback.pop() {
                    ch = i32::from(byte as i8);
                } else if (self.stream.pos as i32) < (self.stream.mid as i32) {
                    ch = i32::from(self.stream.data[self.stream.pos as usize]);
                    self.stream.pos = self.stream.pos.wrapping_add(1);
                } else {
                    match world.refill() {
                        Some(byte) => ch = i32::from(byte),
                        None => {
                            failed = true;
                            break;
                        }
                    }
                }
                if ch == NEWLINE {
                    self.line = self.line.wrapping_add(1);
                } else if ch == COMMENT {
                    world.skip_comment();
                } else if ch == END_OF_INPUT {
                    failed = true;
                    break;
                }
                if ch == delimiter {
                    break;
                }
                let drop = skip_ws && skipping && is_space_char(ch);
                if !drop {
                    stored.push(ch as u8);
                    skipping = false;
                }
                if !((stored.len() as i32) < limit) {
                    break;
                }
            }
        }
        if failed && stored.is_empty() {
            return TokenRead {
                len: -1,
                bytes: Vec::new(),
            };
        }
        while let Some(&byte) = stored.last() {
            if is_space_char(i32::from(byte as i8)) {
                stored.pop();
            } else {
                break;
            }
        }
        let len = stored.len() as i32;
        stored.push(0);
        TokenRead { len, bytes: stored }
    }

    /// The first token byte, or zero when the fetch left nothing (the
    /// original reads a zeroed buffer then).
    fn first_byte(fetch: &TokenFetch) -> u8 {
        fetch.bytes.first().copied().unwrap_or(0)
    }

    /// Reads the next token as an integer: tokens starting with `-` (when
    /// the fetch reports one) or with a digit are parsed; otherwise the
    /// fallback is 0 when required and -1 when not.
    pub fn read_int<W: TokenWorld>(&mut self, world: &mut W, required: bool) -> u32 {
        let fetch = world.fetch_token(SCALAR_TOKEN_LEN);
        let first = Self::first_byte(&fetch);
        let mut parse = false;
        if fetch.len != 0 && first == b'-' {
            parse = true;
        } else if first.wrapping_sub(b'0') <= 9 {
            parse = true;
        }
        if parse {
            world.parse_int(&fetch.bytes)
        } else if required {
            0
        } else {
            0xFFFF_FFFF
        }
    }

    /// Reads the next token as a double: tokens starting with `-` (when
    /// the fetch reports one), `.` or a digit are parsed; otherwise the
    /// fallback is 0.0.
    pub fn read_double<W: TokenWorld>(&mut self, world: &mut W, required: bool) -> f64 {
        let fetch = world.fetch_token(SCALAR_TOKEN_LEN);
        let first = Self::first_byte(&fetch);
        let mut parse = false;
        if fetch.len != 0 && first == b'-' {
            parse = true;
        } else if first == b'.' {
            parse = true;
        } else if first.wrapping_sub(b'0') <= 9 {
            parse = true;
        }
        if parse {
            world.parse_float(&fetch.bytes)
        } else {
            debug_assert!(required, "optional double: unproven fallback");
            0.0
        }
    }

    /// Reads `N` floats with a flag, in order.
    pub fn read_floats<W: TokenWorld, const N: usize>(
        &mut self,
        world: &mut W,
        flag: u32,
    ) -> [f32; N] {
        core::array::from_fn(|_| world.read_float(flag))
    }

    /// Fetches one token and answers the fetch's reported length.
    pub fn fetch_len<W: TokenWorld>(&mut self, world: &mut W) -> u32 {
        world.fetch_token(FORWARD_TOKEN_LEN).len
    }

    /// Fetches one token, discards it, then answers the flagged integer
    /// slot with flag 1.
    pub fn fetch_then_int<W: TokenWorld>(&mut self, world: &mut W) -> u32 {
        world.fetch_token(FORWARD_TOKEN_LEN);
        world.read_flagged_int(FORWARD_FLAG)
    }

    /// Fetches one token, discards it, then answers the float reader with
    /// flag 1.
    pub fn fetch_then_float<W: TokenWorld>(&mut self, world: &mut W) -> f32 {
        world.fetch_token(FORWARD_TOKEN_LEN);
        world.read_float(FORWARD_FLAG)
    }

    /// Fetches one token, discards it, then answers a value-forward slot
    /// with (`value`, 1).
    pub fn fetch_then_value<W: TokenWorld>(
        &mut self,
        world: &mut W,
        slot: ForwardSlot,
        value: u32,
    ) -> u32 {
        world.fetch_token(FORWARD_TOKEN_LEN);
        world.forward_value(slot, value)
    }

    /// Fetches one token, compares it against the expected string when the
    /// fetch reports one, then answers the flagged integer slot with
    /// flag 1.
    pub fn fetch_check_then_int<W: TokenWorld>(
        &mut self,
        world: &mut W,
        expected: &[u8],
    ) -> u32 {
        let fetch = world.fetch_token(FORWARD_TOKEN_LEN);
        if fetch.len != 0 {
            world.compare_text(expected, &fetch.bytes);
        }
        world.read_flagged_int(FORWARD_FLAG)
    }

    /// Fetches one token, compares it against the expected string when the
    /// fetch reports one, then answers the float reader with flag 1.
    pub fn fetch_check_then_float<W: TokenWorld>(
        &mut self,
        world: &mut W,
        expected: &[u8],
    ) -> f32 {
        let fetch = world.fetch_token(FORWARD_TOKEN_LEN);
        if fetch.len != 0 {
            world.compare_text(expected, &fetch.bytes);
        }
        world.read_float(FORWARD_FLAG)
    }

    /// Fetches one token, compares it against the expected string when the
    /// fetch reports one, then answers a value-forward slot with
    /// (`value`, 1).
    pub fn fetch_check_then_value<W: TokenWorld>(
        &mut self,
        world: &mut W,
        expected: &[u8],
        slot: ForwardSlot,
        value: u32,
    ) -> u32 {
        let fetch = world.fetch_token(FORWARD_TOKEN_LEN);
        if fetch.len != 0 {
            world.compare_text(expected, &fetch.bytes);
        }
        world.forward_value(slot, value)
    }

    /// Puts one byte: into the stream buffer when the mode word is clear
    /// and the cursor is inside the capacity, else through the slow role.
    fn put_byte<W: TokenWorld>(&mut self, world: &mut W, byte: u8) {
        if self.stream.mid != 0 {
            world.write_byte_slow(byte);
            return;
        }
        if self.stream.pos as i32 >= self.stream.end as i32 {
            world.write_byte_slow(byte);
            return;
        }
        let pos = self.stream.pos as usize;
        self.stream.data[pos] = byte;
        self.stream.pos = self.stream.pos.wrapping_add(1);
    }

    /// Writes an indented opening-brace line and deepens the indent.
    pub fn open_block<W: TokenWorld>(&mut self, world: &mut W) {
        let level = self.level;
        world.write_indent(level);
        self.put_byte(world, b'{');
        self.put_byte(world, CR);
        self.put_byte(world, LF);
        self.level = level.wrapping_add(1);
    }

    /// Shallows the indent, then writes an indented closing-brace line.
    pub fn close_block<W: TokenWorld>(&mut self, world: &mut W) {
        let level = self.level.wrapping_sub(1);
        self.level = level;
        world.write_indent(level);
        self.put_byte(world, b'}');
        self.put_byte(world, CR);
        self.put_byte(world, LF);
    }

    /// Starts a fresh indented line unless one is already open; the state
    /// becomes open either way.
    pub fn start_line<W: TokenWorld>(&mut self, world: &mut W) {
        if self.mode != LINE_OPEN {
            world.write_indent(self.level);
        }
        self.mode = LINE_OPEN;
    }

    /// Ends the current line unless it is already ended; the state becomes
    /// ended either way.
    pub fn end_line<W: TokenWorld>(&mut self, world: &mut W) {
        if self.mode != LINE_ENDED {
            world.put_char(CR);
            world.put_char(LF);
        }
        self.mode = LINE_ENDED;
    }

    /// Writes a string followed by `count` tabs, reporting full success:
    /// true when what the text role wrote plus `count` equals the string
    /// length plus `count`.
    pub fn write_label<W: TokenWorld>(&mut self, world: &mut W, text: &[u8], count: u32) -> bool {
        self.mode = 0;
        debug_assert!(
            !text.is_empty() && text[0] != 0,
            "empty label: unproven default"
        );
        let mut len = 0u32;
        while text[len as usize] != 0 {
            len = len.wrapping_add(1);
        }
        let wrote = world.write_text(&text[..len as usize]);
        let expect = len.wrapping_add(count);
        let got = wrote.wrapping_add(count);
        let mut left = count;
        while left != 0 {
            left = left.wrapping_sub(1);
            self.put_byte(world, TAB);
        }
        got == expect
    }

    /// Formats an integer into the stream, reporting full success: true
    /// when what the block role wrote equals the formatted length.
    pub fn write_int<W: TokenWorld>(&mut self, world: &mut W, value: u32) -> bool {
        self.mode = 0;
        let bytes = world.format_value(value);
        let wrote = world.write_bytes(&bytes);
        wrote == bytes.len() as u32
    }
}
