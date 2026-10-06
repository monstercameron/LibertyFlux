//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by each `diff_*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

// Each target uses a different subset of the helpers.
#![allow(dead_code)]

use lf_audio::sound::{
    CompressorWorld, EffectWorld, ListenerTag, NextTag, ReverbSubTag, ReverbWorld, SubTag,
    VoiceTag,
};
use lf_core::Handle32;
use lf_sounddiff::rt;
use std::collections::{HashMap, VecDeque};

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

/// Edge words: zero, one, sign boundary, all ones, small counts.
pub const U32_EDGE: [u32; 12] = [
    0,
    1,
    2,
    3,
    0x7FFF_FFFF,
    0x8000_0000,
    0xFFFF_FFFE,
    0xFFFF_FFFF,
    0x10000,
    0xFFFF,
    0x5E,
    0xDEAD_BEEF,
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

    pub fn w16(&mut self, off: usize, v: u16) {
        self.buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
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
        addr(&self.buf[0])
    }

    /// Plants `target` at byte slot `slot`.
    pub fn set(&mut self, slot: usize, target: u32) {
        self.buf[slot / 4] = target;
    }
}

// Virtual-slot stubs: each records its name and arguments through the
// runtime and answers from its scripted queue.
extern "thiscall" fn release_stub(obj: u32, flag: u32) -> u32 {
    rt::record_virtual("release", vec![obj, flag]);
    rt::virtual_answer("release")
}

extern "thiscall" fn voice_poll_stub(obj: u32) -> u32 {
    rt::record_virtual("voice_poll", vec![obj]);
    rt::virtual_answer("voice_poll")
}

extern "thiscall" fn notify_stub(obj: u32) -> u32 {
    rt::record_virtual("notify", vec![obj]);
    rt::virtual_answer("notify")
}

extern "thiscall" fn hook_stub(obj: u32) -> u32 {
    rt::record_virtual("hook", vec![obj]);
    rt::virtual_answer("hook")
}

extern "thiscall" fn next_stub(obj: u32) -> u32 {
    rt::record_virtual("next", vec![obj]);
    rt::virtual_answer("next")
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    /// The voice release slot (+0x00 on the voice table).
    pub release: u32,
    /// The voice poll slot (+0x08 on the voice table).
    pub voice_poll: u32,
    /// The listener notify slot (+0x14 on the listener table).
    pub notify: u32,
    /// The reverb refresh-hook slot (+0x14 on the effect table).
    pub hook: u32,
    /// The next-stage advance slot (+0x14 on the next table).
    pub next: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            release: release_stub as *const () as usize as u32,
            voice_poll: voice_poll_stub as *const () as usize as u32,
            notify: notify_stub as *const () as usize as u32,
            hook: hook_stub as *const () as usize as u32,
            next: next_stub as *const () as usize as u32,
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

/// The scripted world fake: answers every collaborator role from queues
/// and records every call. One fake implements both effect traits.
#[derive(Default)]
pub struct Fake {
    answers: HashMap<&'static str, VecDeque<u32>>,
    /// Recorded lift-side calls: method name and argument words.
    pub log: Vec<(String, Vec<u32>)>,
}

impl Fake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues answers for role `name` (popped in call order; exhausted
    /// queues answer 0).
    pub fn answer(&mut self, name: &'static str, values: Vec<u32>) -> &mut Self {
        self.answers.insert(name, values.into_iter().collect());
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
}

impl EffectWorld for Fake {
    fn release_voice(&mut self, voice: Option<Handle32<VoiceTag>>) {
        self.call_unit("fx.release", vec![words(voice)]);
    }
    fn lookup_voice(&mut self, tag: u32, param_plus_one: u32) -> Option<Handle32<VoiceTag>> {
        let ans = self.call("fx.lookup", vec![tag, param_plus_one]);
        Handle32::new(ans)
    }
    fn refresh_entry(&mut self, slot: u32) {
        self.call_unit("fx.refresh", vec![slot]);
    }
    fn poll_voice(&mut self, voice: Option<Handle32<VoiceTag>>) {
        self.call_unit("fx.poll", vec![words(voice)]);
    }
}

impl ReverbWorld for Fake {
    fn base_advance(&mut self) {
        self.call_unit("rv.base", vec![]);
    }
    fn advance_next(&mut self, next: Option<Handle32<NextTag>>) -> u32 {
        self.call("rv.next", vec![words(next)])
    }
    fn base_init(&mut self, a: u32, b: u32) -> u32 {
        self.call("rv.init", vec![a, b])
    }
    fn refresh_hook(&mut self) {
        self.call_unit("rv.hook", vec![]);
    }
    fn refresh_direct(&mut self) -> u32 {
        self.call("rv.direct", vec![])
    }
    fn pre_poll(&mut self) {
        self.call_unit("rv.pre", vec![]);
    }
    fn sub_poll(&mut self, sub: Option<Handle32<ReverbSubTag>>, slot_words: u32) {
        self.call_unit("rv.sub", vec![words(sub), slot_words]);
    }
    fn post_poll(&mut self) -> u32 {
        self.call("rv.post", vec![])
    }
}

impl CompressorWorld for Fake {
    fn base_rotate(&mut self) {
        self.call_unit("cx.base", vec![]);
    }
    fn notify_listener(&mut self, listener: Option<Handle32<ListenerTag>>) -> u32 {
        self.call("cx.notify", vec![words(listener)])
    }
    fn set_base(&mut self, a1: u32, a2: u32) -> u32 {
        self.call("cx.set", vec![a1, a2])
    }
    fn pre_poll(&mut self) {
        self.call_unit("cx.pre", vec![]);
    }
    fn sub_poll(&mut self, sub: Option<Handle32<SubTag>>, slot_words: u32) {
        self.call_unit("cx.sub", vec![words(sub), slot_words]);
    }
    fn post_poll(&mut self) -> u32 {
        self.call("cx.post", vec![])
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

/// Asserts the rewrite's virtual-slot calls equal the expected sequence
/// of (stub name, words).
pub fn check_virtual(got: Vec<(String, Vec<u32>)>, expect: Vec<(&'static str, Vec<u32>)>) {
    let exp: Vec<(String, Vec<u32>)> = expect
        .into_iter()
        .map(|(n, a)| (n.to_string(), a))
        .collect();
    assert_eq!(got, exp, "rewrite virtual calls");
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
