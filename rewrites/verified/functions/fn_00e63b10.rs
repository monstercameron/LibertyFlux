// original: 0x00e63b10 init_object_array
/// Initialise 16 objects of 0x30 bytes at 0x11EE2B0.
///
/// Invokes the element initialiser (thiscall/0, stubbed by the checker) on
/// each slot in order. Returns the last answer, matching the original's EAX.
export!(cdecl, rw_00e63b10() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11EE2B0;
        const COUNT: usize = 16;
        const STRIDE: u32 = 0x30;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut obj = relocated(TABLE);
        let mut last = 0u32;
        for _ in 0..COUNT {
            last = init(obj);
            obj = obj.wrapping_add(STRIDE);
        }
        last
    }
});
