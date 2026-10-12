// original: 0x00E71010 conditional_cleanup_00e71010

/// If the 16-bit state at 0x0115A52A is nonzero, loads the
/// pointer at 0x0115A524 and passes it to the intercepted
/// cdecl cleanup routine. The wrapper has no semantic return value;
/// the checker compares the outgoing call. The state is tested before
/// the pointer is read, matching the original's guarded access.
lf_checker_rt::export!(cdecl, rw_00e71010() -> u32 {
    const STATE_VA: u32 = 0x0115A52A;
    const OBJECT_POINTER_VA: u32 = 0x0115A524;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        if read_u16(lf_checker_rt::relocated(STATE_VA)) != 0 {
            let object = read_u32(lf_checker_rt::relocated(OBJECT_POINTER_VA));
            let _ = lf_checker_rt::callee_cdecl!(1, u32, object);
        }
        0
    }
});
