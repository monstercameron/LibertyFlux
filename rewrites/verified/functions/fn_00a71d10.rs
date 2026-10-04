// original: 0x00a71d10 lookup_or_minus1 (proposed)
/// Looks `index` up in a dword table, returning -1 on a miss.
///
/// `stdcall`, one stack word. Returns -1 when `index` is -1 or when the
/// validity byte for the slot is zero; otherwise returns the table word.
/// The validity bytes sit 0x10 past the table base, so the two views
/// overlap in the harness's single declared range, exactly as in the game.
lf_checker_rt::export!(stdcall, rw_00a71d10(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012fa6bc;
        const VALID: u32 = 0x012fa6cc;
        if index == 0xffff_ffff {
            return 0xffff_ffff;
        }
        let ok = (lf_checker_rt::relocated(VALID).wrapping_add(index)) as *const u8;
        if ok.read() == 0 {
            return 0xffff_ffff;
        }
        ((lf_checker_rt::relocated(TABLE) + index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
