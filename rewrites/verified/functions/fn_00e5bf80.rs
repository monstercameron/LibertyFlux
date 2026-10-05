// original: 0x00e5bf80 run_hook_then_dispatch_e5bf80

/// Rewrite of a hook-then-dispatch stub: runs the parameterless hook,
/// then forwards one constant code address to the shared
/// single-argument callee and returns its result.
///
/// Original shape: `call rel32; push imm32; call rel32; (an instruction of the original); ret`.
export!(cdecl, rw_00e5bf80() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E6E220))
    }
});
