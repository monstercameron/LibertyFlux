//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by each `diff_*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

// Each target uses a different subset of the helpers.
#![allow(dead_code)]

use lf_fontstr_diff::rt;
use lf_input_frontend::font_string::{FontString, FontWorld, StringRow};
use std::collections::{HashMap, VecDeque};

// String object field offsets, as the verified rewrites use them.
pub const TAG_B: usize = 0x1D4;
pub const MODE: usize = 0x1D8;
pub const STYLE: usize = 0x1DC;
pub const SIZE: usize = 0x1E0;
pub const SCALE: usize = 0x1E4;
pub const PUSH_A: usize = 0x1E8;
pub const PUSH_B: usize = 0x1EC;
pub const TAG: usize = 0x1F4;
pub const CORNER_A: usize = 0x1F8;
pub const CORNER_B: usize = 0x1FC;
pub const POS: usize = 0x200;
pub const STYLED: usize = 0x208;
pub const F209: usize = 0x209;
pub const F20A: usize = 0x20A;
pub const WMODE: usize = 0x20B;
pub const F20C: usize = 0x20C;
pub const F20D: usize = 0x20D;
pub const TEXT: usize = 0x20E;
pub const TEXT_NUL: usize = 0x30D;
pub const ROW_TEXT: usize = 0x30E;
// Row descriptor fields, relative to the row base (object + slot * 68).
pub const R_CA: usize = 0xF0;
pub const R_CB: usize = 0xF4;
pub const R_POS: usize = 0x100;
pub const R_WB: usize = 0x108;
pub const R_WA: usize = 0x10C;
pub const R_COL: usize = 0x120;
pub const R_TAG: usize = 0x124;
pub const R_SCR: usize = 0x12C;
pub const R_RDY: usize = 0x130;
pub const ROW_STRIDE: usize = 68;
pub const OBJ_SIZE: usize = 0x700;

// Virtual slots, as the verified rewrites use them.
pub const SLOT_NOTIFY_A: usize = 0x14;
pub const SLOT_RENDER: usize = 0x74;
pub const SLOT_ADV: usize = 0x8C;
pub const SLOT_SINK_PRI: usize = 0x94;
pub const SLOT_SINK_SCL: usize = 0xA0;
pub const SLOT_VIS: usize = 0xB4;
pub const SLOT_MET_C: usize = 0xB8;
pub const SLOT_SINK_H: usize = 0xBC;
pub const SLOT_MET_D: usize = 0xC0;
pub const SLOT_SINK_L: usize = 0xC4;
pub const SLOT_MET_A: usize = 0xC8;
pub const SLOT_MET_B: usize = 0xD0;
pub const SLOT_NOTIFY_B: usize = 0x13C;
pub const SLOT_GUARD: usize = 0x140;

// Shared-global file addresses, as the verified rewrites use them.
pub const G_SCALE85: u32 = 0x00FE_8830;
pub const G_DEFAULT: u32 = 0x0105_76F8;
pub const G_MUL: u32 = 0x0105_76FC;
pub const G_EXT_C: u32 = 0x0105_C880;
pub const G_EXT_D: u32 = 0x0105_C87C;
pub const G_EXT_A: u32 = 0x0105_C884;
pub const G_EXT_B: u32 = 0x0105_C888;
pub const G_DIV: u32 = 0x017A_668C;
pub const G_SLOT86: u32 = 0x017A_65A8;
pub const G_SLOT87: u32 = 0x017A_65AC;
pub const G_UIMODE: u32 = 0x0116_C250;
pub const G_UIALT: u32 = 0x0116_C253;

/// Small deterministic generator (splitmix64).
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    pub fn u8(&mut self) -> u8 {
        self.next() as u8
    }

    /// Any f32 bit pattern, NaN payloads and subnormals included.
    pub fn f32_bits(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }

    /// A byte string of `len` nonzero bytes plus the terminator.
    pub fn cstr(&mut self, len: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(len + 1);
        for _ in 0..len {
            v.push(self.u8() | 0x01);
        }
        v.push(0);
        v
    }
}

