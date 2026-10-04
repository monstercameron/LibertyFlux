// original: 0x00e62fa0 test_code_hook_e712e0
/// Pass a fixed code address to the engine's hook-testing helper and return
/// its answer. The helper itself is stubbed by the checker.
export!(cdecl, rw_00e62fa0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00e712e0)) }
});
