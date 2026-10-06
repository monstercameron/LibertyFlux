//! The UI basic clip: a screen element that plays a short sequence.
//!
//! The original keeps one object per clip: a flag byte, a mode byte, a
//! stored float, four collaborator objects (the submit part, two part
//! objects and a text sink) and an array of part elements. The lifted
//! [`BasicClip`] owns the three words and the element array as ordinary
//! Rust data and carries the collaborators as opaque [`Handle32`] cookies,
//! reached through [`ClipWorld`]: every virtual call and every callee slot
//! of the verified rewrites is one trait method, so dispatch is static
//! and tests script answers through a fake.
//!
//! Each verified 32-bit method with behaviour in it is restated as a
//! method on [`BasicClip`]. Proof is differential: every lifted method
//! runs against its verified rewrite on the same generated inputs,
//! comparing results, every written byte and every collaborator call in
//! order, floats bit for bit (see the `lf-uiclip-diff` test crate).
//! Nothing here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`](super::registry).
//!
//! [`Handle32`]: lf_core::Handle32

use lf_core::Handle32;

/// Tag for the opaque cookie of a part object (the clip's part, second
/// part and child element share one slot interface).
pub struct PartTag;
/// Tag for the opaque cookie of the submit object.
pub struct SubmitTag;
/// Tag for the opaque cookie of the text sink.
pub struct SinkTag;
/// Tag for the opaque cookie of one part-array element.
pub struct ElementTag;
/// Tag for the opaque cookie of a transform entry table.
pub struct EntryTableTag;
/// Tag for the opaque cookie of one resolved transform entry.
pub struct EntryTag;

/// The measure multiplier the float-push path scales by, measured from
/// the executable's data (the rewrite reads it through a shared global).
pub const MEASURE_SCALE: f32 = f32::from_bits(0x3F70_A3D7);
/// The offset the float-push path subtracts either way, measured the
/// same way.
pub const MEASURE_BIAS: f32 = f32::from_bits(0x40C0_0000);

/// The triple splitter's first multiplier (one sixtieth), measured from
/// the executable's data.
pub const TRIPLE_C0: f32 = f32::from_bits(0x3C88_8889);
/// The triple splitter's first back-multiplier (sixty), measured the
/// same way.
pub const TRIPLE_C1: f32 = f32::from_bits(0x4270_0000);
/// The triple splitter's second multiplier (one hundred), measured the
/// same way.
pub const TRIPLE_C2: f32 = f32::from_bits(0x42C8_0000);

/// Which float-triple setter runs: the two verified setters share one
/// conversion chain and differ only in the engine table and the middle
/// slot they drive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TripleKind {
    /// The first setter's table and middle slot.
    First,
    /// The second setter's table and middle slot.
    Second,
}

/// The matcher answer: the entry table its handle resolved to, and the
/// gate word whose high half selects the teardown call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchOut {
    /// The resolved entry table.
    pub table: Handle32<EntryTableTag>,
    /// The gate word; the teardown runs when its high half is nonzero.
    pub gate: u32,
}

/// One cached transform record: sixteen unmodelled bytes the refresh
/// never touches, then the twenty-four transform bytes it rewrites.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransformRecord {
    /// The untouched prefix.
    pub prefix: [u8; 16],
    /// The transform bytes, rewritten by every refresh.
    pub transform: [u8; 24],
}

