// original: 0x0094EEF0 reset_four_subsystems (proposed)

/// Reset four global sub-objects in order, the last by tail call.
///
/// Calls callees 1-3 as thiscalls with the constant object addresses
/// `OBJ_A`, `OBJ_B`, `OBJ_C` in ECX (no stack words, answers ignored),
/// then tails into callee 4 with `OBJ_D` in ECX and returns its answer.
/// Written as a call that forwards the argument and result; the original
/// ends in a jump.
///
/// Original: 0x0094EEF0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_0094EEF0() -> u32 {
    unsafe {
        const OBJ_A: u32 = 0x11F6310;
        const OBJ_B: u32 = 0x11F6418;
        const OBJ_C: u32 = 0x11F61B8;
        const OBJ_D: u32 = 0x11F6598;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ_A));
        lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(OBJ_B));
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(OBJ_C));
        lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(OBJ_D))
    }
});
