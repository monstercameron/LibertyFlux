/// Deliberately wrong version of `rw_00535380` (returned entry plus one); must fail.
lf_checker_rt::export!(thiscall, rw_00535380(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x83;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_TABLE: usize = 4;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let table = st[ST_TABLE];
        rd(table, index).wrapping_add(1)
    }});

#[inline(always)]
unsafe fn rd(base: u32, idx: u32) -> u32 {
    unsafe { (base.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned() }
}
