// original: 0x00da4450 alloc_and_init_subsystem
// s10f13: allocate and initialise the subsystem block (cdecl/0).
//
// Allocates 28 bytes, runs the block init step over them, caches the init
// result in the subsystem global and returns it. A failed allocation caches
// and returns null.
export!(cdecl, rw_s10f13() -> u32 {
    unsafe {
        let m: u32 = callee_cdecl!(1, u32, 0x1cu32);
        let g = global::<u32>(0x17A64E8);
        if m == 0 {
            *g = 0;
            return 0;
        }
        // Stack-arg order at the original site is (0x40, table, 0xc0).
        let init: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let r = init(m, 0x40, relocated(0xEEF108), 0xC0);
        *g = r;
        r
    }
});
