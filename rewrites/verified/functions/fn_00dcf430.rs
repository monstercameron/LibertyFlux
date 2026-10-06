// original: 0x00dcf430 csv_read_int (proposed)

/// Read one token and parse it as an integer into `out`. Returns 1 on
/// success, 0 when the token could not be read.
///
/// Same shape as the float reader: a 32-byte stack buffer receives the
/// token from the reader callee (thiscall: object, buffer, 0x20 capacity),
/// the parser callee (cdecl, one word) converts the text and returns the
/// integer in EAX for storing to `out`, and both paths run the
/// security-cookie check callee before returning.
///
/// Original: 0x00DCF430 (thiscall, one stack word, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf430(this: u32, out: u32) -> u32 {
    unsafe {
        /// Token-reader callee id: (this, buffer, capacity).
        const READ_TOKEN: u32 = 1;
        /// Integer-parser callee id: (text).
        const PARSE_INT: u32 = 2;
        /// Security-cookie check callee id.
        const COOKIE: u32 = 3;
        const COOKIE_GLOB: u32 = 0x1057fb4;
        /// Token buffer capacity.
        const TOKEN_CAP: u32 = 0x20;
        let cookie = (lf_checker_rt::global::<u32>(COOKIE_GLOB) as *const u32).read_unaligned();
        let mut buf = [0u8; 32];
        let ok: u32 =
            lf_checker_rt::callee_thiscall!(READ_TOKEN, u32, this, buf.as_mut_ptr() as u32, TOKEN_CAP);
        if (ok as u8) == 0 {
            lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
            return 0;
        }
        let v: u32 = lf_checker_rt::callee_cdecl!(PARSE_INT, u32, buf.as_mut_ptr() as u32);
        (out as *mut u32).write_unaligned(v);
        lf_checker_rt::callee_thiscall!(COOKIE, u32, cookie);
        1
    }
});
