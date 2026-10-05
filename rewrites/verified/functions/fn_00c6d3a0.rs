// original: 0x00c6d3a0 anim_dict_load_word4 (proposed)

/// Load the word at offset 4 of the dictionary row selected by `idx`.
///
/// The dictionary header comes from its global: the flag byte at
/// `idx + flag_base` decides the path. If its high bit is set the
/// original reads from near-null and faults; otherwise the row address
/// is `stride * idx + base` (wrapping multiply, as the signed imul) and
/// the word at row offset 4 is returned.
///
/// Original: cdecl with one stack word, no calls, reads one global.
/// The faulting path is part of the proof: both sides must fault alike.
lf_checker_rt::export!(cdecl, rw_00c6d3a0(idx: u32) -> u32 {
    unsafe {
        const DICT: u32 = 0x016D_D5D0;
        const FLAG_BASE_OFF: u32 = 0x4;
        const STRIDE_OFF: u32 = 0xC;
        const ROW_WORD_OFF: u32 = 0x4;
        const DEAD_BEAD: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let head = rd32(lf_checker_rt::relocated(DICT));
        let flag_base = rd32(head.wrapping_add(FLAG_BASE_OFF));
        let flag = unsafe { (idx.wrapping_add(flag_base) as *const u8).read() };
        if flag & 0x80 != 0 {
            unsafe { (DEAD_BEAD as *const u32).read_unaligned() }
        } else {
            let stride = rd32(head.wrapping_add(STRIDE_OFF));
            let base = rd32(head);
            let row = stride.wrapping_mul(idx).wrapping_add(base);
            rd32(row.wrapping_add(ROW_WORD_OFF))
        }
    }
});