/// The clip's collaborators: every virtual slot and every intercepted
/// callee the lifted methods reach, as one trait method each.
///
/// Handles are opaque cookies: the world mints them, maps them to its
/// own objects, and answers from its script. Byte strings arrive with
/// their terminator (the original passes pointers to NUL-terminated
/// text); every string the world returns must likewise hold a NUL
/// within the stated bound.
pub trait ClipWorld {
    /// The submit part's stored word.
    fn submit_word(&mut self, submit: Handle32<SubmitTag>) -> u32;
    /// Stores the submit part's word.
    fn set_submit_word(&mut self, submit: Handle32<SubmitTag>, value: u32);
    /// The clip's probe slot: its low byte decides the action slot.
    fn probe(&mut self) -> u32;
    /// The clip's zero-argument action slot.
    fn run_action(&mut self) -> u32;
    /// A part's predicate slot.
    fn part_predicate(&mut self, part: Handle32<PartTag>) -> bool;
    /// A part's forward slot (one word argument).
    fn forward_to_part(&mut self, part: Handle32<PartTag>, arg: u32) -> u32;
    /// The clip's part-count slot, re-read every loop round.
    fn part_count(&mut self) -> u32;
    /// Sets one element's flag byte.
    fn set_element_flag(&mut self, element: Handle32<ElementTag>, flag: u8);
    /// The clip's measure slot: the float the push path adjusts.
    fn measure(&mut self) -> f32;
    /// Pushes adjusted float bits to the submit object.
    fn push_adjusted(&mut self, submit: Handle32<SubmitTag>, bits: u32) -> u32;
    /// The engine triple helper: encodes three bytes under a table.
    fn encode_triple(&mut self, kind: TripleKind, bytes: [u8; 3]);
    /// The clip's triple middle slot for this setter.
    fn run_triple_mid(&mut self, kind: TripleKind);
    /// The clip's triple finishing slot.
    fn finish_triple(&mut self);
    /// The frame check ending the triple setters; its answer is their
    /// result. Production answers its own check word.
    fn frame_check(&mut self) -> u32;
    /// A part's text setter: replaces the part's text.
    fn set_part_text(&mut self, part: Handle32<PartTag>, bytes: &[u8]) -> u32;
    /// A part's text getter: the current text, or nothing when null.
    fn part_text(&mut self, part: Handle32<PartTag>) -> Option<Vec<u8>>;
    /// Whether the sink's title query answers non-null.
    fn sink_title_present(&mut self, sink: Handle32<SinkTag>) -> bool;
    /// The title string behind a part's title query.
    fn source_title(&mut self, part: Handle32<PartTag>) -> Vec<u8>;
    /// The sink's submit slot.
    fn submit_to_sink(&mut self, sink: Handle32<SinkTag>, bytes: &[u8]);
    /// A part's transform-handle slot.
    fn fetch_handle(&mut self, part: Handle32<PartTag>) -> u32;
    /// Resolves a transform handle into its entry table and gate word.
    fn match_entries(&mut self, handle: u32) -> MatchOut;
    /// The transform service: twenty-four bytes for a selector pair.
    fn transform_bytes(&mut self, sel0: u32, sel1: u32) -> [u8; 24];
    /// The transform service's release call.
    fn release_service(&mut self);
    /// The entry-table teardown call.
    fn teardown_entries(&mut self, table: Handle32<EntryTableTag>);
    /// The mutable record behind one table entry.
    fn entry_record(
        &mut self,
        table: Handle32<EntryTableTag>,
        index: u32,
    ) -> &mut TransformRecord;
    /// Resolves one direct entry behind a part's transform handle (the
    /// extra mode-one slots read their entries straight off the handle).
    fn direct_entry(&mut self, handle: u32) -> Handle32<EntryTag>;
    /// The mutable record behind one direct entry.
    fn direct_record(&mut self, entry: Handle32<EntryTag>) -> &mut TransformRecord;
}

/// Truncates a float to an integer exactly like the original's
/// truncating conversion: toward zero, with the indefinite value for
/// NaN, infinities and out-of-range magnitudes.
#[must_use]
pub fn truncate_raw(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        // In range, so the cast truncates exactly, never saturating.
        x as i32
    }
}

/// Multiplies in the written order, pinned against commuting so NaN
/// payloads match the original's bit for bit.
#[inline]
fn mul_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Subtracts in the written order, pinned like [`mul_pinned`].
#[inline]
fn sub_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Splits a float into the three bytes the triple setters hand the
/// engine helper: each is the low eight bits of a truncating
/// scale-and-subtract chain, in the original's order.
#[must_use]
pub fn triple_bytes(x: f32) -> [u8; 3] {
    let b0 = (truncate_raw(mul_pinned(x, TRIPLE_C0)) & 0xff) as u8;
    let x1 = sub_pinned(x, mul_pinned(f32::from(b0), TRIPLE_C1));
    let b1 = (truncate_raw(x1) & 0xff) as u8;
    let x2 = mul_pinned(sub_pinned(x1, f32::from(b1)), TRIPLE_C2);
    let b2 = (truncate_raw(x2) & 0xff) as u8;
    [b0, b1, b2]
}

/// The bytes of a NUL-terminated string excluding the terminator.
///
/// # Panics
///
/// When `text` holds no NUL: the original reads on past the end there,
/// which has no meaning here.
#[must_use]
pub fn up_to_nul(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .position(|b| *b == 0)
        .expect("text must be NUL-terminated within bounds");
    &text[..end]
}

