// original: 0x00e62fb0 test_code_hook_e71300
/// Pass a second fixed code address to the engine's hook-testing helper and
/// return its answer. The helper itself is stubbed by the checker.
export!(cdecl, rw_00e62fb0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00e71300)) }
});
