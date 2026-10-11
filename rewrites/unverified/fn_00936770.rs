// original: 0x00936770 fn_00936770

/// Forward two single-precision input words and three receiver-derived values
/// to the transition helper. The two float arguments are copied as raw bits
/// in their original order; no arithmetic is performed. The helper receives
/// the receiver fields at `+0xd60` and `+0xd74`, the receiver pointer, the two
/// stack arguments, and three zero words. This is a thiscall routine with two
/// stack arguments and no defined return value.
lf_checker_rt::export!(thiscall, rw_00936770(
    this: u32,
    first_float_bits: u32,
    second_float_bits: u32
) -> () {
    unsafe {
        const HELPER_OBJECT_A: u32 = 0xd60;
        const HELPER_OBJECT_B: u32 = 0xd74;
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let helper_object_a = read_u32(this.wrapping_add(HELPER_OBJECT_A));
        let helper_object_b = read_u32(this.wrapping_add(HELPER_OBJECT_B));
        let _ = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            helper_object_a,
            this,
            helper_object_b,
            first_float_bits,
            second_float_bits,
            0,
            0,
            0
        );
    }
});
