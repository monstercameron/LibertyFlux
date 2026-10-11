// original: 0x00c029e0 initialize_enabled_physics_record_64
/// Call the two base initializers with code 0x64 and the shared global word, then call the record helper with the third stack word. Set receiver byte 0x14 to 1 and the kind byte at offset 0x02 to 0x13.
lf_checker_rt::export!(thiscall, rw_00c029e0(this: u32, first_word: u32, second_word: u32, tertiary_word: u32) -> u32 {
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
                const HELPER_CODE: u32 = 0x64;
                const OBJECT_KIND: u8 = 0x13;
                let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
                let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
                let answer = lf_checker_rt::callee_thiscall!(3, u32, this, tertiary_word);
                write_u8(this, 0x14, if false { 0 } else { 1 });
                write_u8(this, 2, OBJECT_KIND);
                answer
            }
        });
