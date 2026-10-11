// original: 0x006F7E60 mainloop_timer_clear_state

/// Clear the timer state fields at offsets 0x00, 0x04, 0x08, 0x0C, 0x18C,
/// 0x1CC, 0x250, 0x254 and 0x269 in the object passed in ECX. The dword
/// fields are cleared as whole words; the three flags are single-byte stores.
/// The method has no stack arguments and no meaningful return value.
lf_checker_rt::export!(thiscall, rw_006f7e60(this: u32) -> u32 {
    unsafe {
        const ZEROED_DWORDS: [u32; 4] = [0x00, 0x04, 0x08, 0x250];
        const ZEROED_BYTES: [u32; 5] = [0x0c, 0x18c, 0x1cc, 0x254, 0x269];
        for offset in ZEROED_DWORDS {
            ((this + offset) as *mut u32).write_unaligned(0);
        }
        for offset in ZEROED_BYTES {
            ((this + offset) as *mut u8).write(0);
        }
    }
    0
});