/// Edge words: zero, one, small counts, sign boundaries, all ones.
pub const U32_EDGE: [u32; 12] = [
    0,
    1,
    2,
    3, // tiny
    0x7FFF_FFFF,
    0x8000_0000, // sign boundary
    0xFFFF_FFFE,
    0xFFFF_FFFF, // all ones
    0x10000,
    0xFFFF,
    0x100,
    0xFF, // byte and word rims
];

/// Edge float bits: zeros, ones, infinities, NaNs, extremes.
/// (One value per line: the table stays a list of words, not a dump.)
pub const F32_EDGE: [u32; 14] = [
    0x0000_0000, // +0
    0x8000_0000, // -0
    0x3F80_0000, // 1
    0xBF80_0000, // -1
    0x7F80_0000, // +inf
    0xFF80_0000, // -inf
    0x7FC0_0000, // quiet NaN
    0xFFC0_0001, // negative quiet NaN, payload 1
    0x7F80_0001, // signalling NaN
    0x0000_0001, // smallest subnormal
    0x007F_FFFF, // largest subnormal
    0x7F7F_FFFF, // largest finite
    0x4000_0000, // 2
    0xC000_0000, // -2
];

/// Address of a referent as the rewrites take it (32-bit target only).
pub fn addr<T>(r: &T) -> u32 {
    (r as *const T).addr() as u32
}

/// A pinned byte image with word/byte views.
pub struct Image {
    /// Pinned bytes; stable while borrowed.
    pub buf: Box<[u8]>,
}

impl Image {
    /// Randomized filler of `size` bytes.
    pub fn random(size: usize, rng: &mut Rng) -> Self {
        let mut v = vec![0u8; size];
        for b in v.iter_mut() {
            *b = rng.u8();
        }
        Self {
            buf: v.into_boxed_slice(),
        }
    }

    /// Zeroed filler of `size` bytes.
    pub fn zeros(size: usize) -> Self {
        Self {
            buf: vec![0u8; size].into_boxed_slice(),
        }
    }

    /// An image over the given bytes.
    pub fn from_vec(v: Vec<u8>) -> Self {
        Self {
            buf: v.into_boxed_slice(),
        }
    }

    /// Base address.
    pub fn addr(&self) -> u32 {
        addr(&self.buf[0])
    }

    /// Address of byte `off`.
    pub fn at(&self, off: usize) -> u32 {
        self.addr().wrapping_add(off as u32)
    }

    pub fn r32(&self, off: usize) -> u32 {
        u32::from_le_bytes(self.buf[off..off + 4].try_into().unwrap())
    }

    pub fn r8(&self, off: usize) -> u8 {
        self.buf[off]
    }

    pub fn w32(&mut self, off: usize, v: u32) {
        self.buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    pub fn w8(&mut self, off: usize, v: u8) {
        self.buf[off] = v;
    }

    pub fn wbytes(&mut self, off: usize, v: &[u8]) {
        self.buf[off..off + v.len()].copy_from_slice(v);
    }
}

/// A pinned fake vtable: words addressed by byte slot offset.
pub struct VTable {
    /// Pinned words; stable while borrowed.
    pub buf: Box<[u32]>,
}

impl VTable {
    /// `words` words of randomized filler.
    pub fn random(words: usize, rng: &mut Rng) -> Self {
        let v: Vec<u32> = (0..words).map(|_| rng.u32()).collect();
        Self {
            buf: v.into_boxed_slice(),
        }
    }

    /// Base address.
    pub fn addr(&self) -> u32 {
        addr(&self.buf[0])
    }

