//! The UI font string: one line of text measured and drawn in a font.
//!
//! The original keeps one object per string: live layout words (tags, a
//! style word, size and scale floats, two pushed floats, cached corners,
//! a position pair), six flag bytes, a 256-byte text buffer, and one row
//! descriptor per indexed slot holding a snapshot of those words plus
//! its own text copy. The lifted [`FontString`] owns all of that as
//! ordinary Rust data; everything the string calls (its own virtual
//! slots, the text backend, the handle registry, the shared text cache)
//! is one method of [`FontWorld`], so dispatch is static and tests
//! script answers through a fake.
//!
//! Each verified 32-bit method with behaviour in it is restated as a
//! method on [`FontString`]. Proof is differential: every lifted method
//! runs against its verified rewrite on the same generated inputs,
//! comparing results, every written byte and every world call in order,
//! floats bit for bit (see the `lf-fontstr-diff` test crate). Nothing
//! here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`](super::registry).

/// The UI draw mode that forces style 2 while the string is unstyled.
pub const UI_DRAW_MODE: u8 = 0x6A;

/// The default opaque black the emitter pushes before the row colour.
pub const DEFAULT_COLOUR: u32 = 0xFF00_0000;

/// The tag word [`FontString::reset`] installs.
pub const RESET_TAG: u32 = 7;

/// Bytes per indexed row-text region.
pub const ROW_TEXT_LEN: usize = 256;

/// The string's collaborators: every virtual slot and every intercepted
/// callee the lifted methods reach, as one trait method each.
///
/// Byte strings arrive with their terminator (the original passes
/// pointers to NUL-terminated text). Converted-word arrays are the eight
/// words the converter writes or the cache points at; the submission
/// and height queries take them by reference because the original
/// passes their address.
pub trait FontWorld {
    /// The backend position notify: the four words at the position
    /// pair, snapshotted while the caller's frame is alive.
    fn notify_position(&mut self, snap: [u32; 4]);
    /// The handle registry: two words for a key.
    fn resolve_text(&mut self, key: u32) -> [u32; 2];
    /// The handle registry under a size-selected scratch slot.
    fn resolve_sized(&mut self, size: u32) -> [u32; 2];
    /// The parent refresh slot.
    fn refresh_parent(&mut self);
    /// The text copier: always moves a full buffer; the copy itself is
    /// the world's work, the call and its bytes are what the proof
    /// compares.
    fn copy_text(&mut self, text: &[u8; 256]);
    /// The reset guard slot: true vetoes the whole reset.
    fn guard_veto(&mut self) -> bool;
    /// The primary float sink (slot 37): the reset feeds it the default
    /// float, the measure pass the aspect-scaled height. Answers the
    /// slot's answer.
    fn sink_primary(&mut self, bits: u32) -> u32;
    /// The reset's first notify slot (called with 1).
    fn notify_a(&mut self);
    /// The reset's last notify slot (called with 1).
    fn notify_b(&mut self);
    /// The refresh's first metric hook.
    fn metric_a(&mut self) -> f32;
    /// The refresh's second metric hook.
    fn metric_b(&mut self) -> f32;
    /// The refresh's third metric hook.
    fn metric_c(&mut self) -> f32;
    /// The refresh's fourth metric hook.
    fn metric_d(&mut self) -> f32;
    /// The refresh's render tail slot. Its answer is the refresh's.
    fn render(&mut self) -> u32;
    /// The measure pass's base refresh.
    fn refresh_base(&mut self);
    /// The measure pass's scale push (called with 0 and the bits).
    fn push_scale(&mut self, bits: u32);
    /// The measure pass's visibility hook.
    fn visible(&mut self) -> bool;
    /// The measure pass's advance hook. Its answer is ignored.
    fn advance(&mut self) -> f32;
    /// Pushes the selected style word.
    fn push_style(&mut self, style: u32);
    /// Pushes the position pair.
    fn push_position(&mut self, lo: u32, hi: u32);
    /// Pushes the first live float.
    fn push_a(&mut self, bits: u32);
    /// Pushes a colour word.
    fn push_colour(&mut self, colour: u32);
    /// Pushes the row's colour word.
    fn push_row_word(&mut self, bits: u32);
    /// Pushes the constant one word.
    fn push_one(&mut self);
    /// Pushes the row's width pair.
    fn push_width(&mut self, base: u32, add: u32);
    /// Pushes the second live float.
    fn push_opacity(&mut self, bits: u32);
    /// Converts text into eight words through the scratch path.
    fn convert_text(&mut self, text: &[u8]) -> [u32; 8];
    /// Resolves text into eight words through the shared cache.
    fn resolve_cached(&mut self, text: &[u8]) -> [u32; 8];
    /// Submits a row: corners, converted words, and the two constant
    /// mask words. Answers the submission answer.
    fn submit(&mut self, corner_a: u32, corner_b: u32, words: &[u32; 8]) -> u32;
    /// Queries the height behind converted words.
    fn query_height(&mut self, words: &[u32; 8]) -> f32;
    /// Queries the line height.
    fn line_height(&mut self) -> f32;
    /// Sinks the measured height.
    fn sink_height(&mut self, bits: u32);
    /// Sinks the line height.
    fn sink_line(&mut self, bits: u32);
    /// Sinks the scale-multiplied height.
    fn sink_scaled(&mut self, bits: u32);
    /// Whether the first display extent picks its alternate.
    fn pick_ext_a(&mut self) -> bool;
    /// Whether the second display extent picks its alternate.
    fn pick_ext_b(&mut self) -> bool;
    /// The frame-ending call of the emit and measure passes.
    fn end_frame(&mut self);
}

