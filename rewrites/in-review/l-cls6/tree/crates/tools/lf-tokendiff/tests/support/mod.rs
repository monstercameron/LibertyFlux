//! Shared differential-test support: images, tables, stubs and fakes.
//!
//! 32-bit target only. The vtable stubs record through the runtime's
//! virtual log; the fakes implement the lifted traits from the same
//! scripts, so each test compares the two call logs.

// Shared across test binaries: each binary uses a subset.
#![allow(dead_code)]

use lf_core::Handle32;
use lf_files_memory::tokenizer::{
    ForwardSlot, StreamEntry, StreamWorld, TokenFetch, TokenWorld,
};
use lf_tokendiff::rt;

/// Wraps a raw address as an opaque cookie (`None` for null).
#[must_use]
pub fn cookie<T: ?Sized>(addr: u32) -> Option<Handle32<T>> {
    Handle32::new(addr)
}

/// The raw word of an optional cookie (zero for `None`).
#[must_use]
pub fn raw<T: ?Sized>(handle: Option<Handle32<T>>) -> u32 {
    Handle32::raw_or_zero(handle)
}

/// A test-owned byte block with a stable address.
pub struct Image {
    /// The bytes (boxed so the address never moves).
    pub buf: Box<[u8]>,
}

impl Image {
    /// A zeroed block.
    #[must_use]
    pub fn zeroed(len: usize) -> Self {
        Self {
            buf: vec![0u8; len].into_boxed_slice(),
        }
    }

    /// A block of scripted bytes.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        Self {
            buf: bytes.to_vec().into_boxed_slice(),
        }
    }

    /// The block's address.
    #[must_use]
    pub fn addr(&self) -> u32 {
        self.buf.as_ptr() as usize as u32
    }

    /// Length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    #[must_use]
    pub fn r32(&self, off: usize) -> u32 {
        u32::from_le_bytes(self.buf[off..off + 4].try_into().unwrap())
    }

    #[must_use]
    pub fn r16(&self, off: usize) -> u16 {
        u16::from_le_bytes(self.buf[off..off + 2].try_into().unwrap())
    }

    #[must_use]
    pub fn r8(&self, off: usize) -> u8 {
        self.buf[off]
    }

    pub fn w32(&mut self, off: usize, v: u32) {
        self.buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    pub fn w16(&mut self, off: usize, v: u16) {
        self.buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }

    pub fn w8(&mut self, off: usize, v: u8) {
        self.buf[off] = v;
    }
}

/// A planted virtual table: words addressed by byte offset.
pub struct VTable {
    img: Image,
}

impl VTable {
    /// A zeroed table of `words` entries.
    #[must_use]
    pub fn zeroed(words: usize) -> Self {
        Self {
            img: Image::zeroed(words * 4),
        }
    }

    /// The table's address.
    #[must_use]
    pub fn addr(&self) -> u32 {
        self.img.addr()
    }

    /// Plants a stub address at a byte offset.
    pub fn set(&mut self, off: usize, addr: u32) {
        self.img.w32(off, addr);
    }
}

/// A tiny seeded generator (xorshift: good enough for scripts).
pub struct Rng(pub u64);

impl Rng {
    #[must_use]
    pub fn u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as u32
    }

    #[must_use]
    pub fn below(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        self.u32() % n
    }
}

/// Edge words shared by every case.
pub const U32_EDGE: &[u32] = &[
    0,
    1,
    2,
    0x7F,
    0x80,
    0xFF,
    0x100,
    0x7FFF,
    0x8000,
    0xFFFF,
    0x1_0000,
    0x7FFF_FFFF,
    0x8000_0000,
    0xFFFF_FFFE,
    0xFFFF_FFFF,
];

/// Edge float bits: zeros, ones, infinities, NaNs, subnormals.
pub const F32_EDGE: &[u32] = &[
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_0000,
    0xFFC0_0000,
    0x7F80_0001,
    0x0000_0001,
    0x8000_0001,
    0x7F7F_FFFF,
    0x0080_0000,
    0x3F00_0000,
    0xC000_0000,
    0x42C8_0000,
];

