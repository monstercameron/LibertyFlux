// original: 0x00e5e920 net_array_init32_19efd80
/// Init loop over 32 records of 0x40 bytes starting at 0x19EFD80.
///
/// Invokes the element init helper (thiscall/0, stubbed by the checker)
/// on each record front to back. Returns the last helper answer, matching
/// the value the original leaves in EAX.
export!(cdecl, rw_00e5e920() -> u32 {
    unsafe {
        const BASE: u32 = 0x19EFD80;
        const COUNT: u32 = 32;
        const STRIDE: u32 = 0x40;
        
        let mut ptr = relocated(BASE);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            last = callee_thiscall!(1, u32, ptr);
            ptr = ptr.wrapping_add(STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
