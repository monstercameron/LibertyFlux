// original: 0x00e60f20 forward_ptr_e6fda0
/// Forward the fixed descriptor at 0x00E6FDA0 to the shared helper.
///
/// Pushes the constant and calls the shared callee (cdecl/1, stubbed by the
/// checker), returning whatever it answers.
export!(cdecl, rw_00e60f20() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E6FDA0)) }
});