/// Quiets a signalling NaN the way the x87 float return does: the
/// float reader's answers reach the method through that return, so a
/// scripted signalling NaN would arrive quietened on the rewrite side
/// but unquietened on the lift side. Scripts pass through here so both
/// sides receive the same bits.
#[must_use]
pub fn quiet_snan(bits: u32) -> u32 {
    if bits & 0x7F80_0000 == 0x7F80_0000 && bits & 0x007F_FFFF != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// Edge double bits.
pub const F64_EDGE: &[u64] = &[
    0x0000_0000_0000_0000,
    0x8000_0000_0000_0000,
    0x3FF0_0000_0000_0000,
    0xBFF0_0000_0000_0000,
    0x7FF0_0000_0000_0000,
    0xFFF0_0000_0000_0000,
    0x7FF8_0000_0000_0000,
    0xFFF8_0000_0000_0000,
    0x7FF0_0000_0000_0001,
    0x0000_0000_0000_0001,
    0x8000_0000_0000_0001,
    0x7FEF_FFFF_FFFF_FFFF,
    0xC000_0000_0000_0000,
    0x4059_0000_0000_0000,
];

// Tokenizer vtable stubs.

/// The token fetcher: fills the buffer from the queued script.
pub extern "thiscall" fn stub_fetch(this: u32, buf: u32, len: u32) -> u32 {
    let item = rt::pop_fetch();
    assert!(
        item.bytes.len() as u32 <= len,
        "fetch script longer than the buffer"
    );
    for (i, byte) in item.bytes.iter().enumerate() {
        unsafe { ((buf.wrapping_add(i as u32)) as *mut u8).write(*byte) };
    }
    rt::record_virtual("fetch", vec![this, len], vec![item.bytes.clone()]);
    item.len
}

/// The float reader.
pub extern "thiscall" fn stub_read_float(this: u32, flag: u32) -> f32 {
    rt::record_virtual("read_float", vec![this, flag], Vec::new());
    f32::from_bits(rt::virtual_answer("read_float"))
}

/// The flagged integer slot.
pub extern "thiscall" fn stub_fwd_int(this: u32, flag: u32) -> u32 {
    rt::record_virtual("fwd_int", vec![this, flag], Vec::new());
    rt::virtual_answer("fwd_int")
}

macro_rules! fwd_val_stub {
    ($name:ident, $slot:literal) => {
        /// One value-forward slot.
        pub extern "thiscall" fn $name(this: u32, value: u32, flag: u32) -> u32 {
            rt::record_virtual($slot, vec![this, value, flag], Vec::new());
            rt::virtual_answer($slot)
        }
    };
}

fwd_val_stub!(stub_fwd_1c, "fwd_1c");
fwd_val_stub!(stub_fwd_20, "fwd_20");
fwd_val_stub!(stub_fwd_24, "fwd_24");
fwd_val_stub!(stub_fwd_28, "fwd_28");
fwd_val_stub!(stub_fwd_2c, "fwd_2c");

/// The channel close role.
pub extern "thiscall" fn stub_close(obj: u32) -> u32 {
    rt::record_virtual("close", vec![obj], Vec::new());
    rt::virtual_answer("close")
}

/// The channel span-sink role.
pub extern "thiscall" fn stub_post(
    obj: u32,
    sink: u32,
    lo: u32,
    hi: u32,
    a3: u32,
    a4: u32,
) -> u32 {
    rt::record_virtual("post", vec![obj, sink, lo, hi, a3, a4], Vec::new());
    rt::virtual_answer("post")
}

/// The tokenizer vtable slot offsets the rewrites call through.
pub mod slot {
    /// The token fetcher.
    pub const FETCH: usize = 0x08;
    /// The flagged integer slot.
    pub const FWD_INT: usize = 0x14;
    /// The float reader.
    pub const READ_FLOAT: usize = 0x18;
    /// The five value-forward slots.
    pub const FWD_1C: usize = 0x1C;
    /// The five value-forward slots.
    pub const FWD_20: usize = 0x20;
    /// The five value-forward slots.
    pub const FWD_24: usize = 0x24;
    /// The five value-forward slots.
    pub const FWD_28: usize = 0x28;
    /// The five value-forward slots.
    pub const FWD_2C: usize = 0x2C;
}

/// Plants the full tokenizer vtable.
#[must_use]
pub fn tokenizer_vtable() -> VTable {
    let mut vt = VTable::zeroed(12);
    vt.set(slot::FETCH, stub_fetch as *const () as usize as u32);
    vt.set(slot::FWD_INT, stub_fwd_int as *const () as usize as u32);
    vt.set(
        slot::READ_FLOAT,
        stub_read_float as *const () as usize as u32,
    );
    vt.set(slot::FWD_1C, stub_fwd_1c as *const () as usize as u32);
    vt.set(slot::FWD_20, stub_fwd_20 as *const () as usize as u32);
    vt.set(slot::FWD_24, stub_fwd_24 as *const () as usize as u32);
    vt.set(slot::FWD_28, stub_fwd_28 as *const () as usize as u32);
    vt.set(slot::FWD_2C, stub_fwd_2c as *const () as usize as u32);
    vt
}

/// Maps a value-forward slot to its stub name.
#[must_use]
pub const fn slot_name(slot: ForwardSlot) -> &'static str {
    match slot {
        ForwardSlot::V1c => "fwd_1c",
        ForwardSlot::V20 => "fwd_20",
        ForwardSlot::V24 => "fwd_24",
        ForwardSlot::V28 => "fwd_28",
        ForwardSlot::V2c => "fwd_2c",
    }
}

