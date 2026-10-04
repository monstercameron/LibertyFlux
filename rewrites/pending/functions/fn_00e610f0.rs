// original: 0x00e610f0 clear_flag_and_forward_18e0106
/// Clear flag bit 0 at 0x018E0106, forward the descriptor, return the answer.
export!(cdecl, rw_00e610f0() -> u32 {
    unsafe {
        *global::<u8>(0x018E0106) &= !1;
        callee_cdecl!(1, u32, relocated(0x00E6FE90))
    }
});
