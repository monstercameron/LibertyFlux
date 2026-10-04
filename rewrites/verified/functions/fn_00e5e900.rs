// original: 0x00e5e900 net_array_init4_19efc80
/// Init loop over 4 records of 0x40 bytes starting at 0x19EFC80.
///
/// Invokes the element init helper (thiscall/0, stubbed by the checker)
/// on each record front to back. Returns the last helper answer, matching
/// the value the original leaves in EAX.
export!(cdecl, rw_00e5e900() -> u32 {
    unsafe {
        const BASE: u32 = 0x19EFC80;
        const COUNT: u32 = 4;
        const STRIDE: u32 = 0x40;
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
