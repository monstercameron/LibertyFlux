// original: 0x00e60fc0 reset_array_128x8_19fe284
/// Reset 128 eight-byte slots at 0x019FE284 to (0xFFFFFFFF, 0).
///
/// Each slot holds a 32-bit marker set to all-ones followed by a 16-bit
/// field set to zero; the trailing two bytes of every slot are left
/// untouched. Returns the address one past the last slot.
export!(cdecl, rw_00e60fc0() -> u32 {
    unsafe {
        const BASE: u32 = 0x019FE284;
        const COUNT: usize = 128;
        const STRIDE: u32 = 8;
        let mut ptr = relocated(BASE);
        for _ in 0..COUNT {
            *(ptr as *mut u32) = 0xFFFF_FFFF;
            *((ptr.wrapping_add(4)) as *mut u16) = 0;
            ptr = ptr.wrapping_add(STRIDE);
        }
        ptr
    }
});
