// original: 0x00a93f70 stream_accumulate_entry_costs (proposed)

/// Add entry `idx`'s packed cost fields into the four running totals.
///
/// The entry's flag bit 13 gates every block. The first two totals (at
/// `this+0x24` and `this+0x28`) add the low packed field: when the gate bit
/// is clear the raw entry word is added, otherwise
/// `(word & 0x7ff) << (((word >> 11) & 0xf) + 8)`. The third total (at
/// `+0x30`) adds 0 when gated, otherwise
/// `(((word >> 15) & 0x7ff) << (((word >> 26) & 0xf) + 8))`. The fourth
/// total (at `+0x34`) is skipped when gated, otherwise adds the same high
/// field. All adds wrap. Returns the last computed addend (the third
/// block's value when the fourth is gated).
///
/// Original: thiscall, one stack argument (index).
lf_checker_rt::export!(thiscall, rw_00a93f70(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_FLAGS: u32 = 0x0e;
        const GATE_BIT: u32 = 13;
        const MANT_MASK: u32 = 0x7ff;
        const EXP_BIAS: u32 = 8;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        let word = rd32(ent);
        let gated = (rd16(ent.wrapping_add(ENT_FLAGS)) >> GATE_BIT) & 1 == 0;
        let low = if gated {
            word
        } else {
            (word & MANT_MASK).wrapping_shl(((word >> 11) & 0xf) + EXP_BIAS)
        };
        wr32(this.wrapping_add(0x24), rd32(this.wrapping_add(0x24)).wrapping_add(low));
        wr32(this.wrapping_add(0x28), rd32(this.wrapping_add(0x28)).wrapping_add(low));
        let high = if gated {
            0
        } else {
            ((word >> 15) & MANT_MASK)
                .wrapping_shl(((word >> 26) & 0xf) + EXP_BIAS)
        };
        wr32(this.wrapping_add(0x30), rd32(this.wrapping_add(0x30)).wrapping_add(high));
        if gated {
            return high;
        }
        wr32(this.wrapping_add(0x34), rd32(this.wrapping_add(0x34)).wrapping_add(high));
        high
    }
});
