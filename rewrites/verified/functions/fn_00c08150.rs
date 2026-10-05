// original: 0x00c08150 stream_header_check (proposed)

/// Open the streaming header and check its two magic words.
///
/// The opener (callee 1) always runs first with the image pointer `M1` and
/// zero; the finder (callee 2) receives the caller's `key` and the image
/// pointer `M2`. A null find skips the reads. Otherwise the reader (callees 3
/// and 4, one id per site so each answer can be scripted on its own) fills
/// two scratch words through out-parameters, and the found flag is set when
/// they equal `MAGIC1` and `MAGIC2`; the releaser (callee 5) then runs with
/// the find. The closer (callee 6) always runs last with the image pointer
/// `M3`. Returns the closer's answer with its low byte replaced by the flag.
///
/// Original: 0x00c08150 (cdecl, key first, one ignored word; all callees cdecl).
lf_checker_rt::export!(cdecl, rw_00c08150(key: u32, _ignored: u32) -> u32 {
    unsafe {
        const M1: u32 = 0xebde8c;
        const M2: u32 = 0xebde94;
        const M3: u32 = 0xebdc8f;
        const MAGIC1: u32 = 0x10291205;
        const MAGIC2: u32 = 0x52334850;
        const OPEN: u32 = 1;
        const FIND: u32 = 2;
        const READ0: u32 = 3;
        const READ1: u32 = 4;
        const RELEASE: u32 = 5;
        const CLOSE: u32 = 6;
        let _: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, lf_checker_rt::relocated(M1), 0);
        let h: u32 = lf_checker_rt::callee_cdecl!(FIND, u32, key, lf_checker_rt::relocated(M2));
        let mut flag = 0u32;
        if h != 0 {
            let mut w0 = 0u32;
            let mut w1 = 0u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                READ0, u32, h, (&mut w0 as *mut u32) as u32, 4);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                READ1, u32, h, (&mut w1 as *mut u32) as u32, 4);
            if w0 == MAGIC1 && w1 == MAGIC2 {
                flag = 1;
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, h);
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(CLOSE, u32, lf_checker_rt::relocated(M3));
        (r & 0xffff_ff00) | flag
    }
});
