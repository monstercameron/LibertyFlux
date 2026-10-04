// original: 0x00a72820 any_gate_true (proposed)
/// True (1) when any of three gate checks passes, short-circuiting.
///
/// `cdecl`, one stack word. Calls gate 1 with the argument; if its low
/// byte is zero calls gate 2 (no arguments); if that is also zero calls
/// gate 3. Returns 1 at the first non-zero low byte, else 0. Only the low
/// byte of each answer matters (0x100 counts as false, 0x101 as true).
lf_checker_rt::export!(cdecl, rw_00a72820(arg: u32) -> u8 {
    if lf_checker_rt::callee_cdecl!(1, u32, arg) as u8 != 0 {
        return 1;
    }
    if lf_checker_rt::callee_cdecl!(2, u32,) as u8 != 0 {
        return 1;
    }
    if lf_checker_rt::callee_cdecl!(3, u32,) as u8 != 0 {
        return 1;
    }
    0
});
