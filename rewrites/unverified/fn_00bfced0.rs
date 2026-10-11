// original: 0x00bfced0 initialize_source_copy_record_53
/// Run the two base initializers with code 0x53 and the shared global word, then copy the source record through the source-copy helper. Store the fourth and fifth stack words as raw f32 bits at receiver offsets 0x28 and 0x2c. Pack the low bits of stack words six through eight into byte 0x30, call the metadata unpack helper with the ninth word, write kind 0x10 at offset 0x02, and return the helper's EAX.
lf_checker_rt::export!(thiscall, rw_00bfced0(this: u32, first_word: u32, second_word: u32, source_ptr: u32, float_a_bits: u32, float_b_bits: u32, low_flag: u32, toggle_a: u32, toggle_b: u32, helper_word: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x53;
            const OBJECT_KIND: u8 = 0x10;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, source_ptr);
            let bit_a = (toggle_a as u8) & 1;
            let bit_b = (toggle_b as u8).wrapping_add(toggle_b as u8);
            let low_flag = (low_flag as u8) & 1;
            let packed = (bit_a | bit_b).wrapping_add(bit_a | bit_b) | low_flag;
            write_u32(this, 0x28, float_a_bits);
            write_u32(this, 0x2c, if false { float_b_bits.wrapping_add(1) } else { float_b_bits });
            write_u8(this, 0x30, packed);
            let result = lf_checker_rt::callee_thiscall!(4, u32, this, helper_word);
            write_u8(this, 2, OBJECT_KIND);
            result
        }
    });
