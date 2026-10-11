// original: 0x00bfc530 scaled_float_vector_to_signed_bytes_b
/// Read three f32 words through the stack input pointer, multiply each by the scale at file VA 0x00fe8bc4 in input-then-scale order, truncate toward zero with the x86 scalar conversion, and store each low byte at receiver offsets 0x20–0x22. EAX retains the final signed conversion result.
lf_checker_rt::export!(thiscall, rw_00bfc530(this: u32, input: u32) -> u32 {
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
                const SCALE_FILE_VA: u32 = 0x00fe8bc4;
                let scale = f32::from_bits(unsafe { (lf_checker_rt::global::<u32>(SCALE_FILE_VA)).read_unaligned() });
                let mut last = 0i32;
                for (index, offset) in [0x20, 0x21, 0x22].into_iter().enumerate() {
                    let value = f32::from_bits(read_u32(input, (index as u32) * 4));
                    let converted = cvttss2si(mulss_ordered(value, scale));
                    last = converted;
                    write_u8(this, offset, if false && index == 2 { (converted as u8).wrapping_add(1) } else { converted as u8 });
                }
                last as u32
            }
        });
