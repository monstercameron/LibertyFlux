// original: 0x0053CA80 leaderboard_info_ctor

/// Initialise an empty leaderboard info object.
///
/// Installs the class virtual table, zeroes the three counter words, stores
/// the shared class-info pointer, and returns the object pointer. The two
/// stored constants are file addresses the original writes literally (no
/// relocation entries), so the rewrite writes the same literals.
///
/// Original: thiscall with no stack words, returns ECX.

lf_checker_rt::export!(thiscall, rw_0053ca80(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FDC_384;
        const CLASS_INFO: u32 = 0x00FCF_664;
        let p = this as *mut u32;
        p.wrapping_add(1).write_unaligned(0);
        p.wrapping_add(2).write_unaligned(0);
        p.wrapping_add(0).write_unaligned(VTABLE);
        p.wrapping_add(3).write_unaligned(0);
        p.wrapping_add(4).write_unaligned(CLASS_INFO);
        this
    }
});
