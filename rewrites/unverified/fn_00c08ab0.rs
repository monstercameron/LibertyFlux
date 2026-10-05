// original: 0x00c08ab0 stream_message_emit (proposed)

/// Format the streaming message into scratch and hand it to the sink.
///
/// `this` points to the source and the two caller words ride along to the
/// sink. The filler (callee 1) clears the scratch area, the opener (callee 2)
/// runs with the image pointer `M1`, and three writers append to the scratch:
/// the image string at `S1` with its length (callee 3), the source name at
/// `NAME` past `this` with its length (callee 4), and four bytes at the image
/// pointer `M2` (callee 5). The lengths are plain NUL-terminated string
/// lengths. The producer (callee 6) turns the scratch into a handle; a null
/// handle skips the sink, otherwise the sink (callee 7) receives the handle
/// and both caller words and the releaser (callee 8) the handle. The scratch
/// contents are never read back, so the five scratch pointers are skipped in
/// the proof. Returns the cookie check's (callee 9) answer; the cookie word
/// itself is frame-derived and skipped too.
///
/// Original: 0x00c08ab0 (thiscall, two stack words; all callees cdecl).
lf_checker_rt::export!(thiscall, rw_00c08ab0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const NAME: u32 = 0x2c;
        const M1: u32 = 0xebde60;
        const S1: u32 = 0x1168dd8;
        const M2: u32 = 0xebde68;
        const SCRATCH_LEN: u32 = 0x120;
        const FILL: u32 = 1;
        const OPEN: u32 = 2;
        const WRITE0: u32 = 3;
        const WRITE1: u32 = 4;
        const WRITE2: u32 = 5;
        const PRODUCE: u32 = 6;
        const SINK: u32 = 7;
        const RELEASE: u32 = 8;
        const COOKIE: u32 = 9;
        unsafe fn strlen(mut p: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while (p as *const u8).read() != 0 {
                    p = p.wrapping_add(1);
                    n = n.wrapping_add(1);
                }
                n
            }
        }
        let mut scratch = 0u32;
        let sp = (&mut scratch as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(FILL, u32, sp, 0, SCRATCH_LEN);
        let _: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, lf_checker_rt::relocated(M1), 0);
        let s1 = lf_checker_rt::relocated(S1);
        let _: u32 = lf_checker_rt::callee_cdecl!(WRITE0, u32, sp, s1, strlen(s1));
        let name = this.wrapping_add(NAME);
        let _: u32 = lf_checker_rt::callee_cdecl!(WRITE1, u32, sp, name, strlen(name));
        let _: u32 =
            lf_checker_rt::callee_cdecl!(WRITE2, u32, sp, lf_checker_rt::relocated(M2), 4);
        let h: u32 = lf_checker_rt::callee_cdecl!(PRODUCE, u32, sp);
        if h != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(SINK, u32, h, a0, a1);
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, h);
        }
        lf_checker_rt::callee_cdecl!(COOKIE, u32,)
    }
});
