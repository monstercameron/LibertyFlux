// original: 0x00B49280 CEventShocking::vf8

/// Whether the shocking event affects a ped: the ped must not be flagged busy; a raised flag with a set mode accepts at once, otherwise the squared length of the event's stored offset must exceed 0.05.
///
/// `this` points to the event, `ped` to the candidate ped. Both are read at
/// the byte offsets below; all comparisons are unsigned. Returns 1 when the
/// event affects the ped, 0 otherwise.
lf_checker_rt::export!(thiscall, rw_00b49280(this: u32, ped: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    #[inline(always)]
    fn mul_ss(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add_ss(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }

    unsafe {
        if read_u8(ped.wrapping_add(0x211)) != 0 {
            return 0;
        }
        let flag = read_u8(this.wrapping_add(0x36));
        let mode = read_u32(this.wrapping_add(0x38));
        if flag != 0 {
            return (mode != 0) as u32;
        }
        if mode != 0 {
            return 0;
        }
        let x = f32::from_bits(read_u32(this.wrapping_add(0x20)));
        let y = f32::from_bits(read_u32(this.wrapping_add(0x24)));
        let z = f32::from_bits(read_u32(this.wrapping_add(0x28)));
        let dist2 = add_ss(add_ss(mul_ss(x, x), mul_ss(y, y)), mul_ss(z, z));
        (dist2 > 0.05) as u32
    }
});
