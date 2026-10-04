// original: 0x00dfc669 vswprintf_s_l
/// Locale-aware bounded wide-character formatted print.
///
/// Validates the buffer, size and format arguments (reporting `EINVAL` and
/// returning -1 when any is missing), then forwards everything to the
/// formatting helper. A negative result NUL-terminates the buffer; the
/// helper's truncation signal is translated to `ERANGE` with a -1 return.
export!(cdecl, rw_00dfc669(buf: u32, count: u32, fmt: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        if fmt == 0 || buf == 0 || count == 0 {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x16);
            callee_cdecl!(2, u32,);
            return 0xFFFF_FFFF;
        }
        let out = lf_checker_rt::relocated(0x00e082a2);
        let written = callee_cdecl!(3, u32, out, buf, count, fmt, a3, a4);
        if (written as i32) < 0 {
            (buf as *mut u16).write(0);
        }
        if written == 0xFFFF_FFFE {
            let slot = callee_cdecl!(1, u32,);
            (slot as *mut u32).write(0x22);
            callee_cdecl!(2, u32,);
            return 0xFFFF_FFFF;
        }
        written
    }
});
