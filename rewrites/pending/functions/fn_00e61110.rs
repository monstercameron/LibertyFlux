// original: 0x00e61110 clear_flag_and_forward_18e0107
/// Clear flag bit 0 at 0x018E0107, forward the descriptor, return the answer.
export!(cdecl, rw_00e61110() -> u32 {
    unsafe {
        *global::<u8>(0x018E0107) &= !1;
        callee_cdecl!(1, u32, relocated(0x00E6FEF0))
    }
});
