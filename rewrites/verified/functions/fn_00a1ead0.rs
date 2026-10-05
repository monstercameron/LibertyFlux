// original: 0x00a1ead0 cam_follow_flag_test (proposed)

/// Tests whether an object's mode field selects follow mode 0x80.
///
/// `obj` is either null or points to an object whose 32-bit field at
/// `+FLAGS_OFF` holds mode bits. Returns 1 when `obj` is non-null and
/// `(field & MODE_MASK) == MODE_FOLLOW`, otherwise 0. Only the `al`
/// channel carries the result: on the false path the original leaves the
/// masked value's upper bytes in `eax` (`(an instruction of the original)` clears just the low
/// byte), so the comparison is `al`-only by the function's own convention.
///
/// Original: 0x00a1ead0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a1ead0(obj: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: u32 = 0x28;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_FOLLOW: u32 = 0x80;
        if obj == 0 {
            return 0;
        }
        let field = ((obj + FLAGS_OFF) as *const u32).read_unaligned();
        u32::from(field & MODE_MASK == MODE_FOLLOW)
    }
});
