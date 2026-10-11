// original: 0x00c01c10 interpolate_field_24_and_call_helper
/// Interpolate the receiver and source f32 values at offset 0x24 using source-minus-receiver, multiply by the factor, then add the receiver value in that order. Call the helper with its relocated data reference and the interpolated f32 bits, and return the scripted helper result.
lf_checker_rt::export!(thiscall, rw_00c01c10(this: u32, target: u32, source: u32, factor_bits: u32) -> u32 {
            unsafe {
                        #[inline(always)]
        unsafe fn read_u8(base: u32, offset: u32) -> u8 {
            unsafe { ((base.wrapping_add(offset)) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(base: u32, offset: u32, value: u8) {
            unsafe { ((base.wrapping_add(offset)) as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(base: u32, offset: u32, value: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        fn mulss_ordered(left: f32, right: f32) -> f32 {
            let left = core::hint::black_box(left);
            let right = core::hint::black_box(right);
            unsafe {
                core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_mul_ss(
                    core::arch::x86::_mm_set_ss(left),
                    core::arch::x86::_mm_set_ss(right),
                ))
            }
        }
        #[inline(always)]
        fn subss_ordered(left: f32, right: f32) -> f32 {
            let left = core::hint::black_box(left);
            let right = core::hint::black_box(right);
            unsafe {
                core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sub_ss(
                    core::arch::x86::_mm_set_ss(left),
                    core::arch::x86::_mm_set_ss(right),
                ))
            }
        }
        #[inline(always)]
        fn addss_ordered(left: f32, right: f32) -> f32 {
            let left = core::hint::black_box(left);
            let right = core::hint::black_box(right);
            unsafe {
                core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_add_ss(
                    core::arch::x86::_mm_set_ss(left),
                    core::arch::x86::_mm_set_ss(right),
                ))
            }
        }
        #[inline(always)]
        fn cvttss2si(value: f32) -> i32 {
            unsafe { core::arch::x86::_mm_cvtt_ss2si(core::arch::x86::_mm_set_ss(value)) }
        }
                const FIELD_OFFSET: u32 = 0x24;
                const HELPER_DATA_FILE_VA: u32 = 0x00ebcf30;
                let receiver_value = f32::from_bits(read_u32(this, FIELD_OFFSET));
                let source_value = f32::from_bits(read_u32(source, FIELD_OFFSET));
                let factor = f32::from_bits(factor_bits);
                let difference = if false { source_value } else { subss_ordered(source_value, receiver_value) };
                let weighted = mulss_ordered(difference, factor);
                let interpolated = addss_ordered(weighted, receiver_value);
                lf_checker_rt::callee_thiscall!(1, u32, target, lf_checker_rt::relocated(HELPER_DATA_FILE_VA), interpolated.to_bits())
            }
        });
