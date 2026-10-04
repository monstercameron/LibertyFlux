// original: 0x00e5e4e0 net_pool_init_e4e0
/// Initialise a pool of 959 network objects, then register the handler.
///
/// For each of the 960 consecutive 0x50-byte slots starting at `0x019D5A50`:
/// zeroes the dword 0x40 bytes before the slot, initialises the object 0x30
/// bytes before the slot, and zeroes the slot's first word. Then registers
/// the handler at `0x00E6EF20` and returns the registrar's answer.
export!(cdecl, rw_00e5e4e0() -> u32 {
    unsafe {
        /// First cleared dword (file VA).
        const FIRST_DWORD: u32 = 0x019D5A10;
        /// First initialised object (file VA).
        const FIRST_OBJ: u32 = 0x019D5A20;
        /// First cleared word (file VA).
        const FIRST_WORD: u32 = 0x019D5A50;
        /// Handler registered (file VA).
        const HANDLER: u32 = 0x00E6EF20;
        /// Slot stride in bytes.
        const STRIDE: u32 = 0x50;
        /// Slots in the pool.
        const COUNT: usize = 960;
        let mut i = 0;
        while i < COUNT {
            let step = STRIDE * i as u32;
            global::<u32>(FIRST_DWORD.wrapping_add(step)).write(0);
            let _: u32 = callee_thiscall!(1, u32, relocated(FIRST_OBJ.wrapping_add(step)));
            global::<u16>(FIRST_WORD.wrapping_add(step)).write(0);
            i += 1;
        }
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
