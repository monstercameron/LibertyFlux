//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by each `diff_*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

// Each target uses a different subset of the helpers.
#![allow(dead_code)]

use lf_core::Handle32;
use lf_input_frontend::ui_clip::{
    BasicClip, ClipWorld, ElementTag, EntryTableTag, EntryTag, MatchOut, PartTag, SinkTag,
    SubmitTag, TransformRecord, TripleKind,
};
use lf_uiclip_diff::rt;
use std::collections::{HashMap, VecDeque};

// Clip object field offsets, as the verified rewrites use them.
pub const VT: usize = 0x00;
pub const FLAG: usize = 0xCB;
pub const PARTS: usize = 0x1D4;
pub const SUBMIT: usize = 0x1E0;
pub const PART: usize = 0x1E4;
pub const PART2: usize = 0x1E8;
pub const SINK: usize = 0x1EC;
pub const MODE: usize = 0x2F8;
pub const STORED: usize = 0x2FC;
pub const OBJ_SIZE: usize = 0x300;

// Virtual slots, as the verified rewrites use them.
pub const SLOT_MEASURE: usize = 0x98;
pub const SLOT_SINK_PUSH: usize = 0xA0;
pub const SLOT_FWD: usize = 0x120;
pub const SLOT_PRED: usize = 0x124;
pub const SLOT_ACTION: usize = 0x13C;
pub const SLOT_PROBE: usize = 0x140;
pub const SLOT_TRANSFORM: usize = 0xF8;
pub const SLOT_MID1: usize = 0x1F8;
pub const SLOT_FIN: usize = 0x204;
pub const SLOT_MID2: usize = 0x228;
pub const SLOT_COUNT: usize = 0x1D4;
pub const SLOT_SET_TEXT: usize = 0x1E0;
pub const SLOT_GET_TEXT: usize = 0x20C;

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
pub const F32_EDGE: [u32; 16] = [
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
    0x4F00_0000, // 2^31, the truncation rim
    0xCF00_0000, // -2^31
    0x3C88_8889, // the triple splitter's own first factor
    0x4270_0000, // the triple splitter's own back factor
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
// runtime and answers from its scripted queue. Text sinks also snapshot
// the bytes behind the string pointer (256 bytes: every string image is
// that roomy, so the read is always in bounds).
extern "thiscall" fn probe_stub(this: u32) -> u32 {
    rt::record_virtual("probe", vec![this]);
    rt::virtual_answer("probe")
}

extern "thiscall" fn action_stub(this: u32, zero: u32) -> u32 {
    rt::record_virtual("action", vec![this, zero]);
    rt::virtual_answer("action")
}

extern "thiscall" fn count_stub(this: u32) -> u32 {
    rt::record_virtual("count", vec![this]);
    rt::virtual_answer("count")
}

extern "thiscall" fn measure_stub(this: u32) -> f32 {
    rt::record_virtual("measure", vec![this]);
    f32::from_bits(rt::virtual_answer("measure"))
}

extern "thiscall" fn mid1_stub(this: u32, fp: u32, zero: u32) -> u32 {
    rt::record_virtual("mid1", vec![this, fp, zero]);
    rt::virtual_answer("mid1")
}

extern "thiscall" fn mid2_stub(this: u32, fp: u32, zero: u32) -> u32 {
    rt::record_virtual("mid2", vec![this, fp, zero]);
    rt::virtual_answer("mid2")
}

extern "thiscall" fn fin_stub(this: u32, one: u32) -> u32 {
    rt::record_virtual("fin", vec![this, one]);
    rt::virtual_answer("fin")
}

extern "thiscall" fn pred_stub(member: u32) -> u32 {
    rt::record_virtual("pred", vec![member]);
    rt::virtual_answer("pred")
}

extern "thiscall" fn fwd_stub(member: u32, arg: u32) -> u32 {
    rt::record_virtual("fwd", vec![member, arg]);
    rt::virtual_answer("fwd")
}

extern "thiscall" fn settext_stub(child: u32, ptr: u32, zero: u32) -> u32 {
    rt::record_virtual_snap("settext", vec![child, ptr, zero], ptr, 256);
    rt::virtual_answer("settext")
}

extern "thiscall" fn gettext_stub(child: u32) -> u32 {
    rt::record_virtual("gettext", vec![child]);
    rt::virtual_answer("gettext")
}

extern "thiscall" fn title_sink_stub(child: u32) -> u32 {
    rt::record_virtual("title_sink", vec![child]);
    rt::virtual_answer("title_sink")
}

extern "thiscall" fn title_src_stub(child: u32) -> u32 {
    rt::record_virtual("title_src", vec![child]);
    rt::virtual_answer("title_src")
}

extern "thiscall" fn submit_stub(sink: u32, ptr: u32, zero: u32) -> u32 {
    rt::record_virtual_snap("submit", vec![sink, ptr, zero], ptr, 256);
    rt::virtual_answer("submit")
}

extern "thiscall" fn sink_push_stub(submit: u32, bits: u32) -> u32 {
    rt::record_virtual("sink_push", vec![submit, bits]);
    rt::virtual_answer("sink_push")
}

extern "thiscall" fn transform_stub(part: u32) -> u32 {
    rt::record_virtual("transform", vec![part]);
    rt::virtual_answer("transform")
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    pub probe: u32,
    pub action: u32,
    pub count: u32,
    pub measure: u32,
    pub mid1: u32,
    pub mid2: u32,
    pub fin: u32,
    pub pred: u32,
    pub fwd: u32,
    pub settext: u32,
    pub gettext: u32,
    pub title_sink: u32,
    pub title_src: u32,
    pub submit: u32,
    pub sink_push: u32,
    pub transform: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            probe: probe_stub as *const () as usize as u32,
            action: action_stub as *const () as usize as u32,
            count: count_stub as *const () as usize as u32,
            measure: measure_stub as *const () as usize as u32,
            mid1: mid1_stub as *const () as usize as u32,
            mid2: mid2_stub as *const () as usize as u32,
            fin: fin_stub as *const () as usize as u32,
            pred: pred_stub as *const () as usize as u32,
            fwd: fwd_stub as *const () as usize as u32,
            settext: settext_stub as *const () as usize as u32,
            gettext: gettext_stub as *const () as usize as u32,
            title_sink: title_sink_stub as *const () as usize as u32,
            title_src: title_src_stub as *const () as usize as u32,
            submit: submit_stub as *const () as usize as u32,
            sink_push: sink_push_stub as *const () as usize as u32,
            transform: transform_stub as *const () as usize as u32,
        }
    }
}

