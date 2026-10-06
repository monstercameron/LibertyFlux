//! The task parameter block and its kind initialisers.
//!
//! A parameter block is the plain record one of the kind initialisers
//! fills: no vtable, no pointers, 64 bytes observed. The base and main
//! collaborators stamp the kind byte and the low words (their domain,
//! behind [`TaskInit`]); each initialiser then stores its kind's
//! arguments and flag packing and writes its tag byte. The tag byte at
//! offset 2 is the one field every initialiser writes.
//!
//! Offsets below come from the verified rewrites' read/write sets: kind
//! at +0, tag at +2, flag bits at +3, the mode byte at +0x14, word slots
//! at +0x18..+0x28, and one trailing byte at +0x3c. Slots mean different
//! things per kind, so they travel as bytes and words, not typed fields.

/// Length of the observed parameter block in bytes.
///
/// The highest offset any kind initialiser touches is the trailing byte
/// at +0x3c, so 64 bytes cover every lifted method; the collaborators'
/// writes below +0x18 are their own domain.
pub const PARAM_LEN: usize = 0x40;

/// Offset of the kind byte.
const OFF_KIND: usize = 0x00;
/// Offset of the tag byte every initialiser writes.
const OFF_TAG: usize = 0x02;
/// Offset of the shared flag byte.
const OFF_FLAGS: usize = 0x03;
/// Offset of the mode byte.
const OFF_MODE: usize = 0x14;
/// Offset of the first word slot.
const OFF_W0: usize = 0x18;
/// Offset of the second word slot.
const OFF_W1: usize = 0x1c;
/// Offset of the byte past the second word slot's first byte.
const OFF_B1D: usize = 0x1d;
/// Offset of the byte past the second word slot's second byte.
const OFF_B1E: usize = 0x1e;
/// Offset of the third word slot.
const OFF_W2: usize = 0x20;
/// Offset of the flag byte inside the third word slot's top half.
const OFF_F22: usize = 0x22;
/// Offset of the fourth word slot.
const OFF_W3: usize = 0x24;
/// Offset of the fifth word slot.
const OFF_W4: usize = 0x28;
/// Offset of the flag byte past the fifth word slot.
const OFF_F2C: usize = 0x2c;
/// Offset of the second byte of the third word slot.
const OFF_B21: usize = 0x21;
/// Offset of the fourth byte of the third word slot.
const OFF_B23: usize = 0x23;
/// Offset of the trailing packed byte (kind `0x2f` only).
const OFF_TRAIL: usize = 0x3c;

/// The collaborators every kind initialiser builds on.
///
/// The base and main routines run first (base stamps the kind, main the
/// low words); the tail calls run one kind's block or quantiser step.
/// Tail slots that target one routine across kinds share one method,
/// the rest travel one per slot (verified from the verifying
/// lane's call map: no two tail slots share a target), so each travels
/// as its own method. Returns are unmodelled: every rewrite discards
/// them.
pub trait TaskInit {
    /// Run the base initialiser for `kind`.
    fn base(&mut self, kind: u32);
    /// Run the main initialiser: `kind`, the shared word `g`, the two
    /// header arguments and the header flag (1 on the base+main path).
    fn main(&mut self, kind: u32, g: u32, a1: u32, a0: u32, flag: u32);
    /// Run the kind-`0x2f` block step on `arg`.
    fn block_2f(&mut self, arg: u32);
    /// Run the kind-`0x40` word-quantiser step on `arg`.
    fn quant_word(&mut self, arg: u32);
    /// Run the first tail call of the kind-`0x43` chain on `arg`.
    fn chain_43_first(&mut self, arg: u32);
    /// Run the second tail call of the kind-`0x43` chain on `arg`.
    fn chain_43_second(&mut self, arg: u32);
    /// Run the shared block-copy step on `arg` (kinds 0x35/0x33/0x41/0x46).
    fn copy_block(&mut self, arg: u32);
    /// Run the kind-`0x33` vector-quantiser step on `arg`.
    fn quant_vec(&mut self, arg: u32);
    /// Run the kind-`0x41` byte-quantiser step on `arg`.
    fn quant_byte(&mut self, arg: u32);
    /// Run the first step of the shared tail pair on `arg` (kinds 0x44/0x45).
    fn pair_first(&mut self, arg: u32);
    /// Run the second step of the shared tail pair on `arg` (kinds 0x44/0x45).
    fn pair_second(&mut self, arg: u32);
    /// Run the kind-`0x42`/`0x45` block step on `arg`.
    fn block_aux(&mut self, arg: u32);
    /// Run the kind-`0x3e` sub-initialiser (kinds 0x3a/0x3b/0x3c chain it).
    fn sub_3e(&mut self, a0: u32, a1: u32, a_last: u32);
    /// Run the kind-`0x3a` block step on `arg`.
    fn block_3a(&mut self, arg: u32);
}

