// original: 0x00e631d0 init_array_4xbc
/// Forward init sweep over 4 records of 0xBC bytes at 0x118D470.
///
/// Same shape as [`rw_00e63130`] with a wider stride and a different
/// element routine. Returns the last element answer in EAX.
export!(cdecl, rw_00e631d0() -> u32 {
    unsafe {
        const BASE: u32 = 0x118D470;
        const COUNT: u32 = 4;
        const STRIDE: u32 = 0xBC;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut ptr = relocated(BASE);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            last = init(ptr);
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
