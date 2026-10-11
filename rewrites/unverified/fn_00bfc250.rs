// original: 0x00bfc250 initialize_physics_record_4d
/// Run the two base initializers with code 0x4d and the shared global word, then call two receiver helpers with the third and fourth stack words. Store the fifth word as raw f32 bits at offset 0x24 and the low 16 bits of the sixth word at offset 0x2c. Pack the low bits of stack words seven and eight into byte 0x33; copy stack words nine through eleven to bytes 0x30 through 0x32; copy the twelfth word's low byte to offsets 0x2e and 0x14; clear byte 0x2f; set kind 0x0b; and return the twelfth word's low byte.
lf_checker_rt::export!(thiscall, rw_00bfc250(this: u32, first_word: u32, second_word: u32, third_word: u32, fourth_word: u32, field_float_bits: u32, attribute_word: u32, low_flag: u32, upper_flag: u32, byte_30: u32, byte_31: u32, byte_32: u32, final_byte: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x4d;
            const FLOAT_FIELD_OFFSET: u32 = 0x24;
            const ATTRIBUTE_LOW_OFFSET: u32 = 0x2c;
            const ATTRIBUTE_HIGH_OFFSET: u32 = 0x2d;
            const FLAGS_OFFSET: u32 = 0x33;
            const OBJECT_KIND_OFFSET: u32 = 0x02;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, third_word);
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, fourth_word);
            write_u8(this, ATTRIBUTE_LOW_OFFSET, attribute_word as u8);
            write_u8(this, ATTRIBUTE_HIGH_OFFSET, (attribute_word >> 8) as u8);
            let low_bit = (low_flag as u8) & 1;
            let upper_bit = (upper_flag as u8) & 1;
            write_u8(this, FLAGS_OFFSET, upper_bit.wrapping_add(upper_bit) | low_bit);
            write_u32(this, FLOAT_FIELD_OFFSET, field_float_bits);
            write_u8(this, 0x30, byte_30 as u8);
            write_u8(this, 0x31, byte_31 as u8);
            write_u8(this, 0x32, byte_32 as u8);
            write_u8(this, 0x2e, final_byte as u8);
            write_u8(this, 0x14, final_byte as u8);
            write_u8(this, 0x2f, 0);
            write_u8(this, OBJECT_KIND_OFFSET, 0x0b);
            if false { (final_byte as u8).wrapping_add(1) as u32 } else { final_byte as u8 as u32 }
        }
    });
