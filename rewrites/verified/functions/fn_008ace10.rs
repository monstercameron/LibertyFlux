// original: 0x008ace10 audOcclusionPool_destroy
/// Frees the arrays (only when the count is positive and the owned flag is
/// set), clears the header, tears down the lock and frees the header.
/// Returns entry EAX on the null-pool path, so the return channel is `none`.
export!(cdecl, rw_008ace10() -> u32 {
    unsafe {
        let pool = *global::<u32>(0x115fd54);
        if pool != 0 {
            if *((pool as *const i32).add(0x28 / 4)) > 0 {
                if *((pool as *const u8).add(0x34)) != 0 {
                    callee_cdecl!(1, u32, *(pool as *const u32));
                    callee_cdecl!(2, u32, *((pool as *const u32).add(1)));
                }
                *(pool as *mut u32) = 0;
                *((pool as *mut u32).add(1)) = 0;
                *((pool as *mut u32).add(0x28 / 4)) = 0;
                *((pool as *mut u32).add(0x2c / 4)) = 0;
                *((pool as *mut u32).add(0x30 / 4)) = 0;
            }
            callee_thiscall!(3, u32, pool.wrapping_add(8));
            callee_cdecl!(4, u32, pool);
        }
        0
    }
});

