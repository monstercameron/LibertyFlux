// original: 0x0053CA80 leaderboard_info_ctor

/// Initialise an empty leaderboard info object.
///
/// Installs the class virtual table, zeroes the three counter words, stores
/// the shared class-info pointer, and returns the object pointer. The two
/// stored constants are file addresses the worker relocates with the image,
/// so the rewrite derives them with `relocated()` like any image address.
///
/// Original: thiscall with no stack words, returns ECX.

lf_checker_rt::export!(thiscall, rw_0053ca80(this: u32) -> u32 {
    unsafe {
        let vtable: u32 = lf_checker_rt::relocated(0x00FDC_384);
        let class_info: u32 = lf_checker_rt::relocated(0x00FCF_664);
        let p = this as *mut u32;
        p.wrapping_add(1).write_unaligned(0);
        p.wrapping_add(2).write_unaligned(0);
        p.wrapping_add(0).write_unaligned(vtable);
        p.wrapping_add(3).write_unaligned(0);
        p.wrapping_add(4).write_unaligned(class_info);
        this
    }
});
