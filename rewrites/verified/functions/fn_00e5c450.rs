// original: 0x00e5c450 init_then_store_const
/// Runs the subsystem initializer, then stores the constant 0x00fca26c
/// at 0x018dc2b8. Returns the initializer's answer.
export!(cdecl, rw_00e5c450() -> u32 {
    unsafe {
        const SLOT: u32 = 0x018DC2B8;
        const VALUE: u32 = 0x00FCA26C;
        let answer: u32 = callee_cdecl!(1, u32,);
        *global::<u32>(SLOT) = relocated(VALUE);
        answer
    }
});
