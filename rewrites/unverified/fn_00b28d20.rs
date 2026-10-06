// original: 0x00b28d20 format_and_load_slot

/// Formats a slot record into a frame buffer and loads it.
///
/// Formats 64 bytes through the formatter callee into a frame buffer from
/// (`a1`, `a2`), passes the buffer 8 bytes in together with `a0` to the
/// loader callee, runs the stack-cookie check and returns the loader's
/// answer. (The cookie value itself mixes the cookie with the stack pointer
/// and cannot self-check identically across frames; the check call is kept
/// so the call sequence matches, with its register argument uncompared.)
/// Cdecl, three stack words.
lf_checker_rt::export!(cdecl, rw_00b28d20(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FMT: u32 = 0x00EAC4C0;
        const BUF_LEN: u32 = 0x40;
        const FORMAT: u32 = 0;
        const LOAD: u32 = 1;
        const COOKIE: u32 = 2;
        let mut buf = [0u32; 16];
        let bp = buf.as_mut_ptr() as u32;
        lf_checker_rt::callee_cdecl!(
            FORMAT, u32, bp, BUF_LEN, a1, a2, 0,
            lf_checker_rt::relocated(FMT)
        );
        let r = lf_checker_rt::callee_cdecl!(LOAD, u32, bp.wrapping_add(8), a0);
        lf_checker_rt::callee_thiscall!(COOKIE, u32, 0);
        r
    }
});
