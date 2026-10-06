//! Shared differential-test support: generator, images, stubs, fakes.
//!
//! Included by the `diff.rs` test target (`#[path]`). 32-bit only:
//! addresses are real.

// The target uses a subset of the helpers.
#![allow(dead_code)]

use lf_core::Handle32;
use lf_cutdiff::rt;
use lf_world::cutscene_object::{
    AttachTag, BlockTag, BoneRow, BoneTag, BoundsScale, ChainTag, CtxTag, CutsceneObject,
    CutsceneWorld, DrawTag, EarlyTag, EntryTag, HelperTag, Matrix34, MemberTag, PlacementTag,
    PoseRecord, UpdateEntry,
};
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

    /// Any f32 bit pattern, NaN payloads and subnormals included.
    pub fn f32_bits(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }
}

/// Edge floats: signed zeros, ones, halves, the rectangle seeds, range
/// ends, infinities, quiet and signalling NaNs.
pub const F32_EDGE: [f32; 14] = [
    0.0,
    -0.0,
    1.0,
    -1.0,
    0.5,
    2.0, // small magnitudes
    1e6,
    -1e6, // the rectangle seeds
    f32::MAX,
    f32::MIN,
    f32::MIN_POSITIVE, // range ends
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN, // non-finite
];

/// Edge words: zero, one, sign boundary, all ones, small counts.
pub const U32_EDGE: [u32; 8] = [
    0,
    1,
    2,
    3, // tiny
    0x7FFF_FFFF,
    0x8000_0000, // sign boundary
    0xFFFF_FFFE,
    0xFFFF_FFFF, // all ones
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
    pub fn zeroed(size: usize) -> Self {
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

    pub fn r16(&self, off: usize) -> u16 {
        u16::from_le_bytes(self.buf[off..off + 2].try_into().unwrap())
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
extern "thiscall" fn pose_stub(this: u32, scratch: u32) -> u32 {
    rt::record_virtual("pose", vec![this, scratch]);
    rt::virtual_answer("pose")
}

extern "thiscall" fn followup_stub(this: u32) -> u32 {
    rt::record_virtual("followup", vec![this]);
    rt::virtual_answer("followup")
}

extern "thiscall" fn successor_stub(member: u32, word: u32) -> u32 {
    rt::record_virtual("successor", vec![member, word]);
    rt::virtual_answer("successor")
}

extern "thiscall" fn helper_stub(helper: u32, a: u32, b: u32) -> u32 {
    rt::record_virtual("helper", vec![helper, a, b]);
    rt::virtual_answer("helper")
}

extern "thiscall" fn destroy_stub(member: u32, flag: u32) -> u32 {
    rt::record_virtual("destroy", vec![member, flag]);
    rt::virtual_answer("destroy")
}

extern "thiscall" fn guard_a_stub(this: u32) -> u32 {
    rt::record_virtual("guard_a", vec![this]);
    rt::virtual_answer("guard_a")
}

extern "thiscall" fn guard_b_stub(this: u32) -> u32 {
    rt::record_virtual("guard_b", vec![this]);
    rt::virtual_answer("guard_b")
}

extern "thiscall" fn probe_stub(member: u32) -> u32 {
    rt::record_virtual("probe", vec![member]);
    rt::virtual_answer("probe")
}

/// Addresses of the virtual-slot stubs.
pub struct Stubs {
    /// The pose-record slot (+0x54 on the object table).
    pub pose: u32,
    /// The follow-up slot (+0x58 on the object table).
    pub followup: u32,
    /// The member successor slot (+0x20 on the member table).
    pub successor: u32,
    /// The helper command slot (+0x08 on the helper table).
    pub helper: u32,
    /// The member deleting entry (+0x00 on the member table).
    pub destroy: u32,
    /// The first guard slot (+0x24 on the object table).
    pub guard_a: u32,
    /// The second guard slot (+0x28 on the object table).
    pub guard_b: u32,
    /// The member probe slot (+0x08 on the member table).
    pub probe: u32,
}

impl Stubs {
    // Function addresses travel as words into the fake vtables.
    pub fn new() -> Self {
        Self {
            pose: pose_stub as *const () as usize as u32,
            followup: followup_stub as *const () as usize as u32,
            successor: successor_stub as *const () as usize as u32,
            helper: helper_stub as *const () as usize as u32,
            destroy: destroy_stub as *const () as usize as u32,
            guard_a: guard_a_stub as *const () as usize as u32,
            guard_b: guard_b_stub as *const () as usize as u32,
            probe: probe_stub as *const () as usize as u32,
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
/// and records every call.
#[derive(Default)]
pub struct Fake {
    answers: HashMap<&'static str, VecDeque<u32>>,
    blocks: HashMap<&'static str, VecDeque<Vec<u32>>>,
    entries: VecDeque<UpdateEntry>,
    brows: VecDeque<BoneRow>,
    /// The modelled setup store cell.
    pub store_cell: u32,
    /// Recorded lift-side calls: method name and argument words.
    pub log: Vec<(String, Vec<u32>)>,
}

impl Fake {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues scalar answers for role `name` (popped in call order;
    /// exhausted queues answer 0).
    pub fn answer(&mut self, name: &'static str, values: Vec<u32>) -> &mut Self {
        self.answers.insert(name, values.into_iter().collect());
        self
    }

    /// Queues word-block answers for role `name` (popped in call order;
    /// a call with no block queued panics).
    pub fn answer_block(&mut self, name: &'static str, values: Vec<Vec<u32>>) -> &mut Self {
        self.blocks.insert(name, values.into_iter().collect());
        self
    }

    /// Queues table entries (popped in call order; an empty queue panics).
    pub fn answer_entries(&mut self, values: Vec<UpdateEntry>) -> &mut Self {
        self.entries = values.into_iter().collect();
        self
    }

    /// Queues bone rows (popped in call order; an empty queue panics).
    pub fn answer_brows(&mut self, values: Vec<BoneRow>) -> &mut Self {
        self.brows = values.into_iter().collect();
        self
    }

    fn call(&mut self, name: &'static str, args: Vec<u32>) -> u32 {
        self.log.push((name.to_string(), args));
        self.answers
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }

    fn call_block(&mut self, name: &'static str, args: Vec<u32>) -> Vec<u32> {
        self.log.push((name.to_string(), args));
        self.blocks
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or_else(|| panic!("no block queued for fake role {name}"))
    }

    fn call_unit(&mut self, name: &'static str, args: Vec<u32>) {
        self.log.push((name.to_string(), args));
    }
}

impl CutsceneWorld for Fake {
    fn placement_matrix(&mut self, placement: Handle32<PlacementTag>) -> [u32; 16] {
        let b = self.call_block("matrix", vec![Handle32::raw_or_zero(Some(placement))]);
        assert_eq!(b.len(), 16, "matrix block must hold sixteen words");
        let mut m = [0u32; 16];
        m.copy_from_slice(&b);
        m
    }

    fn transform_corner(
        &mut self,
        placement: Handle32<PlacementTag>,
        corner: [f32; 3],
    ) -> [f32; 2] {
        let b = self.call_block(
            "corner",
            vec![
                Handle32::raw_or_zero(Some(placement)),
                corner[0].to_bits(),
                corner[1].to_bits(),
                corner[2].to_bits(),
            ],
        );
        assert_eq!(b.len(), 2, "corner block must hold two words");
        [f32::from_bits(b[0]), f32::from_bits(b[1])]
    }

    fn pose_slot(&mut self) -> PoseRecord {
        let b = self.call_block("pose", vec![]);
        assert_eq!(b.len(), 4, "pose block must hold four words");
        PoseRecord {
            head: b[0],
            mid0: f32::from_bits(b[1]),
            mid1: f32::from_bits(b[2]),
            tail: b[3],
        }
    }

    fn pose_followup(&mut self) -> u32 {
        self.call("followup", vec![])
    }

    fn notify(&mut self, word: u32) -> u32 {
        self.call("notify", vec![word])
    }

    fn member_forward(&mut self, member: Handle32<MemberTag>, word: u32) -> u32 {
        self.call("forward", vec![Handle32::raw_or_zero(Some(member)), word])
    }

    fn refresh_hook(&mut self) {
        self.call_unit("hook", vec![]);
    }

    fn refresh_run(&mut self) -> u32 {
        self.call("refresh", vec![])
    }

    fn helper_command(&mut self, helper: Handle32<HelperTag>, a: u32, b: u32) -> u32 {
        self.call("helper", vec![Handle32::raw_or_zero(Some(helper)), a, b])
    }

    fn draw_mode0_a(&mut self, a0: u32, a1: u32) {
        self.call_unit("draw.a", vec![a0, a1]);
    }

    fn draw_mode0_b(&mut self, a0: u32, a1: u32) {
        self.call_unit("draw.b", vec![a0, a1]);
    }

    fn draw_emit(&mut self, a0: u32, a1: u32, a2: u32) -> Option<Handle32<DrawTag>> {
        Handle32::new(self.call("draw.emit", vec![a0, a1, a2]))
    }

    fn mark_emitted(&mut self, target: Handle32<DrawTag>) {
        self.call_unit("draw.mark", vec![Handle32::raw_or_zero(Some(target))]);
    }

    fn destroy_member(&mut self, member: Handle32<MemberTag>) {
        self.call_unit("destroy", vec![Handle32::raw_or_zero(Some(member))]);
    }

    fn teardown_block(&mut self, block: Handle32<BlockTag>) {
        self.call_unit("block.teardown", vec![Handle32::raw_or_zero(Some(block))]);
    }

    fn free_block(&mut self, block: Handle32<BlockTag>) {
        self.call_unit("block.free", vec![Handle32::raw_or_zero(Some(block))]);
    }

    fn ask_registry(&mut self) -> u32 {
        self.call("reg.ask", vec![])
    }

    fn registry_word(&mut self, index: i16) -> i32 {
        self.call("reg.word", vec![index as u32]) as i32
    }

    fn ask_gate(&mut self, word: u32) -> u32 {
        self.call("reg.gate", vec![word])
    }

    fn registry_run(&mut self) {
        self.call_unit("reg.run", vec![]);
    }

    fn context_run(&mut self, ctx: Option<Handle32<CtxTag>>, mode: u32) {
        self.call_unit("reg.ctx", vec![words(ctx), mode]);
    }

    fn registry_tell(&mut self, word: u32) {
        self.call_unit("reg.tell", vec![word]);
    }

    fn base_destroy(&mut self) -> u32 {
        self.call("base", vec![])
    }

    fn entry_notify(&mut self) {
        self.call_unit("entry.1", vec![]);
    }

    fn entry_second(&mut self) {
        self.call_unit("entry.2", vec![]);
    }

    fn guard_a(&mut self) -> u32 {
        self.call("guard.a", vec![])
    }

    fn guard_b(&mut self) -> u32 {
        self.call("guard.b", vec![])
    }

    fn member_probe(&mut self, member: Handle32<MemberTag>) {
        self.call_unit("probe", vec![Handle32::raw_or_zero(Some(member))]);
    }

    fn table_entry(&mut self, index: i16) -> UpdateEntry {
        let e = self.entries.pop_front().expect("table entry queued");
        self.log.push((
            "table".to_string(),
            vec![
                index as u32,
                Handle32::raw_or_zero(Some(e.id)),
                u32::from(e.flag),
                e.mode,
                e.weight.to_bits(),
                e.index_words[0],
                e.index_words[1],
                e.index_words[2],
                e.index_words[3],
            ],
        ));
        e
    }

    fn early_block(&mut self) -> Handle32<EarlyTag> {
        Handle32::new(self.call("early.block", vec![])).expect("early block nonzero")
    }

    fn early_word(&mut self, block: Handle32<EarlyTag>) -> u32 {
        self.call("early.word", vec![Handle32::raw_or_zero(Some(block))])
    }

    fn early_tail(&mut self, word: u32) -> u32 {
        self.call("early.tail", vec![word])
    }

    fn early_call(&mut self, sx: i32, v294: u32, v310: u32) -> u32 {
        self.call("early.call", vec![sx as u32, v294, v310])
    }

    fn setup_primary(
        &mut self,
        entry: Handle32<EntryTag>,
        record: Option<Handle32<AttachTag>>,
        fixed: [u32; 6],
    ) {
        let mut args = vec![Handle32::raw_or_zero(Some(entry)), words(record)];
        args.extend_from_slice(&fixed);
        self.call_unit("setup.9", args);
    }

    fn setup_secondary(
        &mut self,
        entry: Handle32<EntryTag>,
        record: Option<Handle32<AttachTag>>,
        fixed: [u32; 8],
    ) {
        let mut args = vec![Handle32::raw_or_zero(Some(entry)), words(record)];
        args.extend_from_slice(&fixed);
        self.call_unit("setup.10", args);
    }

    fn store_setup(&mut self, chain: Handle32<ChainTag>, value: f32) {
        self.call_unit(
            "store",
            vec![Handle32::raw_or_zero(Some(chain)), value.to_bits()],
        );
        self.store_cell = value.to_bits();
    }

    fn bone_row(&mut self, index: u32) -> BoneRow {
        let r = self.brows.pop_front().expect("bone row queued");
        self.log.push((
            "bone".to_string(),
            vec![
                index,
                Handle32::raw_or_zero(Some(r.set)),
                r.xyz[0].to_bits(),
                r.xyz[1].to_bits(),
                r.xyz[2].to_bits(),
            ],
        ));
        r
    }

    fn submit_j(
        &mut self,
        record: Option<Handle32<AttachTag>>,
        first: [u32; 4],
        second: [u32; 4],
        third: [u32; 4],
        scalars: [u32; 4],
    ) -> u32 {
        let mut args = vec![words(record)];
        args.extend_from_slice(&first);
        args.extend_from_slice(&second);
        args.extend_from_slice(&third);
        args.extend_from_slice(&scalars);
        self.call("submit.j", args)
    }

    fn submit_k(
        &mut self,
        record: Option<Handle32<AttachTag>>,
        first: [u32; 4],
        second: [u32; 4],
        third: [u32; 4],
        scalars: [u32; 4],
    ) -> u32 {
        let mut args = vec![words(record)];
        args.extend_from_slice(&first);
        args.extend_from_slice(&second);
        args.extend_from_slice(&third);
        args.extend_from_slice(&scalars);
        self.call("submit.k", args)
    }
}

/// Reads the twelve record floats out of a matrix image.
pub fn read_matrix(img: &Image) -> Matrix34 {
    Matrix34 {
        vx: [
            f32::from_bits(img.r32(0)),
            f32::from_bits(img.r32(4)),
            f32::from_bits(img.r32(8)),
        ],
        vy: [
            f32::from_bits(img.r32(0x10)),
            f32::from_bits(img.r32(0x14)),
            f32::from_bits(img.r32(0x18)),
        ],
        vz: [
            f32::from_bits(img.r32(0x20)),
            f32::from_bits(img.r32(0x24)),
            f32::from_bits(img.r32(0x28)),
        ],
        origin: [
            f32::from_bits(img.r32(0x30)),
            f32::from_bits(img.r32(0x34)),
            f32::from_bits(img.r32(0x38)),
        ],
    }
}

/// Writes the twelve record floats into a matrix image (pads untouched).
pub fn write_matrix(img: &mut Image, m: &Matrix34) {
    for (i, v) in m.vx.iter().enumerate() {
        img.w32(i * 4, v.to_bits());
    }
    for (i, v) in m.vy.iter().enumerate() {
        img.w32(0x10 + i * 4, v.to_bits());
    }
    for (i, v) in m.vz.iter().enumerate() {
        img.w32(0x20 + i * 4, v.to_bits());
    }
    for (i, v) in m.origin.iter().enumerate() {
        img.w32(0x30 + i * 4, v.to_bits());
    }
}

/// A random record with edge and arbitrary floats.
pub fn random_matrix(rng: &mut Rng) -> Matrix34 {
    let pick = |rng: &mut Rng| {
        if rng.u32() % 3 == 0 {
            F32_EDGE[(rng.u32() as usize) % F32_EDGE.len()]
        } else {
            rng.f32_bits()
        }
    };
    Matrix34 {
        vx: [pick(rng), pick(rng), pick(rng)],
        vy: [pick(rng), pick(rng), pick(rng)],
        vz: [pick(rng), pick(rng), pick(rng)],
        origin: [pick(rng), pick(rng), pick(rng)],
    }
}

/// A random shared-state word block for the box computation.
pub fn random_scales(rng: &mut Rng) -> BoundsScale {
    // Masks: the true absolute mask, all ones, zero, sign-only, arbitrary.
    const MASKS: [u32; 5] = [
        0x7FFF_FFFF, // clears the sign
        0xFFFF_FFFF, // keeps everything
        0x0000_0000, // clears everything
        0x8000_0000, // keeps the sign
        0x7F80_0000, // keeps the exponent
    ];
    BoundsScale {
        gx: rng.f32_bits(),
        gy: rng.f32_bits(),
        gz: rng.f32_bits(),
        abs_mask: if rng.u32() % 2 == 0 {
            MASKS[(rng.u32() as usize) % MASKS.len()]
        } else {
            rng.u32()
        },
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

/// Builds the lifted object owning the same words an image holds. The
/// matrix image supplies the attached record when the link is nonzero.
#[allow(clippy::too_many_arguments)]
pub fn lift_from(
    obj: &Image,
    mat: Option<&Image>,
    triple: usize,
    attach: usize,
    helper: usize,
    hflag: usize,
    flag: usize,
    rfa: usize,
    rfb: usize,
    radius: usize,
    corner_a: usize,
    corner_b: usize,
    member: usize,
    mode: usize,
    flags_24: usize,
    table_index: usize,
    ctx: usize,
    gate: usize,
    member_b: usize,
    block0: usize,
    block1: usize,
    block2: usize,
    done: usize,
    script: usize,
    chain: usize,
) -> CutsceneObject {
    let attached = if obj.r32(attach) == 0 {
        None
    } else {
        Some(read_matrix(
            mat.expect("attached link with no matrix image"),
        ))
    };
    CutsceneObject {
        inline_triple: [obj.r32(triple), obj.r32(triple + 4), obj.r32(triple + 8)],
        placement: Handle32::new(obj.at(triple)).expect("placement address is nonzero"),
        attached,
        helper: Handle32::new(obj.at(helper)).expect("helper address is nonzero"),
        helper_flag: obj.r8(hflag),
        flag_word: obj.r32(flag),
        refresh_a: obj.r8(rfa),
        refresh_b: obj.r8(rfb),
        radius: f32::from_bits(obj.r32(radius)),
        corner_a: [
            f32::from_bits(obj.r32(corner_a)),
            f32::from_bits(obj.r32(corner_a + 4)),
            f32::from_bits(obj.r32(corner_a + 8)),
        ],
        corner_b: [
            f32::from_bits(obj.r32(corner_b)),
            f32::from_bits(obj.r32(corner_b + 4)),
            f32::from_bits(obj.r32(corner_b + 8)),
        ],
        member_a: cookie(obj.r32(member)),
        mode: obj.r32(mode),
        flags_24: obj.r32(flags_24),
        table_index: obj.r16(table_index) as i16,
        ctx: cookie(obj.r32(ctx)),
        gate_d4: obj.r32(gate),
        member_b: cookie(obj.r32(member_b)),
        blocks: [
            cookie(obj.r32(block0)),
            cookie(obj.r32(block1)),
            cookie(obj.r32(block2)),
        ],
        done_2ac: obj.r8(done),
        script_word: obj.r16(script),
        store_chain: cookie(obj.r32(chain)),
        attached_id: cookie(obj.r32(attach)),
    }
}
