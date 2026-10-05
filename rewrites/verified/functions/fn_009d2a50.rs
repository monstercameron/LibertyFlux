// original: 0x009D2A50 sscanf_gate_dispatch (proposed)
//
/// Parses a string into slots, then dispatches each value until a sentinel.
///
/// Parses `text` with the scanner (callee 1, format selected by a relocated
/// address) into sixteen frame slots (out-only, so the pointers are skipped
/// and the values are observed through the dispatch calls below). When the
/// flag at `obj + 0x70` is zero, the scanner's answer is returned as is.
/// Else the sixteen slots are read in reverse push order starting from the
/// last-pushed one, dispatching
/// each (callee 2 with `(aux, value)`) until a value of -1, and -1 is
/// returned. The trailing
/// security cookie check (callee 3) preserves registers and is called for
/// sequence parity. Cdecl; three stack words are declared because the
/// original reads the object from slot 0, a context word from slot 1
/// (passed to every dispatch call) and the text from slot 2.
lf_checker_rt::export!(cdecl, rw_009D2A50(obj: u32, aux: u32, text: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x70;
        const FMT_FILE: u32 = 0xe96860;
        const SCAN: u32 = 1;
        const DISPATCH: u32 = 2;
        const COOKIE: u32 = 3;
        const SENTINEL: u32 = 0xffffffff;
        let fmt = lf_checker_rt::relocated(FMT_FILE);
        let mut outs = [0u32; 16];
        let p = [
            &mut outs[0] as *mut u32, &mut outs[1] as *mut u32,
            &mut outs[2] as *mut u32, &mut outs[3] as *mut u32,
            &mut outs[4] as *mut u32, &mut outs[5] as *mut u32,
            &mut outs[6] as *mut u32, &mut outs[7] as *mut u32,
            &mut outs[8] as *mut u32, &mut outs[9] as *mut u32,
            &mut outs[10] as *mut u32, &mut outs[11] as *mut u32,
            &mut outs[12] as *mut u32, &mut outs[13] as *mut u32,
            &mut outs[14] as *mut u32, &mut outs[15] as *mut u32,
        ];
        // Pointer order matches the original's stack-arg order: arg(2+k)
        // receives the k-th value read. Reads 0..9 land in outs[9]..outs[0],
        // reads 10..15 in outs[15]..outs[10].
        let n: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, text, fmt,
            p[9] as u32, p[8] as u32, p[7] as u32, p[6] as u32,
            p[5] as u32, p[4] as u32, p[3] as u32, p[2] as u32,
            p[1] as u32, p[0] as u32, p[15] as u32, p[14] as u32,
            p[13] as u32, p[12] as u32, p[11] as u32, p[10] as u32);
        let flag = (obj.wrapping_add(FLAG) as *const u32).read_unaligned();
        if flag == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return n;
        }
        let mut v = outs[9];
        if v != SENTINEL {
            let order = [9usize, 8, 7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10];
            for k in order {
                v = outs[k];
                if v == SENTINEL {
                    break;
                }
                let _: u32 = lf_checker_rt::callee_stdcall!(DISPATCH, u32, aux, v);
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        SENTINEL
    }
});
