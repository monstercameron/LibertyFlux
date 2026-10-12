// original: 0x00B27250 conditionally_copy_nearby_blend_component

/// Require a nonzero presence word, flag bit 0x20, and a clear exclusion bit
/// 0x08000000. Measure the squared three dimensional distance from the linked
/// position at +0x30 or the inline position at +0x10 to the target vector. The
/// ordered single precision sum is y² + x² + z². Call the clone helper only
/// when the distance is strictly below the image-provided limit; unordered
/// comparisons skip it.
/// This cdecl helper takes receiver and target pointers and has no defined
/// return value.
const NEAR_BLEND_LIMIT_FILE_VA: u32 = 0x00FE8CA4;
const BLEND_PRESENT_OFFSET: u32 = 0x70;
const BLEND_FLAGS_OFFSET: u32 = 0x24;
const BLEND_SOURCE_OFFSET: u32 = 0x20;
const INLINE_POSITION_OFFSET: u32 = 0x10;
const LINKED_POSITION_OFFSET: u32 = 0x30;

lf_checker_rt::export!(cdecl, rw_b27250(receiver: u32, target: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        if read_word(receiver.wrapping_add(BLEND_PRESENT_OFFSET)) == 0 { return 0; }
        let flags = read_word(receiver.wrapping_add(BLEND_FLAGS_OFFSET));
        if flags & 0x20 == 0 || flags & 0x0800_0000 != 0 { return 0; }
        let linked = read_word(receiver.wrapping_add(BLEND_SOURCE_OFFSET));
        let position = if linked == 0 { receiver.wrapping_add(INLINE_POSITION_OFFSET) }
                       else { linked.wrapping_add(LINKED_POSITION_OFFSET) };
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss, _mm_sub_ss};
        #[inline(always)]
        fn subtract(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        #[inline(always)]
        fn multiply(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            unsafe { _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(core::hint::black_box(left)), _mm_set_ss(core::hint::black_box(right)))) }
        }
        let x_delta = subtract(f32::from_bits(read_word(position)), f32::from_bits(read_word(target)));
        let y_delta = subtract(f32::from_bits(read_word(position.wrapping_add(4))), f32::from_bits(read_word(target.wrapping_add(4))));
        let z_delta = subtract(f32::from_bits(read_word(position.wrapping_add(8))), f32::from_bits(read_word(target.wrapping_add(8))));
        let y_square = multiply(y_delta, y_delta);
        let x_square = multiply(x_delta, x_delta);
        let z_square = multiply(z_delta, z_delta);
        let distance_squared = add(add(y_square, x_square), z_square);
        let near_blend_limit = unsafe { lf_checker_rt::global::<f32>(NEAR_BLEND_LIMIT_FILE_VA).read_unaligned() };
        if near_blend_limit > distance_squared {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, receiver);
        }
        0
    }
});
