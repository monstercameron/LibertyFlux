// original: 0x00a952d0 stream_release_entry_cost_a (proposed)

/// Subtract a streaming entry's decoded costs from accumulator set A.
///
/// The entry is `table[idx*3*8]` where `table` is the pointer at `this+0x00`.
/// Each entry word at `+0x00` packs two variable-length cost fields; bit 13
/// of the flag word at `+0x0e` selects whether the packed form is decoded
/// (exponent in bits 11..14 or 26..29 plus 8, 11-bit mantissa) or the raw
/// word is used. The first cost is subtracted from `this+0x24`. The second
/// cost is subtracted from `this+0x30` only when the flag is set; when it is
/// clear no store happens and the return is the high half of the first cost
/// above the shifted flag word (the original only reloads `ax` there).
///
/// Returns the second subtracted value, or the mixed leftover described
/// above. Thiscall: object in ecx, one stack
/// word (the index), callee pops 4. Twin of 0x00a95330, which uses set B
/// (`+0x28` / `+0x34`).
lf_checker_rt::export!(thiscall, rw_00a952d0(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const ACC0: u32 = 0x24;
        const ACC1: u32 = 0x30;
        const ENTRY_WORD: u32 = 0x00;
        const FLAGS_OFF: u32 = 0x0e;
        const PACKED_BIT: u16 = 13;
        const MANT_MASK: u32 = 0x7ff;
        const EXP_BIAS: u32 = 8;
        let entry = ((this + TABLE) as *const u32).read_unaligned()
            + idx.wrapping_mul(3).wrapping_mul(8);
        let raw = ((entry + ENTRY_WORD) as *const u32).read_unaligned();
        let packed =
            (((entry + FLAGS_OFF) as *const u16).read_unaligned() >> PACKED_BIT) & 1 != 0;
        let v0 = if packed {
            let shift = ((raw >> 11) & 0xf) + EXP_BIAS;
            (raw & MANT_MASK).wrapping_shl(shift)
        } else {
            raw
        };
        let a0 = (this + ACC0) as *mut u32;
        a0.write_unaligned(a0.read_unaligned().wrapping_sub(v0));
        // The second cost is subtracted only when the flag is set; otherwise
        // the original skips the store and returns eax as the `(an instruction of the original)` left
        // it: the high half of the first cost above the shifted flag word.
        let flagword = ((entry + FLAGS_OFF) as *const u16).read_unaligned();
        if (flagword >> PACKED_BIT) & 1 == 0 {
            return (v0 & 0xffff0000) | ((flagword >> PACKED_BIT) as u32);
        }
        let shift = ((raw >> 26) & 0xf) + EXP_BIAS;
        let v1 = ((raw >> 15) & MANT_MASK).wrapping_shl(shift);
        let a1 = (this + ACC1) as *mut u32;
        a1.write_unaligned(a1.read_unaligned().wrapping_sub(v1));
        v1
    }
});