// Fakes: the lifted traits fed from the same scripts.

/// One recorded [`StreamWorld`] call, in words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamCall {
    /// `resolve(key)`.
    Resolve(u32),
    /// `shifted` of the entry at an offset.
    Shifted(i32),
    /// `data_word` of the entry at an offset.
    DataWord(i32),
    /// `close_channel_object` (raw cookie word).
    Close(u32),
    /// `post_span` (raw cookie word, then words).
    Post(u32, u32, u32, u32, u32, u32),
}

/// The [`StreamWorld`] fake: scripted answers, recorded calls.
pub struct StreamFake {
    /// Queued resolve answers.
    pub resolve: std::collections::VecDeque<Option<StreamEntry>>,
    /// Queued shift answers.
    pub shifted: std::collections::VecDeque<u32>,
    /// Queued data answers.
    pub data: std::collections::VecDeque<u32>,
    /// Queued post answers.
    pub post: std::collections::VecDeque<u32>,
    /// The recorded calls.
    pub log: Vec<StreamCall>,
}

impl StreamFake {
    /// An empty fake.
    #[must_use]
    pub fn new() -> Self {
        Self {
            resolve: std::collections::VecDeque::new(),
            shifted: std::collections::VecDeque::new(),
            data: std::collections::VecDeque::new(),
            post: std::collections::VecDeque::new(),
            log: Vec::new(),
        }
    }
}

impl Default for StreamFake {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamWorld for StreamFake {
    fn resolve(&mut self, key: u32) -> Option<StreamEntry> {
        self.log.push(StreamCall::Resolve(key));
        self.resolve.pop_front().expect("fake resolve unscripted")
    }

    fn shifted(&mut self, entry: &StreamEntry) -> u32 {
        self.log.push(StreamCall::Shifted(entry.offset));
        self.shifted.pop_front().expect("fake shifted unscripted")
    }

    fn data_word(&mut self, entry: &StreamEntry) -> u32 {
        self.log.push(StreamCall::DataWord(entry.offset));
        self.data.pop_front().expect("fake data unscripted")
    }

    fn close_channel_object(&mut self, object: Option<lf_core::Handle32<lf_files_memory::tokenizer::ChannelTag>>) {
        self.log.push(StreamCall::Close(raw(object)));
    }

    fn post_span(
        &mut self,
        object: Option<lf_core::Handle32<lf_files_memory::tokenizer::ChannelTag>>,
        sink_arg: u32,
        lo: u32,
        hi: u32,
        a3: u32,
        a4: u32,
    ) -> u32 {
        self.log.push(StreamCall::Post(
            raw(object),
            sink_arg,
            lo,
            hi,
            a3,
            a4,
        ));
        self.post.pop_front().expect("fake post unscripted")
    }
}

/// One recorded [`TokenWorld`] call.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenCall {
    /// `refill()`.
    Refill,
    /// `skip_comment()`.
    SkipComment,
    /// `fetch_token(len)`.
    Fetch(u32),
    /// `parse_int(text)`.
    ParseInt(Vec<u8>),
    /// `parse_float(text)`.
    ParseFloat(Vec<u8>),
    /// `read_float(flag)`.
    ReadFloat(u32),
    /// `read_flagged_int(flag)`.
    FlaggedInt(u32),
    /// `forward_value(slot, value)`.
    Forward(ForwardSlot, u32),
    /// `compare_text(expected, token)`.
    Compare(Vec<u8>, Vec<u8>),
    /// `write_indent(level)`.
    Indent(u32),
    /// `write_byte_slow(byte)`.
    ByteSlow(u8),
    /// `put_char(byte)`.
    PutChar(u8),
    /// `write_text(text)`.
    WriteText(Vec<u8>),
    /// `format_value(value)`.
    Format(u32),
    /// `write_bytes(bytes)`.
    WriteBytes(Vec<u8>),
}

