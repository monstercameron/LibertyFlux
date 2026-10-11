// original: 0x00A84270 CSimpleNMExplosionTaskInfo::append_explosion_position

/// Forward the base report and log the three f32 coordinates at +0x20, +0x24 and +0x28 after widening each to the logger's f64 argument in x, y, z order.
lf_checker_rt::export!(thiscall, rw_00a84270(this: u32, logger_context: u32) -> () {

    #[inline(always)]
    fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { core::ptr::read(base.wrapping_add(offset) as *const u8) }
    }
    #[inline(always)]
    fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32) }
    }
    #[inline(always)]
    fn double_words(value: f32) -> (u32, u32) {
        let bits = (value as f64).to_bits();
        (bits as u32, (bits >> 32) as u32)
    }

    const FIELD_X: u32 = 0x20;
    const FIELD_Y: u32 = 0x24;
    const FIELD_Z: u32 = 0x28;
    const FORMAT_EXPLOSION_POSITION: u32 = 0x00EA0C38;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, logger_context);
    let x = f32::from_bits(read_u32(this, FIELD_X));
    let y = f32::from_bits(read_u32(this, FIELD_Y));
    let z = f32::from_bits(read_u32(this, FIELD_Z));
    let (x_low, x_high) = double_words(x);
    let (y_low, y_high) = double_words(y);
    let (z_low, z_high) = double_words(z);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, logger_context, 1u32, 1u32, lf_checker_rt::relocated(FORMAT_EXPLOSION_POSITION), x_low, x_high, y_low, y_high, z_low, z_high);
});
