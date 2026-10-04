// original: 0x009e8c30 ped_mode_blend_value
/// Blend weight selected by the mode word at `+0x144`: 0 gives 0.0,
/// 2 gives 1.0, anything else reads the float at `+0x7D4`, or at
/// `+0x8B4` once the word at `+0x7B8` exceeds 2. (thiscall, x87 return.)
lf_checker_rt::export!(thiscall, rw_009e8c30(this_ptr: u32) -> f32 {
    unsafe {
        const MODE_OFF: u32 = 0x144;
        const LIMIT_OFF: u32 = 0x7B8;
        const LO_OFF: u32 = 0x7D4;
        const HI_OFF: u32 = 0x8B4;
        unsafe fn rf(base: u32, off: u32) -> f32 {
            f32::from_bits((base.wrapping_add(off) as *const u32).read_unaligned())
        }
        let mode = (this_ptr.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
        if mode == 0 {
            0.0
        } else if mode == 2 {
            1.0
        } else if (this_ptr.wrapping_add(LIMIT_OFF) as *const u32).read_unaligned() as i32 > 2 {
            rf(this_ptr, HI_OFF)
        } else {
            rf(this_ptr, LO_OFF)
        }
    }
});
