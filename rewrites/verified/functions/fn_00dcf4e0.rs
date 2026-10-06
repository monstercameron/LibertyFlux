// original: 0x00dcf4e0 csv_skip_to_nul (proposed)

/// Consume input up to the next NUL character. Returns 1 when a NUL (or an
/// already-empty current char) ends the scan, 0 when the end flag does.
///
/// `this` points to the reader. If the current char at `+0x418` is already
/// NUL, return 1 at once. Otherwise poll the end callee and the next-char
/// callee (both thiscall): a NUL char returns 1, the end flag returns 0.
///
/// Original: 0x00DCF4E0 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf4e0(this: u32) -> u32 {
    unsafe {
        /// End-of-input predicate callee id.
        const AT_END: u32 = 1;
        /// Next-character callee id.
        const NEXT: u32 = 2;
        const CUR: u32 = 0x418;
        if ((this + CUR) as *const u8).read() == 0 {
            return 1;
        }
        let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
        if (e as u8) != 0 {
            return 0;
        }
        loop {
            let c: u32 = lf_checker_rt::callee_thiscall!(NEXT, u32, this);
            if (c as u8) == 0 {
                return 1;
            }
            let e: u32 = lf_checker_rt::callee_thiscall!(AT_END, u32, this);
            if (e as u8) != 0 {
                return 0;
            }
        }
    }
});
