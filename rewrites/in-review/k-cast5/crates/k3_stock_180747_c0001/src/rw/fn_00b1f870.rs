// original: 0x00b1f870 fill_level_rows (proposed)

/// Fills four level-row blocks from two clamped, scaled fields.
///
/// Thiscall of one stack word: a block index (callee pops 4 bytes). The
/// fields at +0x38 and +0x3C are each scaled by 256 and clamped into
/// [-32768, 32767] (a NaN survives unclamped); both are truncated to
/// integers and packed as two halfwords. Four consecutive 0x600-byte
/// blocks starting at block*4 (floored division by 8 of index*32, which
/// always spans exactly four blocks short of 32-bit overflow) are then
/// filled, 32 rows each: eight copies of the packed word and eight zero
/// halfwords interleaved per row. Returns the address one past the last
/// block, or the second truncated value when the range is empty.
lf_checker_rt::export!(thiscall, rw_00b1f870(this: u32, index: u32) -> u32 {
    unsafe {
        const SCALE_ADDR: u32 = 0x00fe8c0c;
        const LO_ADDR: u32 = 0x00eabdc0;
        const HI_ADDR: u32 = 0x00e758fc;
        const BLOCK_BASE: u32 = 0xc256;
        const BLOCK_STRIDE: u32 = 0x600;
        const ROW_STRIDE: u32 = 0x30;
        const ROWS: u32 = 32;
        let scale = (lf_checker_rt::relocated(SCALE_ADDR) as *const f32).read_unaligned();
        let lo = (lf_checker_rt::relocated(LO_ADDR) as *const f32).read_unaligned();
        let hi = (lf_checker_rt::relocated(HI_ADDR) as *const f32).read_unaligned();
        let span = (index.wrapping_mul(32) as i32) >> 3;
        let end = ((index.wrapping_mul(32).wrapping_add(0x20)) as i32) >> 3;
        let mut v1 = core::hint::black_box(
            ((this + 0x38) as *const f32).read_unaligned(),
        ) * core::hint::black_box(scale);
        if lo > v1 {
            v1 = lo;
        } else if v1 > hi {
            v1 = hi;
        }
        let w1 = v1 as i32 as u16;
        let v2raw = core::hint::black_box(
            ((this + 0x3c) as *const f32).read_unaligned(),
        ) * core::hint::black_box(scale);
        let v2 = if lo > v2raw {
            lo
        } else if !(v2raw > hi) {
            v2raw
        } else {
            hi
        };
        // Exact cvttss2si: Rust saturates, the instruction yields 0x80000000
        // for NaN (overflow is impossible here: v2 is clamped or NaN).
        let w2i = if v2.is_nan() { i32::MIN } else { v2 as i32 };
        let packed = (w1 as u32) | ((w2i as u16 as u32) << 16);
        if span >= end {
            return w2i as u32;
        }
        let mut k = span;
        let mut past = 0u32;
        while k < end {
            let mut row = this
                .wrapping_add(BLOCK_BASE)
                .wrapping_add((k as u32).wrapping_mul(BLOCK_STRIDE));
            let mut r = 0u32;
            while r < ROWS {
                for off in [-6i32, 0, 6, 0x0c, 0x12, 0x18, 0x1e, 0x24] {
                    ((row.wrapping_add(off as u32)) as *mut u32).write_unaligned(packed);
                }
                for off in [-2i32, 4, 0x0a, 0x10, 0x16, 0x1c, 0x22, 0x28] {
                    ((row.wrapping_add(off as u32)) as *mut u16).write_unaligned(0);
                }
                row = row.wrapping_add(ROW_STRIDE);
                r += 1;
            }
            past = row;
            k += 1;
        }
        past
    }
});
