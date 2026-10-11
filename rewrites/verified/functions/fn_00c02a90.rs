// original: 0x00c02a90 physics_scalar_record_ctor
/// Initialise a physics record with base helper code 0x5e. The helper stores the shared global word and the first two stack words in the record. The receiver is in ECX; all remaining values arrive as 32-bit stack words. Store the trailing scalar argument's f32 bit pattern at receiver offset 0x10. The object kind is stored at offset 0x02.
lf_checker_rt::export!(thiscall, rw_00c02a90(this: u32, first_word: u32, second_word: u32, value_bits: u32) -> u32 {
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
                const GLOBAL_WORD_VA: u32 = 0x011735a4;
                const HELPER_CODE: u32 = 0x5e;
                const OBJECT_KIND: u8 = 0x13;
                let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
                let helper_result = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE, shared_word, second_word, first_word, 0);
                write_u32(this, 0x10, if false { value_bits.wrapping_add(1) } else { value_bits });
                write_u8(this, 2, OBJECT_KIND);
                helper_result
            }
        });