/// A UI basic clip: the owned words and the element array.
///
/// The collaborators (submit part, two parts, text sink) travel as
/// opaque cookies; a missing cookie panics only in the method that
/// needs it, where the original would fault on the null pointer.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BasicClip {
    /// The flag byte: set by [`BasicClip::set_flag`].
    flag: u8,
    /// The mode byte: read by [`BasicClip::mode`], selected by
    /// [`BasicClip::set_display_mode`].
    mode: u8,
    /// The stored float: written by the triple setters, read by
    /// [`BasicClip::stored`]. Its role beyond storage is unestablished.
    stored: f32,
    /// The submit part.
    submit: Option<Handle32<SubmitTag>>,
    /// The first part.
    part: Option<Handle32<PartTag>>,
    /// The second part (also the text child and the title source).
    part2: Option<Handle32<PartTag>>,
    /// The text sink.
    sink: Option<Handle32<SinkTag>>,
    /// The part elements.
    parts: Vec<Handle32<ElementTag>>,
}

impl BasicClip {
    /// A clip over the given words, collaborators and elements.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        flag: u8,
        mode: u8,
        stored: f32,
        submit: Option<Handle32<SubmitTag>>,
        part: Option<Handle32<PartTag>>,
        part2: Option<Handle32<PartTag>>,
        sink: Option<Handle32<SinkTag>>,
        parts: Vec<Handle32<ElementTag>>,
    ) -> Self {
        Self {
            flag,
            mode,
            stored,
            submit,
            part,
            part2,
            sink,
            parts,
        }
    }

    /// The flag byte.
    #[must_use]
    pub fn flag(&self) -> u8 {
        self.flag
    }

    /// The element array.
    #[must_use]
    pub fn parts(&self) -> &[Handle32<ElementTag>] {
        &self.parts
    }

    /// The mode byte.
    #[must_use]
    pub fn mode(&self) -> u8 {
        self.mode
    }

    /// The text sink.
    #[must_use]
    pub fn sink(&self) -> Option<Handle32<SinkTag>> {
        self.sink
    }

    /// The stored float, bits untouched.
    #[must_use]
    pub fn stored(&self) -> f32 {
        self.stored
    }

    /// Copies the submit part's word two levels out into `out`.
    ///
    /// # Panics
    ///
    /// When the submit part is missing: the original faults on the null
    /// pointer there.
    pub fn submit_word_into<W: ClipWorld>(&self, world: &mut W, out: &mut u32) {
        let submit = self.submit.expect("submit_word_into needs the submit part");
        *out = world.submit_word(submit);
    }

    /// Copies `value` into the submit part and answers it back.
    ///
    /// # Panics
    ///
    /// When the submit part is missing: the original faults on the null
    /// pointer there.
    pub fn store_submit_word<W: ClipWorld>(&self, world: &mut W, value: u32) -> u32 {
        let submit = self.submit.expect("store_submit_word needs the submit part");
        world.set_submit_word(submit, value);
        value
    }

    /// Probes the clip, then runs the zero-argument action when the
    /// probe's low byte is nonzero. Answers the last answer produced.
    pub fn probe_and_act<W: ClipWorld>(&self, world: &mut W) -> u32 {
        let ans = world.probe();
        if ans & 0xff != 0 {
            world.run_action()
        } else {
            ans
        }
    }

    /// Forwards a rewritten argument to the first part: 1 when the
    /// argument's low byte is 1 unless the second part's predicate
    /// passed while the mode holds anything but 1 or 2, else 0.
    /// Answers the forward's answer.
    ///
    /// # Panics
    ///
    /// When the part the path needs is missing: the original faults on
    /// the null pointer there.
    pub fn forward_conditional<W: ClipWorld>(&self, world: &mut W, arg: u32) -> u32 {
        let mut fwd = 0;
        if arg & 0xff == 1 {
            let part2 = self.part2.expect("forward_conditional needs the second part");
            let passed = world.part_predicate(part2);
            if !passed || self.mode == 1 || self.mode == 2 {
                fwd = 1;
            }
        }
        let part = self.part.expect("forward_conditional needs the first part");
        world.forward_to_part(part, fwd)
    }

    /// Sets the flag byte on the clip and on its first elements:
    /// stores `flag`, reads the part count, and while the unsigned
    /// index is below it flags one element per round, re-reading the
    /// count every round. Answers the last count read.
    ///
    /// # Panics
    ///
    /// When the count runs past the element array: the original reads
    /// whatever pointer lies there, which has no meaning here.
    pub fn set_flag<W: ClipWorld>(&mut self, world: &mut W, flag: u8) -> u32 {
        self.flag = flag;
        let mut count = world.part_count();
        if count != 0 {
            let mut i: u32 = 0;
            while i < count {
                let element = *self
                    .parts
                    .get(i as usize)
                    .expect("part count ran past the element array");
                world.set_element_flag(element, flag);
                i += 1;
                count = world.part_count();
            }
        }
        count
    }

    /// Forwards the argument to the second part, then pushes an
    /// adjusted float to the submit part: the measured float, scaled
    /// when the argument's low byte is nonzero, shifted either way.
    /// Answers the push's answer.
    ///
    /// # Panics
    ///
    /// When the second part or the submit part is missing: the original
    /// faults on the null pointer there.
    pub fn forward_and_push<W: ClipWorld>(&self, world: &mut W, arg: u32) -> u32 {
        let part2 = self.part2.expect("forward_and_push needs the second part");
        world.forward_to_part(part2, arg);
        let mut v = world.measure();
        if arg & 0xff != 0 {
            v = mul_pinned(v, MEASURE_SCALE);
        }
        v = sub_pinned(v, MEASURE_BIAS);
        let submit = self.submit.expect("forward_and_push needs the submit part");
        world.push_adjusted(submit, v.to_bits())
    }

    /// Sets a float-driven triple: splits `input` into three bytes for
    /// the engine helper, runs the middle slot, stores the untouched
    /// input bits, and finishes through the last slot. Answers the
    /// frame check's answer.
    pub fn set_triple<W: ClipWorld>(
        &mut self,
        world: &mut W,
        kind: TripleKind,
        input: f32,
    ) -> u32 {
        world.encode_triple(kind, triple_bytes(input));
        world.run_triple_mid(kind);
        self.stored = input;
        world.finish_triple();
        world.frame_check()
    }

    /// Sets the second part's text: replaces it when `append` is false,
    /// else appends to the current text when both plus the terminator
    /// fit the 256-byte scratch. A missing current text falls back to
    /// replacing. Answers the setter's answer, or the remaining free
    /// space when the combined text does not fit.
    ///
    /// # Panics
    ///
    /// When the second part is missing, when `text` holds no NUL, when
    /// the current text holds no NUL within 256 bytes, or when the
    /// getter's second answer is missing after a first one: the
    /// original faults or reads on in each case.
    pub fn set_child_text<W: ClipWorld>(
        &self,
        world: &mut W,
        text: &[u8],
        append: bool,
    ) -> u32 {
        const BUF_LEN: u32 = 256;
        let child = self.part2.expect("set_child_text needs the second part");
        let fresh = up_to_nul(text);
        if !append {
            return world.set_part_text(child, text_with_nul(text, fresh.len()));
        }
        if world.part_text(child).is_none() {
            return world.set_part_text(child, text_with_nul(text, fresh.len()));
        }
        let current = world
            .part_text(child)
            .expect("child text getter went null on its second call");
        let mut buf = [0u8; 256];
        let kept = up_to_nul_bounded(&current);
        buf[..kept.len()].copy_from_slice(kept);
        let cur_len = kept.len() as u32;
        let new_len = fresh.len() as u32;
        let free = BUF_LEN.wrapping_sub(cur_len);
        if free <= new_len {
            return free;
        }
        let end = (cur_len + new_len) as usize;
        buf[cur_len as usize..end].copy_from_slice(fresh);
        // `buf[end]` stays the zero the buffer started with: the terminator.
        world.set_part_text(child, &buf)
    }

    /// Forwards a label to the text sink, prefixing it with the second
    /// part's title when `titling` is set and the sink's title query
    /// agrees. The label is appended only when the combined text plus
    /// the terminator fits the 256-byte scratch.
    ///
    /// # Panics
    ///
    /// When the sink or the second part is missing, or when the label
    /// or the title holds no NUL within bounds: the original faults or
    /// reads on in each case.
    pub fn submit_label<W: ClipWorld>(&self, world: &mut W, label: &[u8], titling: bool) {
        const BUF_LEN: u32 = 256;
        let sink = self.sink.expect("submit_label needs the sink");
        let bare = up_to_nul(label);
        if !titling || !world.sink_title_present(sink) {
            world.submit_to_sink(sink, text_with_nul(label, bare.len()));
            return;
        }
        let part2 = self.part2.expect("submit_label needs the second part");
        let title = world.source_title(part2);
        let mut buf = [0u8; 256];
        let kept = up_to_nul_bounded(&title);
        buf[..kept.len()].copy_from_slice(kept);
        let title_len = kept.len() as u32;
        let label_len = bare.len() as u32;
        if !(BUF_LEN.wrapping_sub(title_len) <= label_len) {
            let start = title_len as usize;
            let end = start + bare.len();
            buf[start..end].copy_from_slice(bare);
            // `buf[end]` stays the zero the buffer started with.
        }
        world.submit_to_sink(sink, &buf);
    }

    /// Switches the display mode, refreshing the cached per-slot
    /// transforms: mode 0 refreshes four slots and notifies the first
    /// part, mode 2 refreshes four slots under fixed selectors, mode 1
    /// refreshes four slots and two extra ones on the second part, and
    /// any other mode (or the current one) stores the byte with no
    /// further calls. The matcher's gate word selects the teardown.
    ///
    /// # Panics
    ///
    /// When the part the path needs is missing: the original faults on
    /// the null pointer there.
    pub fn set_display_mode<W: ClipWorld>(&mut self, world: &mut W, mode: u8) {
        const TEARDOWN_MASK: u32 = 0xffff_0000;
        if self.mode == mode {
            self.mode = mode;
            return;
        }
        if mode == 0 || mode == 2 || mode == 1 {
            let part = self.part.expect("set_display_mode needs the first part");
            let handle = world.fetch_handle(part);
            let matched = world.match_entries(handle);
            if mode == 1 {
                for index in 0..4 {
                    refresh_slot(world, matched.table, index, 0, 0);
                }
                let part2 = self.part2.expect("set_display_mode needs the second part");
                refresh_direct(world, part2, 0, 0x40e0_0000);
                refresh_direct(world, part2, 0, 0x4140_0000);
            } else if mode == 0 {
                for index in 0..4 {
                    refresh_slot(world, matched.table, index, 0, 0);
                }
                world.forward_to_part(part, 0);
            } else {
                refresh_slot(world, matched.table, 0, 0, 0xc140_0000);
                refresh_slot(world, matched.table, 1, 0x4140_0000, 0);
                refresh_slot(world, matched.table, 2, 0, 0);
                refresh_slot(world, matched.table, 3, 0xc140_0000, 0);
            }
            if matched.gate & TEARDOWN_MASK != 0 {
                world.teardown_entries(matched.table);
            }
        }
        self.mode = mode;
    }
}