/// Multiplies in the written order, pinned against commuting so NaN
/// payloads match the original's bit for bit.
#[inline]
fn mul_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Adds in the written order, pinned like [`mul_pinned`].
#[inline]
fn add_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Subtracts in the written order, pinned like [`mul_pinned`].
#[inline]
fn sub_pinned(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
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

/// The string including its terminator: `text` cut after the NUL that
/// [`up_to_nul`] found at `len`.
#[must_use]
pub fn text_with_nul(text: &[u8], len: usize) -> &[u8] {
    &text[..len + 1]
}

/// One indexed row: a snapshot of the live words plus its own text.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StringRow {
    /// The snapped first corner.
    pub corner_a: f32,
    /// The snapped second corner.
    pub corner_b: f32,
    /// The snapped position pair.
    pub pos: [u32; 2],
    /// The width base: the snapped corner, or zero.
    pub width_base: f32,
    /// The width addend: scale plus corner, or one.
    pub width_add: f32,
    /// The snapped style word.
    pub colour: u32,
    /// The snapped tag word.
    pub tag: u32,
    /// The snapped styled byte: nonzero selects the scratch path.
    pub use_scratch: u8,
    /// Whether the row is ready to emit; emitting clears it.
    pub ready: bool,
    /// The row's text copy, terminator included.
    pub text: Vec<u8>,
}

/// The globals the measure pass reads, as one argument.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasureInputs {
    /// The UI mode byte gating the style override.
    pub ui_mode: u8,
    /// The alternate UI mode byte gating the same override.
    pub ui_mode_alt: u8,
    /// The height multiplier.
    pub scale_mul: f32,
    /// The height base.
    pub scale_base: f32,
    /// The height divisor.
    pub scale_div: f32,
    /// The four display extents as raw words.
    pub extents: [u32; 4],
}

/// A UI font string: the owned live words, flags, text and row slots.
///
/// Offsets below are the 32-bit object's, for the reader matching this
/// against the verified rewrites. Floats are stored as `f32` so every
/// bit survives; flag words narrowed to bytes keep the original's low
/// byte.
#[derive(Debug, Clone, PartialEq)]
pub struct FontString {
    /// The second tag word (+0x1D4).
    tag_b: u32,
    /// The mode word (+0x1D8): selects the refresh combination.
    mode: u32,
    /// The style word (+0x1DC).
    style: u32,
    /// The size float (+0x1E0).
    size: f32,
    /// The scale float (+0x1E4).
    scale: f32,
    /// The first pushed float (+0x1E8).
    push_a: f32,
    /// The second pushed float (+0x1EC).
    push_b: f32,
    /// The tag word (+0x1F4).
    tag: u32,
    /// The first cached corner (+0x1F8).
    corner_a: f32,
    /// The second cached corner (+0x1FC).
    corner_b: f32,
    /// The position pair (+0x200/+0x204), or the shared handle words.
    pos: [u32; 2],
    /// The styled byte (+0x208): selects the scratch text path.
    styled: u8,
    /// The flag byte at +0x209. Its role beyond storage is unestablished.
    flag_209: u8,
    /// The flag byte at +0x20A. Its role beyond storage is unestablished.
    flag_20a: u8,
    /// The width-mode byte (+0x20B): selects the snapshot width pair.
    width_mode: u8,
    /// The flag byte at +0x20C. Its role beyond storage is unestablished.
    flag_20c: u8,
    /// The flag byte at +0x20D. Its role beyond storage is unestablished.
    flag_20d: u8,
    /// The live text (+0x20E..+0x30D).
    text: [u8; 256],
    /// The indexed rows.
    rows: Vec<StringRow>,
}

