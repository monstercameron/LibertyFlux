// original: 0x00da41a0 NativeImpl_IS_CHAR_USING_MAP_ATTRACTOR_2
// s10f08: get-or-create the 12-byte singleton (cdecl/0).
//
// Returns the cached pointer when set. Otherwise allocates 12 bytes through
// the global allocator, zeroes the block, runs the one-time init step, caches
// and returns the block. A failed allocation caches and returns null.
export!(cdecl, rw_s10f08() -> u32 {
    unsafe {
        let g = global::<u32>(0x17A64FC);
        if *g != 0 {
            return *g;
        }
        let m: u32 = callee_cdecl!(1, u32, 12u32);
        if m == 0 {
            *g = 0;
            return 0;
        }
        for i in 0..3 {
            *((m as *mut u32).add(i)) = 0;
        }
        let _: u32 = callee_cdecl!(2, u32,);
        *g = m;
        m
    }
});
