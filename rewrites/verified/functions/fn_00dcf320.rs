// original: 0x00dcf320 csv_read_two_ints (proposed)

/// Read one token, scan two integers out of it into `out0`/`out1`, and
/// report whether exactly two items were scanned.
///
/// `this` (in ECX, forwarded to the token reader) points to the reader.
/// Both outputs are zeroed first. A 64-byte stack buffer receives the token
/// from the reader callee (thiscall: object, buffer, 0x40 capacity); a zero
/// byte answer ends with 0. Otherwise the scanner callee (cdecl: text,
/// format string, out0, out1) fills the outputs and its integer answer is
/// compared against 2 for the result. Both paths run the security-cookie
/// check callee before returning.
///
/// Original: 0x00DCF320 (thiscall, two stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf320(this: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        /// Token-reader callee id: (this, buffer, capacity).
        const READ_TOKEN: u32 = 1;
        /// Pair-scanner callee id: (text, format, out0, out1).
        const SCAN_PAIR: u32 = 2;
        /// Security-cookie check callee id.
        const COOKIE: u32 = 3;
        const COOKIE_GLOB: u32 = 0x1057fb4;
        /// Scan format string.
        const FORMAT: u32 = 0xef9f3c;
        /// Token buffer capacity.
        const TOKEN_CAP: u32 = 0x40;
        let cookie = (lf_checker_rt::global::<u32>(COOKIE_GLOB) as *const u32).read_unaligned();
        let mut buf = [0u8; 64];
        (out0 as *mut u32).write_unaligned(0);
        (out1 as *mut u32).write_unaligned(0);
        let ok: u32 =
            lf_checker_rt::callee_thiscall!(READ_TOKEN, u32, this, buf.as_mut_ptr() as u32, TOKEN_CAP);
        if (ok as u8) == 0 {
            lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
            return 0;
        }
        let n: u32 = lf_checker_rt::callee_cdecl!(
            SCAN_PAIR,
            u32,
            buf.as_mut_ptr() as u32,
            lf_checker_rt::relocated(FORMAT),
            out0,
            out1
        );
        lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
        (n == 2) as u32
    }
});