    /// Plants `target` at byte slot `slot`.
    pub fn set(&mut self, slot: usize, target: u32) {
        self.buf[slot / 4] = target;
    }
}

// Virtual-slot stubs: each records its name and arguments through the
// runtime and answers from its scripted queue.
extern "thiscall" fn metric_a_stub(this: u32) -> f32 {
    rt::record_virtual("metric_a", vec![this]);
    f32::from_bits(rt::virtual_answer("metric_a"))
}

extern "thiscall" fn metric_b_stub(this: u32) -> f32 {
    rt::record_virtual("metric_b", vec![this]);
    f32::from_bits(rt::virtual_answer("metric_b"))
}

extern "thiscall" fn metric_c_stub(this: u32) -> f32 {
    rt::record_virtual("metric_c", vec![this]);
    f32::from_bits(rt::virtual_answer("metric_c"))
}

extern "thiscall" fn metric_d_stub(this: u32) -> f32 {
    rt::record_virtual("metric_d", vec![this]);
    f32::from_bits(rt::virtual_answer("metric_d"))
}

extern "thiscall" fn render_stub(this: u32) -> u32 {
    rt::record_virtual("render", vec![this]);
    rt::virtual_answer("render")
}

extern "thiscall" fn guard_stub(this: u32) -> u32 {
    rt::record_virtual("guard", vec![this]);
    rt::virtual_answer("guard")
}

extern "thiscall" fn sink_pri_stub(this: u32, bits: u32) -> u32 {
    rt::record_virtual("sink_primary", vec![this, bits]);
    rt::virtual_answer("sink_primary")
}

extern "thiscall" fn notify_a_stub(this: u32, one: u32) -> u32 {
    rt::record_virtual("notify_a", vec![this, one]);
    rt::virtual_answer("notify_a")
}

extern "thiscall" fn notify_b_stub(this: u32, one: u32) -> u32 {
    rt::record_virtual("notify_b", vec![this, one]);
    rt::virtual_answer("notify_b")
}

extern "thiscall" fn vis_stub(this: u32) -> u32 {
    rt::record_virtual("vis", vec![this]);
    rt::virtual_answer("vis")
}

extern "thiscall" fn adv_stub(this: u32) -> f32 {
    rt::record_virtual("adv", vec![this]);
    f32::from_bits(rt::virtual_answer("adv"))
}

extern "thiscall" fn sink_h_stub(this: u32, bits: u32) -> u32 {
    rt::record_virtual("sink_h", vec![this, bits]);
    rt::virtual_answer("sink_h")
}

extern "thiscall" fn sink_l_stub(this: u32, bits: u32) -> u32 {
    rt::record_virtual("sink_l", vec![this, bits]);
    rt::virtual_answer("sink_l")
}

extern "thiscall" fn sink_s_stub(this: u32, bits: u32) -> u32 {
    rt::record_virtual("sink_s", vec![this, bits]);
    rt::virtual_answer("sink_s")
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    pub metric_a: u32,
    pub metric_b: u32,
    pub metric_c: u32,
    pub metric_d: u32,
    pub render: u32,
    pub guard: u32,
    pub sink_pri: u32,
    pub notify_a: u32,
    pub notify_b: u32,
    pub vis: u32,
    pub adv: u32,
    pub sink_h: u32,
    pub sink_l: u32,
    pub sink_s: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            metric_a: metric_a_stub as *const () as usize as u32,
            metric_b: metric_b_stub as *const () as usize as u32,
            metric_c: metric_c_stub as *const () as usize as u32,
            metric_d: metric_d_stub as *const () as usize as u32,
            render: render_stub as *const () as usize as u32,
            guard: guard_stub as *const () as usize as u32,
            sink_pri: sink_pri_stub as *const () as usize as u32,
            notify_a: notify_a_stub as *const () as usize as u32,
            notify_b: notify_b_stub as *const () as usize as u32,
            vis: vis_stub as *const () as usize as u32,
            adv: adv_stub as *const () as usize as u32,
            sink_h: sink_h_stub as *const () as usize as u32,
            sink_l: sink_l_stub as *const () as usize as u32,
            sink_s: sink_s_stub as *const () as usize as u32,
        }
    }
}

