// original: 0x00e63150 init_array_64x20
/// Forward init sweep over 64 records of 0x20 bytes at 0x1176E80.
///
/// Same shape as [`rw_00e63130`] with twice as many elements. Returns the
/// last element answer in EAX.
export!(cdecl, rw_00e63150() -> u32 {
    unsafe {
        const BASE: u32 = 0x1176E80;
        const COUNT: u32 = 64;
        const STRIDE: u32 = 0x20;
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
