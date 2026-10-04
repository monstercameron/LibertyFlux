// original: 0x00cca720 state_flag_init
/// Tiny state setter: when the second argument is non-null, clear the word
/// at +0x14 and set the byte at +0x18, then return the pointer unchanged.
/// The first argument is ignored. (No merged symbol; name proposed.)
export!(cdecl, rw_00cca720(_a0: u32, a1: u32) -> u32 {
    unsafe {
        if a1 != 0 {
            *((a1 as *mut u8).add(0x14) as *mut u32) = 0;
            *((a1 as *mut u8).add(0x18)) = 1;
        }
        a1
    }
});
