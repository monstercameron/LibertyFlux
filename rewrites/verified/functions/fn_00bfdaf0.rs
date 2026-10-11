// original: 0x00bfdaf0 signed_byte_vector_to_scaled_floats_a
/// Read three signed bytes from receiver offsets 0x1c–0x1e, convert each signed value to f32, multiply it by the single-precision scale at file VA 0x00fe8700, and store three float words through the stack output pointer. Multiplication is pinned in input-then-scale order. EAX contains the sign-extended final source byte on return.
lf_checker_rt::export!(thiscall, rw_00bfdaf0(this: u32, output: u32) -> u32 {
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
                const SCALE_FILE_VA: u32 = 0x00fe8700;
                let scale = f32::from_bits(unsafe { (lf_checker_rt::global::<u32>(SCALE_FILE_VA)).read_unaligned() });
                let mut last = 0i32;
                for (index, offset) in [0x1c, 0x1d, 0x1e].into_iter().enumerate() {
                    let signed = read_u8(this, offset) as i8 as i32;
                    last = signed;
                    let scaled = mulss_ordered(signed as f32, scale);
                    write_u32(output, (index as u32) * 4, scaled.to_bits());
                }
                if false { last as u32 + 1 } else { last as u32 }
            }
        });
