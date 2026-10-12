// original: 0x008DA690 streaming_context_reset

/// Release the three streaming resources in order, then invoke the reset helper and clear the context flag at +0xfca. The object arrives in ECX and the helper's EAX return value is preserved.
lf_checker_rt::export!(thiscall, rw_008da690(this: u32) -> u32 {
    const RELEASE_CALLEE: u32 = 65;
    const RELEASE_LAST_CALLEE: u32 = 66;
    const RESET_CALLEE: u32 = 67;
    const RELEASE_A_OFFSET: usize = 0xFAC;
    const RELEASE_B_OFFSET: usize = 0xFB4;
    const RELEASE_C_OFFSET: usize = 0xFBC;
    const RESET_FLAG_OFFSET: usize = 0xFCA;
    unsafe {
        let context = this as *mut u8;
        let first = core::ptr::read_unaligned(context.wrapping_add(RELEASE_A_OFFSET).cast::<u32>());
        let second = core::ptr::read_unaligned(context.wrapping_add(RELEASE_B_OFFSET).cast::<u32>());
        let third = core::ptr::read_unaligned(context.wrapping_add(RELEASE_C_OFFSET).cast::<u32>());
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, first);
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, second);
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_LAST_CALLEE, u32, third);
        let reset_result = lf_checker_rt::callee_cdecl!(RESET_CALLEE, u32,);
        core::ptr::write_unaligned(context.wrapping_add(RESET_FLAG_OFFSET), 0);
        reset_result
    }
});
