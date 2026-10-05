// original: 0x00cf89a0 ladder_climb_anim_request (proposed)

/// Request a climb-ladder animation operation by mode (cdecl, two words).
///
/// `mode` selects one of five request shapes sent to the animation-request
/// callee; `flag`'s low byte matters only for mode 4. Mode 3, mode 0 and any
/// mode above 5 return 0 immediately with no calls (mode 3 is an explicit
/// jump-table entry to the return-0 path, verified against the table).
///
/// Otherwise the shared animation-set word at `ANIM_SET` is fetched and
/// handed in ECX to the fetch callee (thiscall, no stack words); a null
/// answer returns 0. The request callee is then invoked as thiscall with
/// the fetch result in ECX and eleven stack words
/// `(9, op, 4.0, -1, 1, 0, w4, 0, 0, 0, 0)` where `op` is 0x82/0x83 for
/// modes 1/2, 0x84/0x85 for mode 4 (low byte of `flag` zero or not),
/// 0x86 for mode 5, and `w4` is 0 for modes 1-2 and 1 for modes 4-5
/// (the original builds these words with pushes, two of them `(an instruction of the original)`
/// placeholders overwritten in place with 0 and 4.0, which have no
/// equivalent here). A null request answer returns 0; otherwise bit 3 of
/// the byte at `result+0x18` is cleared and the result is returned.
lf_checker_rt::export!(cdecl, rw_00cf89a0(mode: u32, flag: u32) -> u32 {
    unsafe {
        const ANIM_SET: u32 = 0x167e2a0;
        const FETCH: u32 = 1;
        const REQUEST: u32 = 2;
        const SPEED_BITS: u32 = 0x4080_0000;
        const CLEAR_MASK: u8 = 0xf7;
        const RESULT_FLAG: u32 = 0x18;

        let (op, w4): (u32, u32) = match mode {
            1 => (0x82, 0),
            2 => (0x83, 0),
            3 => return 0,
            4 => (if (flag & 0xff) == 0 { 0x84 } else { 0x85 }, 1),
            5 => (0x86, 1),
            _ => return 0,
        };
        let set = (lf_checker_rt::relocated(ANIM_SET) as *const u32).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(FETCH, u32, set);
        if handle == 0 {
            return 0;
        }
        let result = lf_checker_rt::callee_thiscall!(
            REQUEST, u32, handle, 9, op, SPEED_BITS, 0xffff_ffff, 1, 0, w4, 0, 0, 0, 0
        );
        if result == 0 {
            return 0;
        }
        let flags = ((result + RESULT_FLAG) as *mut u8).read();
        ((result + RESULT_FLAG) as *mut u8).write(flags & CLEAR_MASK);
        result
    }
});
