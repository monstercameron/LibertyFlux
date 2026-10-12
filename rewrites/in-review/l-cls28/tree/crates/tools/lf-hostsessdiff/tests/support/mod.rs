//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by each `diff_*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

// Each target uses a different subset of the helpers.
#![allow(dead_code)]

use lf_core::Handle32;
use lf_hostsessdiff::rt;
use lf_network::host_session_task::{EntryTag, HostSessionTask, HostSessionWorld, SessionTag};
use std::collections::{HashMap, VecDeque};

/// Task object field offsets, as the verified rewrites use them.
pub const T_STARTED: usize = 0x0C;
pub const T_COUNTDOWN: usize = 0x14;
pub const T_SESSION: usize = 0x60;
pub const T_STATE: usize = 0x90;
pub const T_PROBE: usize = 0x94;
pub const T_CONFIG: usize = 0x9C;
pub const T_EMPTY: usize = 0xA0;
pub const T_SPLIT: usize = 0xA4;
pub const T_FIRST: usize = 0xA8;
pub const T_SECOND: usize = 0xAC;
pub const T_GAMERS: usize = 0xB0;
pub const T_EXTRA: usize = 0x4BC;
pub const T_TSIZE: usize = 0x4C0;
pub const T_TRAILER: usize = 0x4C4;
/// Task image size: the trailer record plus scripted words fit after this.
pub const TASK_SIZE: usize = 0x600;

/// Session object offsets, as the verified rewrites use them.
pub const S_SUB: usize = 0x24;
pub const S_TIMER: usize = 0x28;
pub const S_PORT: usize = 0x48;
pub const S_MODE: usize = 0x50;
pub const S_PRIMARY: usize = 0xBB0;
pub const S_MIR_SRC: usize = 0xBF0;
pub const S_MIR_DST: usize = 0xC30;
pub const S_AUX: usize = 0xC6C;
pub const S_FLAG: usize = 0x32F8;
/// Session image size: the flag byte is the last word touched.
pub const SESSION_SIZE: usize = 0x3400;

/// The completion virtual's slot offset on the task table.
pub const SLOT_COMPLETE: usize = 0x1C;

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
}

/// Edge words: zero, one, small counts, sign boundaries, all ones.
pub const U32_EDGE: [u32; 12] = [
    0, 1, 2, 3, // tiny
    0x20, 0x21, 0x1F, // the clamp boundary and its neighbours
    0x7FFF_FFFF, 0x8000_0000, // sign boundary
    0xFFFF_FFFE, 0xFFFF_FFFF, // all ones and below
    0xDEAD_BEEF, // arbitrary bits
];

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

    /// Base address.
    pub fn addr(&self) -> u32 {
        (self.buf.as_ptr() as usize) as u32
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
        (self.buf.as_ptr() as usize) as u32
    }

    /// Plants `target` at byte slot `slot`.
    pub fn set(&mut self, slot: usize, target: u32) {
        self.buf[slot / 4] = target;
    }
}

extern "thiscall" fn complete_stub(this: u32, a: u32, b: u32) -> u32 {
    rt::record_virtual("complete", vec![this, a, b]);
    rt::virtual_answer("complete")
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    /// The completion slot (+0x1c on the task table).
    pub complete: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            complete: complete_stub as *const () as usize as u32,
        }
    }
}

impl Default for Stubs {
    fn default() -> Self {
        Self::new()
    }
}

/// Opaque cookie for a nonzero test address, `None` for null.
pub fn cookie<T>(a: u32) -> Option<Handle32<T>> {
    Handle32::new(a)
}

/// Cookie back to words for call logs.
pub fn words<T>(c: Option<Handle32<T>>) -> u32 {
    Handle32::raw_or_zero(c)
}

/// The scripted world fake: answers every collaborator role from queues,
/// emulates the session flag byte and the entry flag byte across reads
/// and writes, and records every call. Out-pointers the rewrite side
/// writes through get the same scripted words here.
pub struct Fake {
    answers: HashMap<&'static str, VecDeque<u32>>,
    outs: HashMap<&'static str, VecDeque<Vec<u32>>>,
    /// Recorded lift-side calls: method name and argument words.
    pub log: Vec<(String, Vec<u32>)>,
    /// Emulated session flag byte.
    pub session_flag_byte: u8,
    /// Emulated host-entry flag byte.
    pub entry_flag_byte: u8,
}

impl Fake {
    pub fn new() -> Self {
        Self {
            answers: HashMap::new(),
            outs: HashMap::new(),
            log: Vec::new(),
            session_flag_byte: 0,
            entry_flag_byte: 0,
        }
    }

