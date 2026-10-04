// original: 0x00e63340 init_array_67x1c
/// Forward init sweep over 67 records of 0x1C bytes at 0x118E928.
///
/// Same shape as [`rw_00e63130`]. Returns the last element answer in EAX.
export!(cdecl, rw_00e63340() -> u32 {
    unsafe {
        const BASE: u32 = 0x118E928;
        const COUNT: u32 = 67;
        const STRIDE: u32 = 0x1C;
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
