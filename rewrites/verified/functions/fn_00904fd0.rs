// original: 0x00904FD0 input_control_code_lookup (proposed)
//
// Map an input control index to its handler code and fetch the handler's
// current value through the shared input getter.
//
// `out` receives the result, `idx` selects it, only the low byte of `flags`
// matters, and `alt_out` takes a secondary store on both paths. Index 45 and
// any index above 55 take the default path: store 0 through `alt_out` (unless
// null) and the index rotated right by 8 bits through `out`. Every other
// index maps to a
// (code, flag) pair: most map to `idx + 0x14` with flag `0xFF`, a few consult
// the low flag byte, and two use flag `0x14`. The mapped path calls the input
// getter callee with (address of a slot holding `idx`, code, flag), stores
// the code through `alt_out` (unless null), then stores the dword the getter
// returned a pointer to through `out`. Returns `out`.
//
// Original: 0x00904FD0 (cdecl, four stack words).
fn lookup_code(idx: u32, flag_lo: u8) -> Option<(u32, u8)> {
    const STD_FLAG: u8 = 0xFF;
    let nz = (flag_lo != 0) as u32;
    let z = (flag_lo == 0) as u32;
    match idx {
        0 => Some((0, STD_FLAG)),
        1 => Some((4 + nz, STD_FLAG)),
        2 | 41 => Some((0x10 + 2 * z, STD_FLAG)),
        3 | 39 | 40 => Some((8 + z, STD_FLAG)),
        4 => Some((1 + 2 * z, STD_FLAG)),
        5 | 38 => Some((0x0A + nz, STD_FLAG)),
        6..=37 => Some((idx + 0x14, STD_FLAG)),
        42 => Some((if flag_lo == 0 { 4 } else { 9 }, STD_FLAG)),
        43 | 44 | 46 => Some((0x0B, STD_FLAG)),
        45 => None,
        47 | 51 => Some((9, STD_FLAG)),
        48 => Some((4, STD_FLAG)),
        49 => Some((6, STD_FLAG)),
        50 => Some((7, STD_FLAG)),
        52 => Some((6, 0x14)),
        53 => Some((9, 0x14)),
        54 => Some((0x3E, STD_FLAG)),
        55 => Some((0x3A, STD_FLAG)),
        _ => None,
    }
}

lf_checker_rt::export!(cdecl, rw_00904FD0(out: u32, idx: u32, flags: u32, alt_out: u32) -> u32 {
    unsafe {
        const GETTER: u32 = 1;
        match lookup_code(idx, (flags & 0xFF) as u8) {
            None => {
                if alt_out != 0 {
                    (alt_out as *mut u32).write_unaligned(0);
                }
                (out as *mut u32).write_unaligned(idx.rotate_right(8));
            }
            Some((code, flag)) => {
                let mut slot = idx;
                let got: u32 = lf_checker_rt::callee_cdecl!(
                    GETTER,
                    u32,
                    &mut slot as *mut u32 as u32,
                    code,
                    flag as u32
                );
                let val = (got as *const u32).read_unaligned();
                if alt_out != 0 {
                    (alt_out as *mut u32).write_unaligned(code);
                }
                (out as *mut u32).write_unaligned(val);
            }
        }
        out
    }
});