/// The [`TokenWorld`] fake: scripted answers, recorded calls.
pub struct TokenFake {
    /// Queued refill answers.
    pub refill: std::collections::VecDeque<Option<u8>>,
    /// Queued fetch answers.
    pub fetch: std::collections::VecDeque<TokenFetch>,
    /// Queued integer answers.
    pub ints: std::collections::VecDeque<u32>,
    /// Queued double answers, as bits.
    pub doubles: std::collections::VecDeque<u64>,
    /// Queued float answers, as bits.
    pub floats: std::collections::VecDeque<u32>,
    /// Queued flagged answers.
    pub flagged: std::collections::VecDeque<u32>,
    /// Queued forward answers.
    pub forward: std::collections::VecDeque<u32>,
    /// Queued text-write answers.
    pub written: std::collections::VecDeque<u32>,
    /// Queued formatted bytes.
    pub formatted: std::collections::VecDeque<Vec<u8>>,
    /// Queued block-write answers.
    pub blocks: std::collections::VecDeque<u32>,
    /// The recorded calls.
    pub log: Vec<TokenCall>,
}

impl TokenFake {
    /// An empty fake.
    #[must_use]
    pub fn new() -> Self {
        Self {
            refill: std::collections::VecDeque::new(),
            fetch: std::collections::VecDeque::new(),
            ints: std::collections::VecDeque::new(),
            doubles: std::collections::VecDeque::new(),
            floats: std::collections::VecDeque::new(),
            flagged: std::collections::VecDeque::new(),
            forward: std::collections::VecDeque::new(),
            written: std::collections::VecDeque::new(),
            formatted: std::collections::VecDeque::new(),
            blocks: std::collections::VecDeque::new(),
            log: Vec::new(),
        }
    }
}

impl Default for TokenFake {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenWorld for TokenFake {
    fn refill(&mut self) -> Option<u8> {
        self.log.push(TokenCall::Refill);
        self.refill.pop_front().expect("fake refill unscripted")
    }

    fn skip_comment(&mut self) {
        self.log.push(TokenCall::SkipComment);
    }

    fn fetch_token(&mut self, len: u32) -> TokenFetch {
        self.log.push(TokenCall::Fetch(len));
        self.fetch.pop_front().expect("fake fetch unscripted")
    }

    fn parse_int(&mut self, text: &[u8]) -> u32 {
        self.log.push(TokenCall::ParseInt(text.to_vec()));
        self.ints.pop_front().expect("fake int unscripted")
    }

    fn parse_float(&mut self, text: &[u8]) -> f64 {
        self.log.push(TokenCall::ParseFloat(text.to_vec()));
        f64::from_bits(self.doubles.pop_front().expect("fake double unscripted"))
    }

    fn read_float(&mut self, flag: u32) -> f32 {
        self.log.push(TokenCall::ReadFloat(flag));
        f32::from_bits(self.floats.pop_front().expect("fake float unscripted"))
    }

    fn read_flagged_int(&mut self, flag: u32) -> u32 {
        self.log.push(TokenCall::FlaggedInt(flag));
        self.flagged.pop_front().expect("fake flagged unscripted")
    }

    fn forward_value(&mut self, slot: ForwardSlot, value: u32) -> u32 {
        self.log.push(TokenCall::Forward(slot, value));
        self.forward.pop_front().expect("fake forward unscripted")
    }

    fn compare_text(&mut self, expected: &[u8], token: &[u8]) -> u32 {
        self.log
            .push(TokenCall::Compare(expected.to_vec(), token.to_vec()));
        0
    }

    fn write_indent(&mut self, level: u32) {
        self.log.push(TokenCall::Indent(level));
    }

    fn write_byte_slow(&mut self, byte: u8) {
        self.log.push(TokenCall::ByteSlow(byte));
    }

    fn put_char(&mut self, byte: u8) {
        self.log.push(TokenCall::PutChar(byte));
    }

    fn write_text(&mut self, text: &[u8]) -> u32 {
        self.log.push(TokenCall::WriteText(text.to_vec()));
        self.written.pop_front().expect("fake written unscripted")
    }

    fn format_value(&mut self, value: u32) -> Vec<u8> {
        self.log.push(TokenCall::Format(value));
        self.formatted
            .pop_front()
            .expect("fake format unscripted")
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> u32 {
        self.log.push(TokenCall::WriteBytes(bytes.to_vec()));
        self.blocks.pop_front().expect("fake block unscripted")
    }
}