/// Refreshes one table entry: fetches the service bytes under the
/// selector pair, copies all twenty-four into the record, and releases
/// the service.
fn refresh_slot<W: ClipWorld>(
    world: &mut W,
    table: Handle32<EntryTableTag>,
    index: u32,
    sel0: u32,
    sel1: u32,
) {
    let bytes = world.transform_bytes(sel0, sel1);
    let record = world.entry_record(table, index);
    record.transform.copy_from_slice(&bytes);
    world.release_service();
}

/// Refreshes one direct entry behind a part's transform handle.
fn refresh_direct<W: ClipWorld>(world: &mut W, part: Handle32<PartTag>, sel0: u32, sel1: u32) {
    let handle = world.fetch_handle(part);
    let entry = world.direct_entry(handle);
    let bytes = world.transform_bytes(sel0, sel1);
    let record = world.direct_record(entry);
    record.transform.copy_from_slice(&bytes);
    world.release_service();
}

/// The string including its terminator: `text` cut after the NUL that
/// [`up_to_nul`] found at `len`.
fn text_with_nul(text: &[u8], len: usize) -> &[u8] {
    &text[..len + 1]
}

/// The bytes of a world-returned string excluding the terminator.
///
/// # Panics
///
/// When `text` holds no NUL within 256 bytes: the original copies on
/// past the scratch buffer there.
fn up_to_nul_bounded(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .take(256)
        .position(|b| *b == 0)
        .expect("world string must be NUL-terminated within 256 bytes");
    &text[..end]
}