impl Default for Stubs {
    fn default() -> Self {
        Self::new()
    }
}

/// One recorded lift-side call: method name, argument words, and the
/// bytes behind a buffer argument.
pub struct LiftCall {
    pub name: String,
    pub words: Vec<u32>,
    pub bytes: Vec<u8>,
}

/// The scripted world fake: answers every collaborator role from queues
/// and records every call.
#[derive(Default)]
pub struct Fake {
    num: HashMap<&'static str, VecDeque<u32>>,
    pairs: VecDeque<[u32; 2]>,
    groups: VecDeque<[u32; 8]>,
    /// Recorded lift-side calls.
    pub log: Vec<LiftCall>,
}

impl Fake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues word answers for role `name` (popped in call order;
    /// exhausted queues answer 0).
    pub fn answer(&mut self, name: &'static str, values: Vec<u32>) -> &mut Self {
        self.num.insert(name, values.into_iter().collect());
        self
    }

    /// Queues registry pair answers.
    pub fn answer_pairs(&mut self, values: Vec<[u32; 2]>) -> &mut Self {
        self.pairs = values.into_iter().collect();
        self
    }

    /// Queues converted-word group answers.
    pub fn answer_groups(&mut self, values: Vec<[u32; 8]>) -> &mut Self {
        self.groups = values.into_iter().collect();
        self
    }

    fn pop(&mut self, name: &'static str) -> u32 {
        self.num
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }

    fn call(&mut self, name: &'static str, words: Vec<u32>) -> u32 {
        self.log.push(LiftCall {
            name: name.to_string(),
            words,
            bytes: Vec::new(),
        });
        self.pop(name)
    }

    fn call_unit(&mut self, name: &'static str, words: Vec<u32>) {
        self.log.push(LiftCall {
            name: name.to_string(),
            words,
            bytes: Vec::new(),
        });
    }

    fn call_bytes(&mut self, name: &'static str, words: Vec<u32>, bytes: &[u8]) -> u32 {
        self.log.push(LiftCall {
            name: name.to_string(),
            words,
            bytes: bytes.to_vec(),
        });
        self.pop(name)
    }
}

