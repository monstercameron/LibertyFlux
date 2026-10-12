// original: 0x00DB2F30 UITexture::set_unit_texture_rect

/// Reset the texture unit rectangle. The four single-precision components at
/// receiver offsets +0x1F4 and +0x1FC are written as two exact 64-bit pairs:
/// the first pair is (0.0, 0.0), and the second is (1.0, 1.0). The method has
/// no arguments beyond its receiver and returns no value.
lf_checker_rt::export!(thiscall, rw_00db2f30(this: u32) -> u32 {
    const RECT_ORIGIN_BITS: u64 = 0;
    const RECT_EXTENT_BITS: u64 = 0x3F80_0000_3F80_0000;
    const RECT_ORIGIN: u32 = 0x1F4;
    const RECT_EXTENT: u32 = 0x1FC;
    unsafe {
        (this.wrapping_add(RECT_ORIGIN) as *mut u64).write_unaligned(RECT_ORIGIN_BITS);
        (this.wrapping_add(RECT_EXTENT) as *mut u64).write_unaligned(RECT_EXTENT_BITS);
    }
    0
});
