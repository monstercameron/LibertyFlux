// original: 0x00c566b0 task_check_dual_threshold (proposed)

/// Store whether a task parameter meets the low threshold, with a two-stage
/// range test deciding the return code.
///
/// `out` receives 1 when the parameter value (reached as `[[obj+0x40]+0x4c]`)
/// is at or above 0.15 and 0 otherwise, on every path. The return code's low
/// byte is 1 when the value is below 0.5; otherwise it is `(flag == 0)` when
/// the value is also below 0.8 and 0 when it is at or above 0.8. The return
/// value's upper 24 bits are the parameter-block pointer's (the original
/// returns with only `al` set). Unordered (NaN) values take the
/// not-below branch at each stage, matching comiss+jbe. `flag` is read as one
/// byte.
///
/// Original: 0x00c566b0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00c566b0(out: u32, obj: u32, flag: u32) -> u32 {
    unsafe {
        const OBJ_PARAM: u32 = 0x40;
        const PARAM_VALUE: u32 = 0x4c;
        const LOW: f32 = 0.5;
        const HIGH: f32 = 0.8;
        const STORE_AT: f32 = 0.15;
        let mid = ((obj + OBJ_PARAM) as *const u32).read_unaligned();
        let bits = ((mid + PARAM_VALUE) as *const u32).read_unaligned();
        let v = f32::from_bits(bits);
        let code: u32 = if LOW > v {
            1
        } else if HIGH > v {
            u32::from((flag & 0xff) == 0)
        } else {
            0
        };
        (out as *mut u8).write(u8::from(v >= STORE_AT));
        (mid & 0xffff_ff00) | code
    }
});

