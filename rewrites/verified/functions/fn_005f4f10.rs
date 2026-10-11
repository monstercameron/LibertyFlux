// original: 0x005F4F10 fragInstNMGta::vf18

/// Scale a three-component vector and pass it to the instance's virtual slot at
/// `0x44` together with the remaining three stack words.
///
/// `this` is a fragment instance whose first word is its function table. The
/// first stack argument points to three `f32` components; the next two are
/// forwarded unchanged, and the fourth is forwarded as its original 32-bit
/// float word. Each component is multiplied, in component-then-scale order, by
/// the immutable scale value stored in the image. A temporary vector is passed
/// to the virtual call, and its `u32` result is returned. The thiscall method
/// consumes four stack arguments. The object and vector use the original
/// 32-bit memory layout under the checker.
lf_checker_rt::export!(thiscall, rw_005f4f10(
    this: u32,
    vector: u32,
    arg1: u32,
    arg2: u32,
    scalar_bits: u32,
) -> u32 {
    const SCALE_FACTOR: u32 = 0x00FE_8750;
    const VECTOR_X: u32 = 0;
    const VECTOR_Y: u32 = 4;
    const VECTOR_Z: u32 = 8;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    unsafe {
        let scale = core::ptr::read_unaligned(lf_checker_rt::global::<f32>(SCALE_FACTOR));
        let mut scaled_vector = [
            multiply_f32(read_f32(vector.wrapping_add(VECTOR_X)), scale),
            multiply_f32(read_f32(vector.wrapping_add(VECTOR_Y)), scale),
            multiply_f32(read_f32(vector.wrapping_add(VECTOR_Z)), scale),
        ];
        let temp_vector = scaled_vector.as_mut_ptr() as usize as u32;
        lf_checker_rt::callee_thiscall!(1, u32, this, temp_vector, arg1, arg2, scalar_bits)
    }
});