impl FontWorld for Fake {
    fn notify_position(&mut self, snap: [u32; 4]) {
        self.call_unit("notify_position", snap.to_vec());
    }
    fn resolve_text(&mut self, key: u32) -> [u32; 2] {
        self.log.push(LiftCall {
            name: "resolve_text".to_string(),
            words: vec![key],
            bytes: Vec::new(),
        });
        self.pairs.pop_front().unwrap_or([0, 0])
    }
    fn resolve_sized(&mut self, size: u32) -> [u32; 2] {
        self.log.push(LiftCall {
            name: "resolve_sized".to_string(),
            words: vec![size],
            bytes: Vec::new(),
        });
        self.pairs.pop_front().unwrap_or([0, 0])
    }
    fn refresh_parent(&mut self) {
        self.call_unit("refresh_parent", vec![]);
    }
    fn copy_text(&mut self, text: &[u8; 256]) {
        self.call_unit("copy_text", vec![]);
        self.log.last_mut().expect("just pushed").bytes = text.to_vec();
    }
    fn guard_veto(&mut self) -> bool {
        self.call("guard_veto", vec![]) & 0xff != 0
    }
    fn sink_primary(&mut self, bits: u32) -> u32 {
        self.call("sink_primary", vec![bits])
    }
    fn notify_a(&mut self) {
        self.call_unit("notify_a", vec![]);
    }
    fn notify_b(&mut self) {
        self.call_unit("notify_b", vec![]);
    }
    fn metric_a(&mut self) -> f32 {
        f32::from_bits(self.call("metric_a", vec![]))
    }
    fn metric_b(&mut self) -> f32 {
        f32::from_bits(self.call("metric_b", vec![]))
    }
    fn metric_c(&mut self) -> f32 {
        f32::from_bits(self.call("metric_c", vec![]))
    }
    fn metric_d(&mut self) -> f32 {
        f32::from_bits(self.call("metric_d", vec![]))
    }
    fn render(&mut self) -> u32 {
        self.call("render", vec![])
    }
    fn refresh_base(&mut self) {
        self.call_unit("refresh_base", vec![]);
    }
    fn push_scale(&mut self, bits: u32) {
        self.call_unit("push_scale", vec![bits]);
    }
    fn visible(&mut self) -> bool {
        self.call("visible", vec![]) & 0xff != 0
    }
    fn advance(&mut self) -> f32 {
        f32::from_bits(self.call("advance", vec![]))
    }
    fn push_style(&mut self, style: u32) {
        self.call_unit("push_style", vec![style]);
    }
    fn push_position(&mut self, lo: u32, hi: u32) {
        self.call_unit("push_position", vec![lo, hi]);
    }
    fn push_a(&mut self, bits: u32) {
        self.call_unit("push_a", vec![bits]);
    }
    fn push_colour(&mut self, colour: u32) {
        self.call_unit("push_colour", vec![colour]);
    }
    fn push_row_word(&mut self, bits: u32) {
        self.call_unit("push_row_word", vec![bits]);
    }
    fn push_one(&mut self) {
        self.call_unit("push_one", vec![]);
    }
    fn push_width(&mut self, base: u32, add: u32) {
        self.call_unit("push_width", vec![base, add]);
    }
    fn push_opacity(&mut self, bits: u32) {
        self.call_unit("push_opacity", vec![bits]);
    }
    fn convert_text(&mut self, text: &[u8]) -> [u32; 8] {
        self.log.push(LiftCall {
            name: "convert_text".to_string(),
            words: Vec::new(),
            bytes: text.to_vec(),
        });
        self.groups.pop_front().unwrap_or([0; 8])
    }
    fn resolve_cached(&mut self, text: &[u8]) -> [u32; 8] {
        self.log.push(LiftCall {
            name: "resolve_cached".to_string(),
            words: Vec::new(),
            bytes: text.to_vec(),
        });
        self.groups.pop_front().unwrap_or([0; 8])
    }
    fn submit(&mut self, corner_a: u32, corner_b: u32, words: &[u32; 8]) -> u32 {
        let mut logged = vec![corner_a, corner_b];
        logged.extend_from_slice(words);
        self.call("submit", logged)
    }
    fn query_height(&mut self, words: &[u32; 8]) -> f32 {
        self.log.push(LiftCall {
            name: "query_height".to_string(),
            words: words.to_vec(),
            bytes: Vec::new(),
        });
        f32::from_bits(self.pop("query_height"))
    }
    fn line_height(&mut self) -> f32 {
        f32::from_bits(self.call("line_height", vec![]))
    }
    fn sink_height(&mut self, bits: u32) {
        self.call_unit("sink_height", vec![bits]);
    }
    fn sink_line(&mut self, bits: u32) {
        self.call_unit("sink_line", vec![bits]);
    }
    fn sink_scaled(&mut self, bits: u32) {
        self.call_unit("sink_scaled", vec![bits]);
    }
    fn pick_ext_a(&mut self) -> bool {
        self.call("pick_ext_a", vec![]) & 0xff != 0
    }
    fn pick_ext_b(&mut self) -> bool {
        self.call("pick_ext_b", vec![]) & 0xff != 0
    }
    fn end_frame(&mut self) {
        self.call_unit("end_frame", vec![]);
    }
}