    /// Queues answers for role `name` (popped in call order; exhausted
    /// queues answer 0).
    pub fn answer(&mut self, name: &'static str, values: Vec<u32>) -> &mut Self {
        self.answers.insert(name, values.into_iter().collect());
        self
    }

    /// Queues out-pointer rows for role `name` (one row per call; a
    /// missing row leaves the target unchanged).
    pub fn out_words(&mut self, name: &'static str, rows: Vec<Vec<u32>>) -> &mut Self {
        self.outs.insert(name, rows.into_iter().collect());
        self
    }

    fn call(&mut self, name: &'static str, args: Vec<u32>) -> u32 {
        self.log.push((name.to_string(), args));
        self.answers
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }

    fn call_unit(&mut self, name: &'static str, args: Vec<u32>) {
        self.log.push((name.to_string(), args));
    }

    fn out_one(&mut self, name: &'static str) -> Option<u32> {
        self.outs
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .and_then(|row| row.into_iter().next())
    }
}

impl Default for Fake {
    fn default() -> Self {
        Self::new()
    }
}

impl HostSessionWorld for Fake {
    fn session_state(&mut self, session: Option<Handle32<SessionTag>>) -> u32 {
        self.call("session_state", vec![words(session)])
    }
    fn own_complete(&mut self, path: u32, note: u32) {
        self.call_unit("own_complete", vec![path, note]);
    }
    fn append_trailer(&mut self, trailer: &mut Vec<u32>, arg: u32, len: u32) -> u32 {
        self.call_unit("trailer", vec![arg, len]);
        *trailer = self
            .outs
            .get_mut("trailer.words")
            .and_then(VecDeque::pop_front)
            .unwrap_or_default();
        self.answers
            .get_mut("trailer")
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }
    fn prepare_primary(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("prepare_primary", vec![words(session)]);
    }
    fn prepare_session(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("prepare_session", vec![words(session)]);
    }
    fn open_plain(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        net: u32,
        first: u32,
        _gamers: &[u32; 259],
        probe: &mut u32,
    ) -> u32 {
        self.log.push((
            "open_plain".to_string(),
            vec![words(session), net, first, *probe],
        ));
        if let Some(v) = self.out_one("open_plain.probe") {
            *probe = v;
        }
        self.answers
            .get_mut("open_plain")
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }
    #[allow(clippy::too_many_arguments)]
    fn open_hosted(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        net: u32,
        split: u32,
        first: u32,
        second: u32,
        _gamers: &[u32; 259],
        extra: u32,
        probe: &mut u32,
    ) -> u32 {
        self.log.push((
            "open_hosted".to_string(),
            vec![words(session), net, split, first, second, extra, *probe],
        ));
        if let Some(v) = self.out_one("open_hosted.probe") {
            *probe = v;
        }
        self.answers
            .get_mut("open_hosted")
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }
    fn promote_hosted(&mut self) {
        self.call_unit("promote_hosted", vec![]);
    }
    fn resolve_host_key(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        key: &[u32; 4],
        empty: &mut u32,
        probe: &mut u32,
    ) -> u32 {
        self.log.push((
            "resolve_key".to_string(),
            vec![
                words(session),
                key[0],
                key[1],
                key[2],
                key[3],
                *empty,
                *probe,
            ],
        ));
        if let Some(v) = self.out_one("resolve_key.empty") {
            *empty = v;
        }
        if let Some(v) = self.out_one("resolve_key.probe") {
            *probe = v;
        }
        self.answers
            .get_mut("resolve_key")
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }
    fn poll_host(&mut self, session: Option<Handle32<SessionTag>>, probe: &mut u32) -> u32 {
        self.log.push((
            "poll_host".to_string(),
            vec![words(session), *probe],
        ));
        if let Some(v) = self.out_one("poll_host.probe") {
            *probe = v;
        }
        self.answers
            .get_mut("poll_host")
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }
    fn progress_check(&mut self) -> u32 {
        self.call("progress", vec![])
    }
    fn mirror_session_block(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("mirror", vec![words(session)]);
    }
    fn session_flag(&mut self, session: Option<Handle32<SessionTag>>) -> u8 {
        self.call_unit("session_flag", vec![words(session)]);
        self.session_flag_byte
    }
    fn set_session_flag(&mut self, session: Option<Handle32<SessionTag>>, value: u8) {
        self.call_unit("session_flag.set", vec![words(session), u32::from(value)]);
        self.session_flag_byte = value;
    }
    fn session_up(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("session_up", vec![words(session)]);
    }
    fn notify_host(&mut self, path: u32, note: u32) {
        self.call_unit("notify", vec![path, note]);
    }
    fn submit_descriptor(&mut self, session: Option<Handle32<SessionTag>>, tag: u32) {
        self.call_unit("descriptor", vec![words(session), tag]);
    }
    fn derive_block_a(&mut self) {
        self.call_unit("derive_a", vec![]);
    }
    fn derive_block_b(&mut self) {
        self.call_unit("derive_b", vec![]);
    }
    fn derive_block_c(&mut self, table: u32) {
        self.call_unit("derive_c", vec![table]);
    }
    fn derive_block_d(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("derive_d", vec![words(session)]);
    }
    fn finish_key_block(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        empty: u32,
        trailer: &[u32],
        trailer_len: u32,
    ) {
        let mut args = vec![words(session), empty, trailer_len];
        args.extend_from_slice(trailer);
        self.call_unit("finish_key", args);
    }
    fn alloc_session_entry(
        &mut self,
        session: Option<Handle32<SessionTag>>,
    ) -> Option<Handle32<EntryTag>> {
        let raw = self.call("alloc_entry", vec![words(session)]);
        Handle32::new(raw)
    }
    fn entry_flags(&mut self, entry: Option<Handle32<EntryTag>>) -> u8 {
        self.call_unit("entry_flags", vec![words(entry)]);
        self.entry_flag_byte
    }
    fn set_entry_flags(&mut self, entry: Option<Handle32<EntryTag>>, value: u8) {
        self.call_unit("entry_flags.set", vec![words(entry), u32::from(value)]);
        self.entry_flag_byte = value;
    }
    fn verify_cookie(&mut self, cookie: u32) {
        self.call_unit("cookie", vec![cookie]);
    }
    fn close_session_aux(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("close_aux", vec![words(session)]);
    }
    fn release_session_timer(&mut self, session: Option<Handle32<SessionTag>>) {
        self.call_unit("release_timer", vec![words(session)]);
    }
}

