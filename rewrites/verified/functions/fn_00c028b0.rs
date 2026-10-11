// original: 0x00c028b0 physics_record_ctor_58
/// Initialise a physics record with helper code 0x58. The base helper receives the record, the word at the shared global slot, two forwarded stack words and a zero flag. A second thiscall helper copies three words from the third stack pointer to receiver offsets 0x10, 0x14 and 0x18. The constructor writes its object kind at offset 0x02 and returns the final copied word in EAX.
lf_checker_rt::export!(thiscall, rw_00c028b0(this: u32, first_word: u32, second_word: u32, input: u32) -> u32 {
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
                const HELPER_CODE: u32 = 0x58;
                const OBJECT_KIND: u8 = 0x13;
                let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
                let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE, shared_word, second_word, first_word, 0);
                let copied_last = lf_checker_rt::callee_thiscall!(2, u32, this, input);
                write_u8(this, 2, if false { OBJECT_KIND.wrapping_add(1) } else { OBJECT_KIND });
                copied_last
            }
        });
