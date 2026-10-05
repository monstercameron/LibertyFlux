// original: 0x00B3A210 task_flag_or_check

/// Return the masked link word of a flagged object, else run callee 1.
///
/// Reads the link at `obj + 0x6c`. When it is non-null and its flag byte at
/// `+0xe` is non-zero, returns the link with its low byte cleared (the
/// original clears only `al`, so the upper 24 bits are behaviour, not
/// leftovers). Otherwise runs callee 1 as `(obj, f, c, d)` and returns its
/// answer. Cdecl, four stack words.
///
/// Original: 0x00B3A210.

lf_checker_rt::export!(cdecl, rw_00B3A210(obj: u32, c: u32, f: u32, d: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x6C;
        const FLAG_OFF: u32 = 0x0E;
        const CHECK: u32 = 1;
        let link = (obj.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            let flag = (link.wrapping_add(FLAG_OFF) as *const u8).read();
            if flag != 0 {
                return link & 0xFFFF_FF00;
            }
        }
        lf_checker_rt::callee_cdecl!(CHECK, u32, obj, f, c, d)
    }
});
