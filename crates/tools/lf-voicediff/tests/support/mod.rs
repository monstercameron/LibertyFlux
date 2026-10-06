//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by each `diff_*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

// Each target uses a different subset of the helpers.
#![allow(dead_code)]

use lf_audio::voice::{
    AdpcmWorld, AuxTag, BufferTag, ChildTag, CodecTag, DSoundWorld, DeviceTag, MixerTag, PcWorld,
    SoftWorld,
};
use lf_core::Handle32;
use lf_voicediff::rt;
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

    pub fn u16(&mut self) -> u16 {
        self.next() as u16
    }

    /// Any f32 bit pattern, NaN payloads and subnormals included.
    pub fn f32_bits(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }

    /// A small count-like word: edge values and small randoms.
    pub fn count(&mut self) -> u32 {
        const EDGE: [u32; 10] = [
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
        ];
        if self.u32() % 2 == 0 {
            EDGE[(self.u32() as usize) % EDGE.len()]
        } else {
            self.u32() % 100_000
        }
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

/// Edge flag bytes: each bit alone, interesting pairs, all/none.
pub const FLAG_EDGE: [u8; 16] = [
    0x00, 0x01, 0x02, 0x08, 0x10, 0x40, 0x09, 0x41, // none, single bits, two pairs
    0x12, 0x18, 0x11, 0xFF, 0x7F, 0x80, 0x04,
    0x20, // more pairs, all, all but the top, the rest
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
// runtime and answers from its scripted queue. Out-words are written
// from the `<name>.out` queue.
extern "thiscall" fn gate_stub(this: u32) -> u32 {
    rt::record_virtual("gate", vec![this]);
    rt::virtual_answer("gate")
}

extern "thiscall" fn fallback_stub(this: u32) -> u32 {
    rt::record_virtual("fallback", vec![this]);
    0
}

extern "stdcall" fn cursor_stub(dev: u32, out: u32, zero: u32) -> u32 {
    rt::record_virtual("cursor", vec![dev, out, zero]);
    let pos = rt::virtual_answer("cursor.out");
    unsafe {
        (out as *mut u32).write_unaligned(pos);
    }
    0
}

extern "stdcall" fn resume_stub(dev: u32, a: u32, b: u32, looping: u32) -> u32 {
    rt::record_virtual("resume", vec![dev, a, b, looping]);
    0
}

extern "stdcall" fn seek_stub(dev: u32, pos_addr: u32) -> u32 {
    rt::record_virtual("seek", vec![dev, pos_addr]);
    0
}

extern "stdcall" fn release_stub(dev: u32) -> u32 {
    rt::record_virtual("release", vec![dev]);
    0
}

extern "stdcall" fn channel_stub(dev: u32, val: u32) -> u32 {
    rt::record_virtual("channel", vec![dev, val]);
    0
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    /// The state-gate slot (+0x18 on the voice table).
    pub gate: u32,
    /// The mode-fallback slot (+0x14 on the voice table).
    pub fallback: u32,
    /// The device play-cursor slot (+0x10 on the device table).
    pub cursor: u32,
    /// The device resume slot (+0x30 on the device table).
    pub resume: u32,
    /// The device seek slot (+0x24 on the device table).
    pub seek: u32,
    /// The device release slot (+0x08 on the device table).
    pub release: u32,
    /// The channel notify slot (+0x34 on the device table).
    pub channel: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            gate: gate_stub as *const () as usize as u32,
            fallback: fallback_stub as *const () as usize as u32,
            cursor: cursor_stub as *const () as usize as u32,
            resume: resume_stub as *const () as usize as u32,
            seek: seek_stub as *const () as usize as u32,
            release: release_stub as *const () as usize as u32,
            channel: channel_stub as *const () as usize as u32,
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
/// and records every call. One fake implements all four voice traits.
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

impl SoftWorld for Fake {
    fn child_query(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("soft.query", vec![words(child)])
    }
    fn child_stop(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("soft.stop", vec![words(child)])
    }
    fn child_resume(&mut self, child: Option<Handle32<ChildTag>>, mode: u32) {
        self.call_unit("soft.resume", vec![words(child), mode]);
    }
    fn child_start(&mut self, child: Option<Handle32<ChildTag>>, rate: u32) {
        self.call_unit("soft.start", vec![words(child), rate]);
    }
    fn child_poll(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("soft.poll", vec![words(child)])
    }
    fn child_shutdown(&mut self, child: Option<Handle32<ChildTag>>) {
        self.call_unit("soft.shutdown", vec![words(child)]);
    }
    fn own_start(&mut self, mode: u32) {
        self.call_unit("soft.own_start", vec![mode]);
    }
    fn restart(&mut self) -> u32 {
        self.call("soft.restart", vec![])
    }
    fn convert_rate(&mut self, word: u32, base: u32) -> u32 {
        self.call("soft.rate", vec![word, base])
    }
    fn report_level(&mut self, level: u32) {
        self.call_unit("soft.level", vec![level]);
    }
    fn free_buffer(&mut self, buffer: Option<Handle32<BufferTag>>) {
        self.call_unit("soft.free", vec![words(buffer)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("soft.base", vec![])
    }
    fn state_gate(&mut self) -> u32 {
        self.call("soft.gate", vec![])
    }
    fn mode_fallback(&mut self) {
        self.call_unit("soft.fallback", vec![]);
    }
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32 {
        self.call("soft.pos", vec![scaled, base])
    }
    fn refine_count(&mut self, count: u32, rate: u32) -> u32 {
        self.call("soft.refine", vec![count, rate])
    }
    fn consume_samples(&mut self, aux: Option<Handle32<AuxTag>>, count: u32) -> u32 {
        self.call("soft.consume", vec![words(aux), count])
    }
}

impl PcWorld for Fake {
    fn child_query(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("pc.query", vec![words(child)])
    }
    fn child_stop(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("pc.stop", vec![words(child)])
    }
    fn child_resume(&mut self, child: Option<Handle32<ChildTag>>, mode: u32) {
        self.call_unit("pc.resume", vec![words(child), mode]);
    }
    fn child_start(&mut self, child: Option<Handle32<ChildTag>>, rate: u32) {
        self.call_unit("pc.start", vec![words(child), rate]);
    }
    fn child_poll(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("pc.poll", vec![words(child)])
    }
    fn child_measure(&mut self, child: Option<Handle32<ChildTag>>) -> u32 {
        self.call("pc.measure", vec![words(child)])
    }
    fn child_shutdown(&mut self, child: Option<Handle32<ChildTag>>) {
        self.call_unit("pc.shutdown", vec![words(child)]);
    }
    fn own_start(&mut self, mode: u32) {
        self.call_unit("pc.own_start", vec![mode]);
    }
    fn restart(&mut self) -> u32 {
        self.call("pc.restart", vec![])
    }
    fn convert_rate(&mut self, word: u32, base: u32) -> u32 {
        self.call("pc.rate", vec![word, base])
    }
    fn report_level(&mut self, level: u32) {
        self.call_unit("pc.level", vec![level]);
    }
    fn free_buffer(&mut self, buffer: Option<Handle32<BufferTag>>) {
        self.call_unit("pc.free", vec![words(buffer)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("pc.base", vec![])
    }
    fn state_gate(&mut self) -> u32 {
        self.call("pc.gate", vec![])
    }
    fn mode_fallback(&mut self) {
        self.call_unit("pc.fallback", vec![]);
    }
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32 {
        self.call("pc.pos", vec![scaled, base])
    }
    fn resolve_codec(&mut self, codec: Option<Handle32<CodecTag>>, arg: u32) -> u32 {
        self.call("pc.codec", vec![words(codec), arg])
    }
    fn mixer_cursor(&mut self, mixer: Option<Handle32<MixerTag>>) -> u32 {
        self.call("pc.mixcur", vec![words(mixer)])
    }
}

impl DSoundWorld for Fake {
    fn state_gate(&mut self) -> u32 {
        self.call("ds.gate", vec![])
    }
    fn play_cursor(&mut self, device: Option<Handle32<DeviceTag>>) -> u32 {
        self.call("ds.cursor", vec![words(device)])
    }
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32 {
        self.call("ds.pos", vec![scaled, base])
    }
    fn resolve_voice(&mut self, count: u32, rate: u32) -> u32 {
        self.call("ds.resolve", vec![count, rate])
    }
    fn consume_stream(&mut self, count: u32) -> u32 {
        self.call("ds.consume", vec![count])
    }
    fn resolve_length(&mut self, pos: u32, rate: u32) -> u32 {
        self.call("ds.length", vec![pos, rate])
    }
    fn notify_start(&mut self, mode: u32) {
        self.call_unit("ds.notify", vec![mode]);
    }
    fn channel_set_length(&mut self, device: Option<Handle32<DeviceTag>>, len: u32) {
        self.call_unit("ds.channel", vec![words(device), len]);
    }
    fn finish_start(&mut self, level: u32) -> u32 {
        self.call("ds.finish", vec![level])
    }
    fn release_device(&mut self, device: Option<Handle32<DeviceTag>>) {
        self.call_unit("ds.release", vec![words(device)]);
    }
    fn base_teardown(&mut self) -> u32 {
        self.call("ds.base", vec![])
    }
    fn device_resume(&mut self, device: Option<Handle32<DeviceTag>>, looping: bool) {
        self.call_unit("ds.resume", vec![words(device), u32::from(looping)]);
    }
    fn device_seek(&mut self, device: Option<Handle32<DeviceTag>>, pos: u32) {
        self.call_unit("ds.seek", vec![words(device), pos]);
    }
}

impl AdpcmWorld for Fake {
    fn state_gate(&mut self) -> u32 {
        self.call("ad.gate", vec![])
    }
    fn play_cursor(&mut self, device: Option<Handle32<DeviceTag>>) -> u32 {
        self.call("ad.cursor", vec![words(device)])
    }
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32 {
        self.call("ad.pos", vec![scaled, base])
    }
    fn resolve_codec(&mut self, codec: Option<Handle32<CodecTag>>, arg: u32) -> u32 {
        self.call("ad.codec", vec![words(codec), arg])
    }
    fn convert_rate(&mut self, word: u32, base: u32) -> u32 {
        self.call("ad.rate", vec![word, base])
    }
    fn resolve_length(&mut self, pos: u32, rate: u32) -> u32 {
        self.call("ad.length", vec![pos, rate])
    }
    fn refill_region(&mut self, mode: u32) {
        self.call_unit("ad.refill", vec![mode]);
    }
    fn channel_set_cursor(&mut self, device: Option<Handle32<DeviceTag>>, cursor: u32) {
        self.call_unit("ad.channel", vec![words(device), cursor]);
    }
    fn forward_gain(&mut self, gain: u32) {
        self.call_unit("ad.gain", vec![gain]);
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