impl Default for Stubs {
    fn default() -> Self {
        Self::new()
    }
}

/// Opaque cookie for a nonzero test word, `None` for null.
pub fn cookie<T>(a: u32) -> Option<Handle32<T>> {
    Handle32::new(a)
}

/// Cookie back to words for call logs.
pub fn words<T>(c: Option<Handle32<T>>) -> u32 {
    Handle32::raw_or_zero(c)
}

/// Distinct nonzero cookies, minted in order.
pub struct Mint(u32);

impl Mint {
    pub fn new() -> Self {
        Self(0x0010_0000)
    }

    pub fn next<T>(&mut self) -> Handle32<T> {
        let c = self.0;
        self.0 += 1;
        Handle32::new(c).expect("minted cookie must be nonzero")
    }
}

impl Default for Mint {
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
    texts: VecDeque<Option<Vec<u8>>>,
    titles: VecDeque<Vec<u8>>,
    matches: VecDeque<MatchOut>,
    xforms: VecDeque<[u8; 24]>,
    directs: VecDeque<u32>,
    tables: HashMap<u32, Vec<TransformRecord>>,
    direct_recs: HashMap<u32, TransformRecord>,
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

    /// Queues text answers for the part-text getter.
    pub fn answer_texts(&mut self, values: Vec<Option<Vec<u8>>>) -> &mut Self {
        self.texts = values.into_iter().collect();
        self
    }

    /// Queues title answers for the title source.
    pub fn answer_titles(&mut self, values: Vec<Vec<u8>>) -> &mut Self {
        self.titles = values.into_iter().collect();
        self
    }

