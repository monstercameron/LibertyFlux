// original: 0x00e5bfa0 dispatch_const_target_e5bfa0

/// Rewrite of a push-and-dispatch stub: forwards one constant code
/// address to the shared single-argument callee and returns its result.
///
/// Original shape: `push imm32; call rel32; (an instruction of the original); ret` (cdecl/1 callee).
export!(cdecl, rw_00e5bfa0() -> u32 {
    callee_cdecl!(2, u32, relocated(0x00E6E230))
});