impl Default for FontString {
    fn default() -> Self {
        Self {
            tag_b: 0,
            mode: 0,
            style: 0,
            size: 0.0,
            scale: 0.0,
            push_a: 0.0,
            push_b: 0.0,
            tag: 0,
            corner_a: 0.0,
            corner_b: 0.0,
            pos: [0, 0],
            styled: 0,
            flag_209: 0,
            flag_20a: 0,
            width_mode: 0,
            flag_20c: 0,
            flag_20d: 0,
            text: [0; 256],
            rows: Vec::new(),
        }
    }
}

impl FontString {
    /// A string over the given live words, flags, text and rows.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tag_b: u32,
        mode: u32,
        style: u32,
        size: f32,
        scale: f32,
        push_a: f32,
        push_b: f32,
        tag: u32,
        corner_a: f32,
        corner_b: f32,
        pos: [u32; 2],
        styled: u8,
        flag_209: u8,
        flag_20a: u8,
        width_mode: u8,
        flag_20c: u8,
        flag_20d: u8,
        text: [u8; 256],
        rows: Vec<StringRow>,
    ) -> Self {
        Self {
            tag_b,
            mode,
            style,
            size,
            scale,
            push_a,
            push_b,
            tag,
            corner_a,
            corner_b,
            pos,
            styled,
            flag_209,
            flag_20a,
            width_mode,
            flag_20c,
            flag_20d,
            text,
            rows,
        }
    }

    /// The style word.
    #[must_use]
    pub fn style(&self) -> u32 {
        self.style
    }

    /// The live text buffer.
    #[must_use]
    pub fn text(&self) -> &[u8; 256] {
        &self.text
    }

    /// The indexed rows.
    #[must_use]
    pub fn rows(&self) -> &[StringRow] {
        &self.rows
    }

    /// The position pair.
    #[must_use]
    pub fn pos(&self) -> [u32; 2] {
        self.pos
    }

    /// The mode word.
    #[must_use]
    pub fn mode(&self) -> u32 {
        self.mode
    }

    /// The tag words, `(tag_b, tag)`.
    #[must_use]
    pub fn tags(&self) -> (u32, u32) {
        (self.tag_b, self.tag)
    }

    /// The size float.
    #[must_use]
    pub fn size(&self) -> f32 {
        self.size
    }

    /// The scale float.
    #[must_use]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// The pushed floats, `(push_a, push_b)`.
    #[must_use]
    pub fn pushed(&self) -> (f32, f32) {
        (self.push_a, self.push_b)
    }

    /// The cached corners, `(corner_a, corner_b)`.
    #[must_use]
    pub fn corners(&self) -> (f32, f32) {
        (self.corner_a, self.corner_b)
    }

    /// The six flag bytes in address order.
    #[must_use]
    pub fn flags(&self) -> [u8; 6] {
        [
            self.styled,
            self.flag_209,
            self.flag_20a,
            self.width_mode,
            self.flag_20c,
            self.flag_20d,
        ]
    }

    /// The four words the backend notify snapshots at the position
    /// pair: the pair itself, the four flag bytes packed little-endian,
    /// then the last two flag bytes with the first two text bytes.
    #[must_use]
    fn snapshot_words(&self) -> [u32; 4] {
        let flags =
            u32::from_le_bytes([self.styled, self.flag_209, self.flag_20a, self.width_mode]);
        let tail = u32::from_le_bytes([self.flag_20c, self.flag_20d, self.text[0], self.text[1]]);
        [self.pos[0], self.pos[1], flags, tail]
    }

    /// Copies the style word out (vf133).
    pub fn style_word_into(&self, out: &mut u32) {
        *out = self.style;
    }

    /// Stores the style word, answering what was stored (vf130).
    pub fn set_style(&mut self, value: u32) -> u32 {
        self.style = value;
        value
    }

    /// Stores the first pushed float (vf123).
    pub fn set_push_a(&mut self, value: f32) {
        self.push_a = value;
    }

    /// Stores the size float (vf121).
    pub fn set_size(&mut self, value: f32) {
        self.size = value;
    }

    /// Stores the styled byte (vf127).
    pub fn set_styled(&mut self, value: u8) {
        self.styled = value;
    }

    /// Stores the second pushed float (vf124).
    pub fn set_push_b(&mut self, value: f32) {
        self.push_b = value;
    }

    /// Stores the flag byte at +0x209 (vf128).
    pub fn set_flag_209(&mut self, value: u8) {
        self.flag_209 = value;
    }

    /// Stores the flag byte at +0x20A (vf129).
    pub fn set_flag_20a(&mut self, value: u8) {
        self.flag_20a = value;
    }

    /// Stores the flag byte at +0x20D (vf125).
    pub fn set_flag_20d(&mut self, value: u8) {
        self.flag_20d = value;
    }

    /// Snapshots the live words into row `slot` and copies the live
    /// text after it (vf86). The slot selector global travels as the
    /// `slot` argument.
    ///
    /// # Panics
    ///
    /// When `slot` is past the rows (the original writes wherever the
    /// address lands) or the live text holds no NUL within bounds (the
    /// original copies on past the end).
    pub fn snapshot_row(&mut self, slot: u32) {
        let live = up_to_nul(&self.text).len();
        let row = self
            .rows
            .get_mut(slot as usize)
            .expect("snapshot slot is past the rows");
        row.corner_a = self.corner_a;
        row.corner_b = self.corner_b;
        row.pos = self.pos;
        row.colour = self.style;
        row.tag = self.tag;
        row.use_scratch = self.styled;
        if self.width_mode != 0 {
            row.width_base = self.corner_a;
            row.width_add = add_pinned(self.scale, self.corner_a);
        } else {
            row.width_base = 0.0;
            row.width_add = 1.0;
        }
        row.text = text_with_nul(&self.text, live).to_vec();
        row.ready = true;
    }

    /// Emits row `slot` through the text backend (vf87): nothing when
    /// the row's first text byte or its ready flag is clear (answering
    /// the shifted slot), else the style, position, floats, colours and
    /// width go out, the text resolves through the scratch converter or
    /// the shared cache, and the submission answer comes back. The slot
    /// selector and UI mode globals travel as arguments.
    ///
    /// # Panics
    ///
    /// When `slot` is past the rows, or the row text holds no NUL
    /// within bounds.
    pub fn emit_row<W: FontWorld>(
        &mut self,
        world: &mut W,
        slot: u32,
        ui_mode: u8,
        ui_mode_alt: u8,
    ) -> u32 {
        let row = self
            .rows
            .get(slot as usize)
            .expect("emit slot is past the rows");
        if row.text.first().copied().unwrap_or(0) == 0 {
            world.end_frame();
            return slot << 8;
        }
        if !row.ready {
            world.end_frame();
            return slot << 8;
        }
        let use_scratch = row.use_scratch != 0;
        let (corner_a, corner_b, pos, width_base, width_add, colour, tag) = (
            row.corner_a.to_bits(),
            row.corner_b.to_bits(),
            row.pos,
            row.width_base.to_bits(),
            row.width_add.to_bits(),
            row.colour,
            row.tag,
        );
        let live = up_to_nul(&row.text).len();
        let text = text_with_nul(&row.text, live).to_vec();
        let style = if (ui_mode == UI_DRAW_MODE || ui_mode_alt != 0) && self.styled == 0 {
            2
        } else {
            tag
        };
        world.push_style(style);
        world.push_position(pos[0], pos[1]);
        world.push_a(self.push_a.to_bits());
        world.push_colour(DEFAULT_COLOUR);
        world.push_row_word(colour);
        world.push_one();
        world.push_width(width_base, width_add);
        world.push_opacity(self.push_b.to_bits());
        let words = if use_scratch {
            world.convert_text(&text)
        } else {
            world.resolve_cached(&text)
        };
        let answer = world.submit(corner_a, corner_b, &words);
        self.rows[slot as usize].ready = false;
        world.end_frame();
        answer
    }

    /// Stores the position pair, notifying the backend unless `hold` is
    /// set (vf119). The flag word narrows to the hold bit.
    pub fn set_position<W: FontWorld>(&mut self, world: &mut W, x: f32, y: f32, hold: bool) {
        self.pos = [x.to_bits(), y.to_bits()];
        if !hold {
            world.notify_position(self.snapshot_words());
        }
    }

    /// Resolves the shared handle for `key` and notifies the backend
    /// (vf117). The zeroed scratch snapshot is constant on both sides.
    pub fn resolve_handle<W: FontWorld>(&mut self, world: &mut W, key: u32) {
        self.pos = world.resolve_text(key);
        world.notify_position(self.snapshot_words());
    }

    /// Selects a registry slot by kind, resolves the shared handle
    /// through it, and notifies the backend (vf118). Kind 0 resolves
    /// with size 2, kind 2 and every other kind with size 8.
    pub fn select_handle<W: FontWorld>(&mut self, world: &mut W, kind: u32) {
        let size = if kind == 0 {
            2
        } else if kind == 2 {
            8
        } else {
            8
        };
        self.pos = world.resolve_sized(size);
        world.notify_position(self.snapshot_words());
    }

    /// Refreshes through the parent slot when `refresh` is set, and when
    /// `text` is present hands a full buffer to the copier and forces
    /// the terminator (vf120). The flag word narrows to the refresh
    /// bit; the text pointer narrows to the buffer or nothing.
    pub fn set_text<W: FontWorld>(
        &mut self,
        world: &mut W,
        text: Option<&[u8; 256]>,
        refresh: bool,
    ) {
        if refresh {
            world.refresh_parent();
        }
        if let Some(buffer) = text {
            world.copy_text(buffer);
            self.text[255] = 0;
        }
    }

    /// Resets the string state from the given metrics unless the guard
    /// vetoes (vf115). The source pointer narrows to its word; the
    /// default-float global travels as `default`.
    pub fn reset<W: FontWorld>(
        &mut self,
        world: &mut W,
        size: f32,
        src_word: u32,
        tag_a: u32,
        tag_b: u32,
        default: f32,
    ) {
        if world.guard_veto() {
            return;
        }
        self.size = size;
        world.sink_primary(default.to_bits());
        self.mode = tag_a;
        self.tag_b = tag_b;
        self.tag = RESET_TAG;
        self.style = src_word;
        self.scale = 1.0;
        self.styled = 0;
        self.flag_209 = 0;
        self.flag_20a = 0;
        self.width_mode = 0;
        self.flag_20c = 0;
        self.flag_20d = 0;
        self.push_a = f32::from_bits(0);
        self.push_b = 1.0;
        world.notify_a();
        world.notify_position(self.snapshot_words());
        world.notify_b();
    }

    /// Refreshes the cached corners from the four metric hooks and
    /// renders (vf85). The mode word selects the first corner: mode 0
    /// takes the first metric minus the scaled third, mode 1 the scaled
    /// third plus the first, any other mode the first metric alone; the
    /// second corner always takes the second metric minus the scaled
    /// fourth. The scale global travels as `scale`. Answers the
    /// render slot's answer.
    pub fn refresh<W: FontWorld>(&mut self, world: &mut W, scale: f32) -> u32 {
        let m1 = world.metric_a();
        let m2 = world.metric_b();
        let m3 = world.metric_c();
        let m4 = world.metric_d();
        let first = if self.mode == 0 {
            sub_pinned(m1, mul_pinned(m3, scale))
        } else if self.mode == 1 {
            add_pinned(mul_pinned(m3, scale), m1)
        } else {
            m1
        };
        self.corner_a = first;
        self.corner_b = sub_pinned(m2, mul_pinned(m4, scale));
        world.render()
    }

    /// Recomputes the layout metrics and sinks them into their slots
    /// (vf83): refreshes the base, pushes the scale pair when the
    /// width mode is set, re-polls visibility and the advance sink,
    /// forwards the live words to the backend, resolves the text
    /// height, sinks the height, line and scaled triples, and sinks
    /// the aspect-scaled height last. Answers the last sink's answer.
    pub fn measure<W: FontWorld>(&mut self, world: &mut W, inputs: &MeasureInputs) -> u32 {
        world.refresh_base();
        if self.width_mode != 0 {
            world.push_scale(self.scale.to_bits());
        }
        if world.visible() {
            world.refresh_base();
            let _ = world.advance();
        }
        let style =
            if (inputs.ui_mode == UI_DRAW_MODE || inputs.ui_mode_alt != 0) && self.styled == 0 {
                2
            } else {
                self.tag
            };
        world.push_style(style);
        world.push_position(self.pos[0], self.pos[1]);
        world.push_a(self.push_a.to_bits());
        world.push_opacity(self.push_b.to_bits());
        let live = up_to_nul(&self.text).len();
        let text = text_with_nul(&self.text, live).to_vec();
        let height = if self.styled == 0 {
            let words = world.resolve_cached(&text);
            world.query_height(&words)
        } else {
            let words = world.convert_text(&text);
            world.query_height(&words)
        };
        let line = world.line_height();
        world.sink_height(height.to_bits());
        world.sink_line(line.to_bits());
        world.sink_scaled(mul_pinned(inputs.scale_mul, height).to_bits());
        let ext_a = inputs.extents[if world.pick_ext_a() { 1 } else { 0 }];
        let ext_b = inputs.extents[if world.pick_ext_b() { 3 } else { 2 }];
        let aspect = (ext_a as i32 as f32) / (ext_b as i32 as f32);
        let scaled = mul_pinned(inputs.scale_base, height) / (inputs.scale_div / aspect);
        let answer = world.sink_primary(scaled.to_bits());
        world.end_frame();
        answer
    }
}
