// original: 0x00c6c760 anim_dict_store (proposed)

/// Store `val` into the dictionary row selected by `idx`, reporting
/// whether `val` is nonzero.
///
/// Same header layout as the matching loader: if the flag byte at
/// `idx + flag_base` has its high bit set the original stores through a
/// null pointer and faults; otherwise `val` is written at
/// `stride * idx + base` and 1 is returned for nonzero `val`, else 0.
///
/// Original: cdecl with two stack words, no calls, reads one global.
/// The faulting path is part of the proof: both sides must fault alike.
lf_checker_rt::export!(cdecl, rw_00c6c760(idx: u32, val: u32) -> u32 {
    unsafe {
        const DICT: u32 = 0x016D_D5D0;
        const FLAG_BASE_OFF: u32 = 0x4;
        const STRIDE_OFF: u32 = 0xC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let head = rd32(lf_checker_rt::relocated(DICT));
        let flag_base = rd32(head.wrapping_add(FLAG_BASE_OFF));
        let flag = unsafe { (idx.wrapping_add(flag_base) as *const u8).read() };
        if flag & 0x80 != 0 {
            unsafe { (0u32 as *mut u32).write_unaligned(val) };
            0
        } else {
            let stride = rd32(head.wrapping_add(STRIDE_OFF));
            let base = rd32(head);
            let row = stride.wrapping_mul(idx).wrapping_add(base);
            unsafe { (row as *mut u32).write_unaligned(val) };
            (val != 0) as u32
        }
    }
});