/// Builds the lifted task owning the same words the task image holds.
/// The trailer vec starts empty: the setup fills it through the world.
pub fn lift_task(obj: &Image) -> HostSessionTask {
    let mut gamers = [0u32; 259];
    for (i, w) in gamers.iter_mut().enumerate() {
        *w = obj.r32(T_GAMERS + i * 4);
    }
    HostSessionTask {
        started: obj.r32(T_STARTED) != 0,
        countdown: obj.r32(T_COUNTDOWN),
        session: cookie(obj.r32(T_SESSION)),
        state: obj.r32(T_STATE),
        probe: obj.r32(T_PROBE),
        config: obj.r32(T_CONFIG),
        first_empty: obj.r32(T_EMPTY),
        list_split: obj.r32(T_SPLIT),
        first_count: obj.r32(T_FIRST),
        second_count: obj.r32(T_SECOND),
        gamers,
        open_extra: obj.r32(T_EXTRA),
        trailer_size: obj.r32(T_TSIZE),
        trailer: Vec::new(),
    }
}

/// Asserts the rewrite's numbered calls and the lift's trait calls
/// each equal their expected sequence: one entry per call with the
/// callee id and words on the rewrite side and the role name and words
/// on the lift side.
pub fn check_calls(
    numbered: Vec<(u32, Vec<u32>)>,
    lift: Vec<(String, Vec<u32>)>,
    expect: Vec<(u32, Vec<u32>, &'static str, Vec<u32>)>,
) {
    let exp_n: Vec<(u32, Vec<u32>)> = expect
        .iter()
        .map(|(id, a, _, _)| (*id, a.clone()))
        .collect();
    let exp_l: Vec<(String, Vec<u32>)> = expect
        .iter()
        .map(|(_, _, n, a)| ((*n).to_string(), a.clone()))
        .collect();
    assert_eq!(numbered, exp_n, "rewrite numbered calls");
    assert_eq!(lift, exp_l, "lift trait calls");
}

/// Asserts the rewrite's virtual-slot calls equal the expected sequence.
pub fn check_virtual(virt: Vec<(String, Vec<u32>)>, expect: Vec<(&'static str, Vec<u32>)>) {
    let exp: Vec<(String, Vec<u32>)> = expect
        .iter()
        .map(|(n, a)| ((*n).to_string(), a.clone()))
        .collect();
    assert_eq!(virt, exp, "rewrite virtual calls");
}

/// Asserts no byte outside `changed` ranges differs between two images.
pub fn assert_only_changed(before: &[u8], after: &[u8], changed: &[(usize, usize)]) {
    assert_eq!(before.len(), after.len(), "image lengths");
    for (i, (a, b)) in before.iter().zip(after.iter()).enumerate() {
        if changed.iter().any(|(s, e)| i >= *s && i < *e) {
            continue;
        }
        assert_eq!(a, b, "byte {i:#x} changed");
    }
}