/// Reads one row descriptor plus its text out of the object image.
pub fn row_of(obj: &Image, slot: usize) -> StringRow {
    let base = slot * ROW_STRIDE;
    let mut end = ROW_TEXT + slot * 256;
    while obj.buf[end] != 0 {
        end += 1;
        assert!(
            end < ROW_TEXT + slot * 256 + 256,
            "row text holds no NUL within its 256 bytes"
        );
    }
    StringRow {
        corner_a: f32::from_bits(obj.r32(base + R_CA)),
        corner_b: f32::from_bits(obj.r32(base + R_CB)),
        pos: [obj.r32(base + R_POS), obj.r32(base + R_POS + 4)],
        width_base: f32::from_bits(obj.r32(base + R_WB)),
        width_add: f32::from_bits(obj.r32(base + R_WA)),
        colour: obj.r32(base + R_COL),
        tag: obj.r32(base + R_TAG),
        use_scratch: obj.r8(base + R_SCR),
        ready: obj.r8(base + R_RDY) != 0,
        text: obj.buf[ROW_TEXT + slot * 256..=end].to_vec(),
    }
}

/// Builds the lifted string owning the fixture's live words and rows.
pub fn lift_of(obj: &Image, nrows: usize) -> FontString {
    let mut text = [0u8; 256];
    text.copy_from_slice(&obj.buf[TEXT..TEXT + 256]);
    let rows = (0..nrows).map(|s| row_of(obj, s)).collect();
    FontString::new(
        obj.r32(TAG_B),
        obj.r32(MODE),
        obj.r32(STYLE),
        f32::from_bits(obj.r32(SIZE)),
        f32::from_bits(obj.r32(SCALE)),
        f32::from_bits(obj.r32(PUSH_A)),
        f32::from_bits(obj.r32(PUSH_B)),
        obj.r32(TAG),
        f32::from_bits(obj.r32(CORNER_A)),
        f32::from_bits(obj.r32(CORNER_B)),
        [obj.r32(POS), obj.r32(POS + 4)],
        obj.r8(STYLED),
        obj.r8(F209),
        obj.r8(F20A),
        obj.r8(WMODE),
        obj.r8(F20C),
        obj.r8(F20D),
        text,
        rows,
    )
}

/// Asserts the rewrite's numbered calls equal the expected sequence.
pub fn check_numbered(got: Vec<(u32, Vec<u32>)>, expect: &[(u32, Vec<u32>)]) {
    assert_eq!(got, expect, "rewrite numbered calls");
}

/// Asserts the rewrite's virtual calls equal the expected sequence.
pub fn check_virtual(got: Vec<(String, Vec<u32>)>, expect: &[(&str, Vec<u32>)]) {
    let slim: Vec<(String, Vec<u32>)> = got;
    let exp: Vec<(String, Vec<u32>)> = expect
        .iter()
        .map(|(n, a)| ((*n).to_string(), a.clone()))
        .collect();
    assert_eq!(slim, exp, "rewrite virtual calls");
}

/// Asserts the lift's trait calls equal the expected sequence of
/// (name, words, bytes).
pub fn check_lift(got: &[LiftCall], expect: &[(&str, Vec<u32>, Vec<u8>)]) {
    assert_eq!(got.len(), expect.len(), "lift call count");
    for (i, (have, (name, words, bytes))) in got.iter().zip(expect.iter()).enumerate() {
        assert_eq!(have.name, *name, "lift call {i} name");
        assert_eq!(have.words, *words, "lift call {i} ({name}) words");
        assert_eq!(have.bytes, *bytes, "lift call {i} ({name}) bytes");
    }
}

/// Asserts two images agree everywhere except the `changed` byte ranges
/// (start, length), which the caller compares separately.
pub fn assert_only_changed(before: &[u8], after: &[u8], changed: &[(usize, usize)]) {
    assert_eq!(before.len(), after.len(), "image sizes");
    let mut masked = vec![false; before.len()];
    for (start, len) in changed {
        for i in *start..(*start + *len).min(masked.len()) {
            masked[i] = true;
        }
    }
    for (i, (a, b)) in before.iter().zip(after.iter()).enumerate() {
        assert!(
            masked[i] || a == b,
            "byte {i:#x} changed: {a:#x} -> {b:#x}, outside {changed:?}"
        );
    }
}
