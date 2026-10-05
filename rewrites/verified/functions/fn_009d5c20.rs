// original: 0x009D5C20 parse14_forward (proposed)
//
/// Parses fourteen fields, then forwards the frame with an aux value.
///
/// Parses `text` with the scanner (callee 1) into fourteen frame slots; the
/// out-pointers are skipped and observed via one 14-word snapshot. Unless
/// the scanner reports exactly 14 (equality compare, signedness-neutral),
/// its answer is returned. Else the frame base plus `aux` are forwarded to
/// the consumer (callee 2, whose frame pointer is likewise skipped with a
/// 14-word snapshot) and its answer returned. The trailing security cookie
/// check (callee 3) preserves registers and is called for sequence parity.
/// Cdecl, two words.
lf_checker_rt::export!(cdecl, rw_009D5C20(text: u32, aux: u32) -> u32 {
    unsafe {
        const FMT: u32 = 0xe96ee4;
        const WANT: u32 = 14;
        const SCAN: u32 = 1;
        const CONSUME: u32 = 2;
        const COOKIE: u32 = 3;
        let fmt = lf_checker_rt::relocated(FMT);
        let mut f = [0u32; 14];
        let n: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, text, fmt,
            (&mut f[0] as *mut u32) as u32,
            (&mut f[1] as *mut u32) as u32,
            (&mut f[2] as *mut u32) as u32,
            (&mut f[3] as *mut u32) as u32,
            (&mut f[4] as *mut u32) as u32,
            (&mut f[5] as *mut u32) as u32,
            (&mut f[6] as *mut u32) as u32,
            (&mut f[7] as *mut u32) as u32,
            (&mut f[8] as *mut u32) as u32,
            (&mut f[9] as *mut u32) as u32,
            (&mut f[10] as *mut u32) as u32,
            (&mut f[11] as *mut u32) as u32,
            (&mut f[12] as *mut u32) as u32,
            (&mut f[13] as *mut u32) as u32);
        if n != WANT {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return n;
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(
            CONSUME, u32, (&mut f[0] as *mut u32) as u32, aux);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        r
    }
});
