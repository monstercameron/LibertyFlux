// original: 0x00bf4110 pack_rgb_hi
/// Pack the low three bytes of `v` into slots +0x15/+0x16/+0x17.
export!(thiscall, rw_bf4110(this: *mut u8, v: u32) -> u32 {
    unsafe {
        *this.add(0x15) = v as u8;
        *this.add(0x16) = (v >> 8) as u8;
        *this.add(0x17) = (v >> 16) as u8;
        v >> 8
    }
});
