// original: 0x00884fa0 stream_driver_probe (proposed)
/// Probe a driver's tables and record whether the final check passed.
///
/// Probes the header pair (intercepted callee 1, cdecl: `obj+0xa8`,
/// `obj+0xac`); a non-zero answer fails with -4. Probes the body quad
/// (intercepted callee 2, cdecl: `obj+0xb0` .. `obj+0xbc` in ascending
/// order); a non-zero answer fails with -3. Converts the limit word found
/// through the table global (global at file address `0x115a448`, word at
/// `+0x80`) from `u32` to `f64` exactly and passes it to the limit check
/// (intercepted callee 3, cdecl, two words); a non-zero answer skips the
/// next probe. Otherwise probes the tail word (intercepted callee 4, cdecl:
/// `obj+0xc0`); a non-zero answer fails with -6. Runs the final check
/// (intercepted callee 5, cdecl, no arguments), records whether its answer
/// was zero at `obj+0xc8`, and returns 0.
///
/// Original: one stack argument, callee cleans 4, returns in `eax`. (Its one
/// in-batch caller passes the object in `ecx` too, but this function never
/// reads `ecx`, so the contract is plain stdcall.)
lf_checker_rt::export!(stdcall, rw_00884fa0(obj: u32) -> u32 {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x0115_a448;
        const LIMIT_WORD: u32 = 0x80;
        const FLAG_BYTE: u32 = 0xc8;
        const HEAD_CALLEE: u32 = 1;
        const BODY_CALLEE: u32 = 2;
        const LIMIT_CALLEE: u32 = 3;
        const TAIL_CALLEE: u32 = 4;
        const FINAL_CALLEE: u32 = 5;
        let head0: u32 = lf_checker_rt::callee_cdecl!(
            HEAD_CALLEE,
            u32,
            obj.wrapping_add(0xa8),
            obj.wrapping_add(0xac)
        );
        if head0 != 0 {
            return 0xFFFF_FFFCu32;
        }
        let body: u32 = lf_checker_rt::callee_cdecl!(
            BODY_CALLEE,
            u32,
            obj.wrapping_add(0xb0),
            obj.wrapping_add(0xb4),
            obj.wrapping_add(0xb8),
            obj.wrapping_add(0xbc)
        );
        if body != 0 {
            return 0xFFFF_FFFDu32;
        }
        let table =
            (lf_checker_rt::global::<u32>(TABLE_GLOBAL) as *const u32).read_unaligned();
        let limit = ((table + LIMIT_WORD) as *const u32).read_unaligned();
        let as_double = limit as f64;
        let bits = as_double.to_bits();
        let limited: u32 = lf_checker_rt::callee_cdecl!(
            LIMIT_CALLEE,
            u32,
            bits as u32,
            (bits >> 32) as u32
        );
        if limited == 0 {
            let tail: u32 =
                lf_checker_rt::callee_cdecl!(TAIL_CALLEE, u32, obj.wrapping_add(0xc0));
            if tail != 0 {
                return 0xFFFF_FFFAu32;
            }
        }
        let final_answer: u32 = lf_checker_rt::callee_cdecl!(FINAL_CALLEE, u32,);
        ((obj + FLAG_BYTE) as *mut u8).write((final_answer == 0) as u8);
        0
    }
});