    /// Queues matcher answers.
    pub fn answer_matches(&mut self, values: Vec<MatchOut>) -> &mut Self {
        self.matches = values.into_iter().collect();
        self
    }

    /// Queues transform-service answers.
    pub fn answer_xforms(&mut self, values: Vec<[u8; 24]>) -> &mut Self {
        self.xforms = values.into_iter().collect();
        self
    }

    /// Queues direct-entry cookies.
    pub fn answer_directs(&mut self, values: Vec<u32>) -> &mut Self {
        self.directs = values.into_iter().collect();
        self
    }

    /// Registers an entry table's records.
    pub fn add_table(&mut self, table: Handle32<EntryTableTag>, records: Vec<TransformRecord>) {
        self.tables.insert(table.get(), records);
    }

    /// Registers a direct entry's record.
    pub fn add_direct(&mut self, entry: Handle32<EntryTag>, record: TransformRecord) {
        self.direct_recs.insert(entry.get(), record);
    }

    /// The records behind a table, after the run.
    pub fn table(&self, table: Handle32<EntryTableTag>) -> &[TransformRecord] {
        &self.tables[&table.get()]
    }

    /// The record behind a direct entry, after the run.
    pub fn direct(&self, entry: Handle32<EntryTag>) -> &TransformRecord {
        &self.direct_recs[&entry.get()]
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

fn kind_word(kind: TripleKind) -> u32 {
    match kind {
        TripleKind::First => 0,
        TripleKind::Second => 1,
    }
}

impl ClipWorld for Fake {
    fn submit_word(&mut self, submit: Handle32<SubmitTag>) -> u32 {
        self.call("submit_word", vec![submit.get()])
    }
    fn set_submit_word(&mut self, submit: Handle32<SubmitTag>, value: u32) {
        self.call_unit("set_submit_word", vec![submit.get(), value]);
    }
    fn probe(&mut self) -> u32 {
        self.call("probe", vec![])
    }
    fn run_action(&mut self) -> u32 {
        self.call("run_action", vec![])
    }
    fn part_predicate(&mut self, part: Handle32<PartTag>) -> bool {
        self.call("part_predicate", vec![part.get()]) & 0xff != 0
    }
    fn forward_to_part(&mut self, part: Handle32<PartTag>, arg: u32) -> u32 {
        self.call("forward_to_part", vec![part.get(), arg])
    }
    fn part_count(&mut self) -> u32 {
        self.call("part_count", vec![])
    }
    fn set_element_flag(&mut self, element: Handle32<ElementTag>, flag: u8) {
        self.call_unit("set_element_flag", vec![element.get(), u32::from(flag)]);
    }
    fn measure(&mut self) -> f32 {
        f32::from_bits(self.call("measure", vec![]))
    }
    fn push_adjusted(&mut self, submit: Handle32<SubmitTag>, bits: u32) -> u32 {
        self.call("push_adjusted", vec![submit.get(), bits])
    }
    fn encode_triple(&mut self, kind: TripleKind, bytes: [u8; 3]) {
        self.call_unit(
            "encode_triple",
            vec![
                kind_word(kind),
                u32::from(bytes[0]),
                u32::from(bytes[1]),
                u32::from(bytes[2]),
            ],
        );
    }
    fn run_triple_mid(&mut self, kind: TripleKind) {
        self.call_unit("run_triple_mid", vec![kind_word(kind)]);
    }
    fn finish_triple(&mut self) {
        self.call_unit("finish_triple", vec![]);
    }
    fn frame_check(&mut self) -> u32 {
        self.call("frame_check", vec![])
    }
    fn set_part_text(&mut self, part: Handle32<PartTag>, bytes: &[u8]) -> u32 {
        self.call_bytes("set_part_text", vec![part.get()], bytes)
    }
    fn part_text(&mut self, part: Handle32<PartTag>) -> Option<Vec<u8>> {
        self.log.push(LiftCall {
            name: "part_text".to_string(),
            words: vec![part.get()],
            bytes: Vec::new(),
        });
        self.texts.pop_front().unwrap_or(None)
    }
    fn sink_title_present(&mut self, sink: Handle32<SinkTag>) -> bool {
        self.call("sink_title_present", vec![sink.get()]) != 0
    }
    fn source_title(&mut self, part: Handle32<PartTag>) -> Vec<u8> {
        self.log.push(LiftCall {
            name: "source_title".to_string(),
            words: vec![part.get()],
            bytes: Vec::new(),
        });
        self.titles.pop_front().unwrap_or(vec![0])
    }
    fn submit_to_sink(&mut self, sink: Handle32<SinkTag>, bytes: &[u8]) {
        self.call_unit("submit_to_sink", vec![sink.get()]);
        self.log.last_mut().expect("just pushed").bytes = bytes.to_vec();
    }
    fn fetch_handle(&mut self, part: Handle32<PartTag>) -> u32 {
        self.call("fetch_handle", vec![part.get()])
    }
    fn match_entries(&mut self, handle: u32) -> MatchOut {
        self.log.push(LiftCall {
            name: "match_entries".to_string(),
            words: vec![handle],
            bytes: Vec::new(),
        });
        self.matches.pop_front().expect("no matcher answer queued")
    }
    fn transform_bytes(&mut self, sel0: u32, sel1: u32) -> [u8; 24] {
        self.log.push(LiftCall {
            name: "transform_bytes".to_string(),
            words: vec![sel0, sel1],
            bytes: Vec::new(),
        });
        self.xforms.pop_front().expect("no transform answer queued")
    }
    fn release_service(&mut self) {
        self.call_unit("release_service", vec![]);
    }
    fn teardown_entries(&mut self, table: Handle32<EntryTableTag>) {
        self.call_unit("teardown_entries", vec![table.get()]);
    }
    fn entry_record(&mut self, table: Handle32<EntryTableTag>, index: u32) -> &mut TransformRecord {
        self.log.push(LiftCall {
            name: "entry_record".to_string(),
            words: vec![table.get(), index],
            bytes: Vec::new(),
        });
        self.tables
            .get_mut(&table.get())
            .expect("unknown entry table")
            .get_mut(index as usize)
            .expect("entry index out of range")
    }
    fn direct_entry(&mut self, handle: u32) -> Handle32<EntryTag> {
        self.log.push(LiftCall {
            name: "direct_entry".to_string(),
            words: vec![handle],
            bytes: Vec::new(),
        });
        self.directs
            .pop_front()
            .and_then(Handle32::new)
            .expect("no direct-entry cookie queued")
    }
    fn direct_record(&mut self, entry: Handle32<EntryTag>) -> &mut TransformRecord {
        self.log.push(LiftCall {
            name: "direct_record".to_string(),
            words: vec![entry.get()],
            bytes: Vec::new(),
        });
        self.direct_recs
            .get_mut(&entry.get())
            .expect("unknown direct entry")
    }
}

/// Builds the lifted clip owning the fixture's words.
#[allow(clippy::too_many_arguments)]
pub fn lift_of(
    obj: &Image,
    submit: Option<Handle32<SubmitTag>>,
    part: Option<Handle32<PartTag>>,
    part2: Option<Handle32<PartTag>>,
    sink: Option<Handle32<SinkTag>>,
    parts: Vec<Handle32<ElementTag>>,
) -> BasicClip {
    BasicClip::new(
        obj.r8(FLAG),
        obj.r8(MODE),
        f32::from_bits(obj.r32(STORED)),
        submit,
        part,
        part2,
        sink,
        parts,
    )
}

/// Asserts the rewrite's numbered calls equal the expected sequence.
pub fn check_numbered(got: Vec<(u32, Vec<u32>)>, expect: &[(u32, Vec<u32>)]) {
    assert_eq!(got, expect, "rewrite numbered calls");
}

/// Asserts the rewrite's virtual calls' names and words equal the
/// expected sequence (snapshots are asserted separately).
pub fn check_virtual_names(
    got: &[(String, Vec<u32>, Option<Vec<u8>>)],
    expect: &[(&str, Vec<u32>)],
) {
    let slim: Vec<(String, Vec<u32>)> =
        got.iter().map(|(n, a, _)| (n.clone(), a.clone())).collect();
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
