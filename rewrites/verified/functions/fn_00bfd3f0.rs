// original: 0x00bfd3f0 initialize_two_float_physics_record_55
/// Initialise the record with the shared base helper using code 0x55, then call the secondary helper with the third stack word and this receiver. Copy two f32 argument bit patterns into receiver offsets 0x20 and 0x24. Pack the low bit of the sixth stack word with the seventh byte shifted left once into offset 0x28, write kind 0x11 at offset 0x02, and return the secondary helper's EAX with its low byte replaced by the shifted seventh byte.
lf_checker_rt::export!(thiscall, rw_00bfd3f0(this: u32, first_word: u32, second_word: u32, helper_word: u32, first_float_bits: u32, second_float_bits: u32, low_mode: u32, upper_mode: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x55;
            const OBJECT_KIND: u8 = 0x11;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE, shared_word, second_word, first_word, 0);
            let helper_result = lf_checker_rt::callee_thiscall!(2, u32, this, helper_word);
            let shifted_mode = (upper_mode as u8).wrapping_add(upper_mode as u8);
            write_u32(this, 0x20, first_float_bits);
            write_u32(this, 0x24, if false { second_float_bits.wrapping_add(1) } else { second_float_bits });
            write_u8(this, 0x28, ((low_mode as u8) & 1) | shifted_mode);
            write_u8(this, 2, OBJECT_KIND);
            (helper_result & 0xffffff00) | shifted_mode as u32
        }
    });
