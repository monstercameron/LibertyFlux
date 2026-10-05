// original: 0x0089BA10 aud_slot_pair_update (proposed)

/// Run the start hook and then the pair-update helper twice, once per slot.
///
/// `this` is an audio sound object. Bytes at `+0x40` (category) and
/// `+0x48`/`+0x49` (slot indexes) select rows of the global slot table:
/// when the slot byte is `0xFF` there is no row (null is passed), otherwise
/// the row is `stride * slot + row_base[category]`, where `stride` is the
/// global at `G_SLOT_STRIDE` and `row_base` the table at `G_TABLE_BASE`
/// (each category row is `CAT_STRIDE` bytes, the base pointer lives at
/// `+TABLE_ROW` within the row).
///
/// Behaviour: call the start hook (callee 0) with the `+0x48` row, the
/// dword at `+0x54` and 0 (its answer is ignored); then call the pair
/// helper (callee 1) twice, with the `+0x48` row and then the `+0x49` row,
/// each time with the two incoming stack arguments and a middle word built
/// from bit 5 of the byte at `+0x39` planted in the low byte of `this`
/// (the original writes that bit over its own saved `this` slot and
/// re-pushes the whole dword). Returns 2 if either helper answer is 2,
/// else 0 if either is 0, else 1. All comparisons are equalities
/// (`cmp`/`je`, `test`/`je`), so signedness does not apply.
///
/// Original: 0x0089BA10 (thiscall, two stack words, returns full `eax`).
lf_checker_rt::export!(thiscall, rw_0089BA10(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const FLAG_BYTE: u32 = 0x39;
        const SLOT_A: u32 = 0x48;
        const SLOT_B: u32 = 0x49;
        const START_PARAM: u32 = 0x54;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_ROW: u32 = 0x6F10;
        const NO_SLOT: u8 = 0xFF;
        const G_SLOT_STRIDE: u32 = 0x115D964;
        const G_TABLE_BASE: u32 = 0x115D988;
        const CALLEE_START: u32 = 0;
        const CALLEE_PAIR: u32 = 1;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Row pointer for the slot byte at `this + slot_off`, or null.
        unsafe fn slot_row(this: u32, slot_off: u32) -> u32 {
            unsafe {
                let slot = rd8(this + slot_off);
                if slot == NO_SLOT {
                    return 0;
                }
                let stride = (lf_checker_rt::global::<u32>(G_SLOT_STRIDE)).read_unaligned();
                let base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
                let cat = rd8(this + CAT_INDEX) as u32;
                let row = rd32(base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW));
                stride.wrapping_mul(slot as u32).wrapping_add(row)
            }
        }
        /// Middle helper word: bit 5 of the flag byte over the low byte of `this`.
        unsafe fn middle_word(this: u32) -> u32 {
            unsafe { (this & 0xFFFF_FF00) | (((rd8(this + FLAG_BYTE) >> 5) & 1) as u32) }
        }

        let start_row = slot_row(this, SLOT_A);
        lf_checker_rt::callee_thiscall!(CALLEE_START, u32, start_row, rd32(this + START_PARAM), 0);
        let first: u32 =
            lf_checker_rt::callee_thiscall!(CALLEE_PAIR, u32, slot_row(this, SLOT_A), arg1, middle_word(this), arg2);
        let second: u32 =
            lf_checker_rt::callee_thiscall!(CALLEE_PAIR, u32, slot_row(this, SLOT_B), arg1, middle_word(this), arg2);
        if first == 2 || second == 2 {
            2
        } else if first == 0 || second == 0 {
            0
        } else {
            1
        }
    }
});
