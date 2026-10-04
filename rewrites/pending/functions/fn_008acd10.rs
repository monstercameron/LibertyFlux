// original: 0x008acd10 audOcclusionPool_create
/// Allocates the pool header, item array (0x258 x 384 bytes) and flag array,
/// presets all flags to 0x80, publishes the header. Returns 0x258, or 0 with
/// a null publish when the header allocation fails.
export!(cdecl, rw_008acd10() -> u32 {
    unsafe {
        let pool = callee_cdecl!(1, u32, 0x38);
        if pool == 0 {
            *global::<u32>(0x115fd54) = 0;
            return 0;
        }
        callee_thiscall!(4, u32, pool.wrapping_add(8));
        let items = callee_cdecl!(2, u32, 0x38400);
        *(pool as *mut u32) = items;
        let flags = callee_cdecl!(3, u32, 0x258);
        *((pool as *mut u32).add(1)) = flags;
        *((pool as *mut u8).add(0x34)) = 1;
        *((pool as *mut u32).add(0x28 / 4)) = 0x258;
        *((pool as *mut u32).add(0x2c / 4)) = 0x258;
        *((pool as *mut u32).add(0x30 / 4)) = 0xffffffff;
        let mut i: u32 = 0;
        while i < 0x258 {
            *((flags as *mut u8).add(i as usize)) |= 0x80;
            i += 1;
        }
        *global::<u32>(0x115fd54) = pool;
        0x258
    }
});

