// original: 0x009625e0 init_pair_with_clear
/// Initialise two record tables and run the slot-array clear between them.
///
/// First zeroes 0x818 eight-byte slots at 0x1208970 (dword zero, word
/// 0xFFFF), then calls the slot clearer (stubbed by the checker), then
/// falls into the shared tail block that initialises the 0x400-entry
/// 0x14-byte table at 0x11FA030 (same shape as `rw_00962650`).
export!(cdecl, rw_009625e0() -> u32 {
    unsafe {
        let mut ptr = relocated(0x1208970);
        let mut remaining = 0x818u32;
        loop {
            *(ptr as *mut u32) = 0;
            *(ptr.wrapping_add(4) as *mut u16) = 0xFFFF;
            ptr = ptr.wrapping_add(8);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        callee_cdecl!(1, u32,);
        let mut ptr = relocated(0x11FA030);
        let mut remaining = 0x400u32;
        loop {
            *(ptr.wrapping_sub(8) as *mut u32) = 0;
            *(ptr.wrapping_sub(4) as *mut u16) = 0xFFFF;
            *(ptr as *mut u32) = 0;
            *(ptr.wrapping_add(4) as *mut u32) = 0;
            *(ptr.wrapping_add(8) as *mut u16) = 0;
            ptr = ptr.wrapping_add(0x14);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        ptr
    }
});
