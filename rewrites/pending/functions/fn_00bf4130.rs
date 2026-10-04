// original: 0x00bf4130 pack_rgb_lo
/// Pack the low three bytes of `v` into slots +0x11/+0x12/+0x13.
export!(thiscall, rw_bf4130(this: *mut u8, v: u32) -> u32 {
    unsafe {
        *this.add(0x11) = v as u8;
        *this.add(0x12) = (v >> 8) as u8;
        *this.add(0x13) = (v >> 16) as u8;
        v >> 8
    }
});