/// Truncate a float toward zero, as the 32-bit truncation instruction:
/// NaN and out-of-range magnitudes answer `i32::MIN`.
// The truncation is exact: the guards exclude every input the `as`
// conversion would saturate, leaving only values that truncate.
#[allow(clippy::cast_possible_truncation)]
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2_147_483_648.0 || x <= -2_147_483_648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// A task parameter block: the 64 bytes the initialisers fill.
///
/// Built from the caller's bytes ([`TaskParams::from_bytes`]) because
/// several initialisers fold pre-existing flag bits into their stores;
/// [`TaskParams::blank`] is the zeroed convenience form for fresh blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskParams {
    raw: [u8; PARAM_LEN],
}

impl TaskParams {
    /// A zeroed block.
    #[must_use]
    pub fn blank() -> Self {
        Self {
            raw: [0; PARAM_LEN],
        }
    }

    /// A block holding the caller's bytes.
    #[must_use]
    pub fn from_bytes(bytes: [u8; PARAM_LEN]) -> Self {
        Self { raw: bytes }
    }

    /// The block's bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8; PARAM_LEN] {
        &self.raw
    }

    /// The kind byte at +0 (stamped by the collaborators, or by the
    /// `0x3e`-derived initialisers once lifted).
    #[must_use]
    pub fn kind(&self) -> u8 {
        self.raw[OFF_KIND]
    }

    /// The tag byte at +2.
    #[must_use]
    pub fn tag(&self) -> u8 {
        self.raw[OFF_TAG]
    }

    /// The mode byte at +0x14.
    #[must_use]
    pub fn mode(&self) -> u8 {
        self.raw[OFF_MODE]
    }

    fn rd8(&self, off: usize) -> u8 {
        self.raw[off]
    }

    fn wr8(&mut self, off: usize, v: u8) {
        self.raw[off] = v;
    }

    fn wr32(&mut self, off: usize, v: u32) {
        self.raw[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// Initialise a block of kind `0x3e` and store one word argument.
    ///
    /// Base, main, the mode byte 1, `a2` at +0x18, tag 4.
    pub fn init_3e<I: TaskInit>(&mut self, init: &mut I, g: u32, a0: u32, a1: u32, a2: u32) {
        const KIND: u32 = 0x3e;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        self.wr8(OFF_MODE, 1);
        self.wr32(OFF_W0, a2);
        self.wr8(OFF_TAG, 4);
    }

    /// Initialise a block of kind `0x36`: one word and four bytes.
    ///
    /// Base, main, `a5`/`a2`/`a4` at +0x1c/+0x1d/+0x1e, `a3` at +0x18,
    /// `a4 + 11` (wrapping) at the mode byte, tag 1.
    #[allow(clippy::too_many_arguments)]
    pub fn init_36<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u8,
        a3: u32,
        a4: u8,
        a5: u8,
    ) {
        const KIND: u32 = 0x36;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        self.wr8(OFF_W1, a5);
        self.wr32(OFF_W0, a3);
        self.wr8(OFF_B1D, a2);
        self.wr8(OFF_B1E, a4);
        self.wr8(OFF_MODE, a4.wrapping_add(11));
        self.wr8(OFF_TAG, 1);
    }

    /// Initialise a block of kind `0x34`: two words and a flag fold.
    ///
    /// Base, main, `a3`/`a4` at +0x18/+0x1c, mode 4, tag 1. The flag
    /// fold xors bits 1..3 of the byte at +0x24 with twice (`a2 - 1`).
    #[allow(clippy::too_many_arguments)]
    pub fn init_34<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u8,
        a3: u32,
        a4: u32,
    ) {
        const KIND: u32 = 0x34;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        let old = self.rd8(OFF_W3);
        let t = a2.wrapping_sub(1).wrapping_mul(2) ^ old;
        self.wr32(OFF_W0, a3);
        self.wr32(OFF_W1, a4);
        self.wr8(OFF_W3, old ^ (t & 0x0e));
        self.wr8(OFF_MODE, 4);
        self.wr8(OFF_TAG, 1);
    }

    /// Initialise a block of kind `0x3f` and toggle one flag bit.
    ///
    /// Base, main, tag 9, then bit 1 of the flag byte follows bit 0 of
    /// `a2` (xor through twice the argument, masked to bit 1).
    pub fn init_3f<I: TaskInit>(&mut self, init: &mut I, g: u32, a0: u32, a1: u32, a2: u8) {
        const KIND: u32 = 0x3f;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        let old = self.rd8(OFF_FLAGS);
        let t = a2.wrapping_mul(2) ^ old;
        self.wr8(OFF_TAG, 9);
        self.wr8(OFF_FLAGS, old ^ (t & 2));
    }

    /// Initialise a block of kind `0x40`: one word and a flag toggle.
    ///
    /// Base, main, bit 0 of the byte at +0x22 follows `a4`, `a2` at
    /// +0x18, the word-quantiser step on `a3`, tag 8. The stores run
    /// before the quantiser step, as in the rewrite.
    #[allow(clippy::too_many_arguments)]
    pub fn init_40<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u8,
    ) {
        const KIND: u32 = 0x40;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        let old = self.rd8(OFF_F22);
        let t = old ^ a4;
        self.wr8(OFF_F22, old ^ (t & 1));
        self.wr32(OFF_W0, a2);
        init.quant_word(a3);
        self.wr8(OFF_TAG, 8);
    }

    /// Initialise a block of kind `0x2f`: five words, two packed bytes.
    ///
    /// Base, main, the block step on `a2`, `a4`..`a8` at +0x18..+0x28.
    /// The packed byte collects one bit of `a11`, all of `a12`, two of
    /// `a9`, one of `a10` and three of `a3`, low to high; the mode byte
    /// keeps its low three bits and gains 2 when bit 3 is set. Tag 2.
    /// All packing arithmetic wraps at 8 bits, as in the rewrite.
    #[allow(clippy::too_many_arguments)]
    pub fn init_2f<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u8,
        a10: u8,
        a11: u8,
        a12: u8,
    ) {
        const KIND: u32 = 0x2f;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        init.block_2f(a2);
        let mut packed = (a11 & 1) | a12.wrapping_mul(2);
        packed = (packed << 2) | (a9 & 3);
        packed = packed.wrapping_add(packed) | (a10 & 1);
        self.wr32(OFF_W0, a4);
        self.wr32(OFF_W1, a5);
        self.wr32(OFF_W2, a6);
        self.wr32(OFF_W3, a7);
        self.wr32(OFF_W4, a8);
        packed = (packed << 3) | (a3 & 7);
        self.wr8(OFF_TRAIL, packed);
        let mut mode = packed & 7;
        self.wr8(OFF_TAG, 2);
        if packed & 8 != 0 {
            mode = mode.wrapping_add(2);
        }
        self.wr8(OFF_MODE, mode);
    }

    /// Initialise a block of kind `0x43` through its four-call chain.
    ///
    /// Base, main, the chain's two tail calls on `a2`/`a3`, tag 7. The
    /// tail calls are opaque: each targets its own routine.
    pub fn init_43<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
    ) {
        const KIND: u32 = 0x43;
        init.base(KIND);
        init.main(KIND, g, a1, a0, 1);
        init.chain_43_first(a2);
        init.chain_43_second(a3);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x3d` with the main initialiser only.
    ///
    /// Main with header flag 0, tag 6.
    pub fn init_3d<I: TaskInit>(&mut self, init: &mut I, g: u32, a0: u32, a1: u32) {
        const KIND: u32 = 0x3d;
        init.main(KIND, g, a1, a0, 0);
        self.wr8(OFF_TAG, 6);
    }

    /// Initialise a block of kind `0x42`: main, one block step, tag 7.
    pub fn init_42<I: TaskInit>(&mut self, init: &mut I, g: u32, a0: u32, a1: u32, a2: u32) {
        const KIND: u32 = 0x42;
        init.main(KIND, g, a1, a0, 0);
        init.block_aux(a2);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x46`: main, block copy, one word.
    ///
    /// Main with header flag 0, the block copy on `a2`, `a3` at +0x1c,
    /// tag 7.
    pub fn init_46<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
    ) {
        const KIND: u32 = 0x46;
        init.main(KIND, g, a1, a0, 0);
        init.copy_block(a2);
        self.wr32(OFF_W1, a3);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x35`: block copy and packed flags.
    ///
    /// Main with header flag 0, the block copy on `a2`, then the flag
    /// byte collects one bit each of `a6`/`a5`/`a4`, three bits of
    /// `a3 - 1`, and bits 7 and 0 of the old flag byte, low to high.
    /// Tag 1. All packing arithmetic wraps at 8 bits.
    #[allow(clippy::too_many_arguments)]
    pub fn init_35<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    ) {
        const KIND: u32 = 0x35;
        init.main(KIND, g, a1, a0, 0);
        init.copy_block(a2);
        let mut cl = a6 & 1;
        cl = cl.wrapping_add(cl) | (a5 & 1);
        cl = cl.wrapping_add(cl) | (a4 & 1);
        cl = (cl << 3) | (a3.wrapping_sub(1) & 7);
        cl = cl.wrapping_add(cl) | (self.rd8(OFF_W1) & 0x81);
        self.wr8(OFF_W1, cl);
        self.wr8(OFF_TAG, 1);
    }

    /// Initialise a block of kind `0x33`: block copy, quantiser, flag fold.
    ///
    /// Main with header flag 0, the block copy on `a2`, the
    /// vector-quantiser step on `a3`, tag 1, then bits 1..3 of the byte
    /// at +0x1c xor with twice (`a4 - 1`).
    #[allow(clippy::too_many_arguments)]
    pub fn init_33<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u8,
    ) {
        const KIND: u32 = 0x33;
        init.main(KIND, g, a1, a0, 0);
        init.copy_block(a2);
        init.quant_vec(a3);
        let old = self.rd8(OFF_W1);
        let t = a4.wrapping_sub(1).wrapping_mul(2) ^ old;
        self.wr8(OFF_TAG, 1);
        self.wr8(OFF_W1, old ^ (t & 0x0e));
    }

    /// Initialise a block of kind `0x41`: block copy, quantiser, flags.
    ///
    /// Main with header flag 0, the block copy on `a2`, the
    /// byte-quantiser step on `a3`. The flag byte collects two bits of
    /// `a8`, one each of `a4`/`a7`, bits 5..7 of the old byte at +0x24,
    /// and one of `a6`, low to high; `a5` lands at +0x20. Tag 7.
    #[allow(clippy::too_many_arguments)]
    pub fn init_41<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u8,
        a5: u32,
        a6: u8,
        a7: u8,
        a8: u8,
    ) {
        const KIND: u32 = 0x41;
        init.main(KIND, g, a1, a0, 0);
        init.copy_block(a2);
        init.quant_byte(a3);
        let mut cl = a8 & 3;
        cl = cl.wrapping_add(cl) | (a4 & 1);
        cl = cl.wrapping_add(cl) | (a7 & 1);
        cl = cl.wrapping_add(cl) | (self.rd8(OFF_W3) & 0xe0);
        cl |= a6 & 1;
        self.wr32(OFF_W2, a5);
        self.wr8(OFF_W3, cl);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x44`: tail pair and a truncated float.
    ///
    /// Main with header flag 0, the tail pair on `a2`/`a3`, then `a4`
    /// truncates toward zero (see [`cvtt`]) and its low byte, doubled,
    /// plus bit 0 of `a5` lands at +0x20. Tag 7.
    #[allow(clippy::too_many_arguments)]
    pub fn init_44<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u8,
    ) {
        const KIND: u32 = 0x44;
        init.main(KIND, g, a1, a0, 0);
        init.pair_first(a2);
        init.pair_second(a3);
        let c = cvtt(f32::from_bits(a4));
        // The low byte of the truncated value, sign discarded as in the rewrite.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let cl = (c as u8).wrapping_mul(2) | (a5 & 1);
        self.wr8(OFF_W2, cl);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x45`: branched tails, two flag folds.
    ///
    /// Main with header flag 0, then a nonzero `a1` runs the block step
    /// on `a2`, else the tail pair on `a3`/`a4`. `a9` lands at +0x20;
    /// the byte at +0x28 keeps bits 0..1 and 4..7 when `a7 - 1` exceeds
    /// 2, else bits 2..3 follow `a7`; the low two bits then pack one bit
    /// each of `a6`/`a5`. `a8` lands at +0x24. Tag 7.
    #[allow(clippy::too_many_arguments)]
    pub fn init_45<I: TaskInit>(
        &mut self,
        init: &mut I,
        g: u32,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u32,
        a9: u32,
    ) {
        const KIND: u32 = 0x45;
        init.main(KIND, g, a1, a0, 0);
        if a1 != 0 {
            init.block_aux(a2);
        } else {
            init.pair_first(a3);
            init.pair_second(a4);
        }
        self.wr32(OFF_W2, a9);
        if a7.wrapping_sub(1) > 2 {
            self.wr8(OFF_W4, self.rd8(OFF_W4) & 0xf3);
        } else {
            let old = self.rd8(OFF_W4);
            let t = (a7 << 2) ^ old;
            self.wr8(OFF_W4, old ^ (t & 0x0c));
        }
        let cl = (self.rd8(OFF_W4) & 0xfc) | (((a6 & 1) << 1) | (a5 & 1));
        self.wr8(OFF_W4, cl);
        self.wr32(OFF_W3, a8);
        self.wr8(OFF_TAG, 7);
    }

    /// Initialise a block of kind `0x3a` through the `0x3e` sub-initialiser.
    ///
    /// The sub-initialiser on (`a0`, `a1`, `a3`), the kind byte, the
    /// block step on `a2`, then the flag byte collects one bit of `a6`,
    /// one of `a5`, bits 3..7 of the old byte at +0x2c, and one of `a4`,
    /// low to high. Mode 2, tag 4.
    #[allow(clippy::too_many_arguments)]
    pub fn init_3a<I: TaskInit>(
        &mut self,
        init: &mut I,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u8,
        a5: u8,
        a6: u8,
    ) {
        init.sub_3e(a0, a1, a3);
        self.wr8(OFF_KIND, 0x3a);
        init.block_3a(a2);
        let mut cl = (a6 & 1).wrapping_mul(2) | (a5 & 1);
        cl = cl.wrapping_mul(2) | (self.rd8(OFF_F2C) & 0xf8);
        cl |= a4 & 1;
        self.wr8(OFF_MODE, 2);
        self.wr8(OFF_F2C, cl);
        self.wr8(OFF_TAG, 4);
    }

    /// Initialise a block of kind `0x3b`: sub-initialiser and const compare.
    ///
    /// The sub-initialiser on (`a0`, `a1`, `a3`), `a2 + 2` at the mode
    /// byte, `a5`/`a6` at +0x20/+0x21, bit 0 of +0x23 following `a7`,
    /// the kind byte, `a4` at +0x1c, `a2` at +0x22, tag 4. When the two
    /// shared float constants compare equal (ordered: NaN never sets
    /// it), bit 1 of the flag byte sets.
    #[allow(clippy::too_many_arguments)]
    pub fn init_3b<I: TaskInit>(
        &mut self,
        init: &mut I,
        a0: u32,
        a1: u32,
        a2: u8,
        a3: u32,
        a4: u32,
        a5: u8,
        a6: u8,
        a7: u8,
        f1_bits: u32,
        f2_bits: u32,
    ) {
        init.sub_3e(a0, a1, a3);
        self.wr8(OFF_MODE, a2.wrapping_add(2));
        self.wr8(OFF_W2, a5);
        self.wr8(OFF_B21, a6);
        let old = self.rd8(OFF_B23);
        let t = old ^ a7;
        self.wr8(OFF_KIND, 0x3b);
        self.wr8(OFF_B23, old ^ (t & 1));
        self.wr32(OFF_W1, a4);
        self.wr8(OFF_F22, a2);
        let f1 = f32::from_bits(f1_bits);
        let f2 = f32::from_bits(f2_bits);
        self.wr8(OFF_TAG, 4);
        // Ordered equality, exactly as the rewrite: equal magnitudes
        // (including both zeros) set the bit, NaN never does.
        #[allow(clippy::float_cmp)]
        if f1 == f2 {
            self.wr8(OFF_FLAGS, self.rd8(OFF_FLAGS) | 2);
        }
    }

    /// Initialise a block of kind `0x3c`: sub-initialiser, three-word copy.
    ///
    /// The sub-initialiser on (`a0`, `a1`, `a2`), the kind byte, the
    /// three words at `src` to +0x1c/+0x20/+0x24, mode 0, tag 5.
    pub fn init_3c<I: TaskInit>(
        &mut self,
        init: &mut I,
        a0: u32,
        a1: u32,
        a2: u32,
        src: &[u32; 3],
    ) {
        init.sub_3e(a0, a1, a2);
        self.wr8(OFF_KIND, 0x3c);
        self.wr32(OFF_W1, src[0]);
        self.wr32(OFF_W2, src[1]);
        self.wr32(OFF_W3, src[2]);
        self.wr8(OFF_MODE, 0);
        self.wr8(OFF_TAG, 5);
    }
}
