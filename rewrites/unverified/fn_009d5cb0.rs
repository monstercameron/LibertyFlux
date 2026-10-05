// original: 0x009D5CB0 parse_double_dispatch (proposed)
//
/// Parses a string into three slots, then dispatches on two of them.
///
/// Parses `text` with the scanner (callee 1, format selected by a relocated
/// address) into three frame slots (out-only, so the pointers are skipped;
/// the values surface through the snapshots below). Unless the scanner
/// reports exactly 3, its answer is returned. Else slot 1 is resolved
/// through the lookup (callee 2, whose argument is skipped with a snapshot
/// of the value) and slot 0 is dispatched with the result (callee 3,
/// likewise snapped). The trailing security cookie check (callee 4)
/// preserves registers and is called for sequence parity. Cdecl, one stack
/// word.
lf_checker_rt::export!(cdecl, rw_009D5CB0(text: u32) -> u32 {
    unsafe {
        const FMT_FILE: u32 = 0xe96940;
        const WANT: u32 = 3;
        const SCAN: u32 = 1;
        const LOOKUP: u32 = 2;
        const DISPATCH: u32 = 3;
        const COOKIE: u32 = 4;
        let fmt = lf_checker_rt::relocated(FMT_FILE);
        let mut o2 = 0u32;
        let mut o1 = 0u32;
        let mut o0 = 0u32;
        // Pointer order matches the original's stack-arg order (last pushed
        // is the first scanner output).
        let n: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, text, fmt,
            (&mut o0 as *mut u32) as u32,
            (&mut o1 as *mut u32) as u32,
            (&mut o2 as *mut u32) as u32);
        if n != WANT {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return n;
        }
        let q: u32 = lf_checker_rt::callee_cdecl!(
            LOOKUP, u32, (&mut o1 as *mut u32) as u32);
        let r: u32 = lf_checker_rt::callee_thiscall!(
            DISPATCH, u32, q, (&mut o2 as *mut u32) as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        r
    }
});
