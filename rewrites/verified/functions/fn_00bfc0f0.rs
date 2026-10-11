// original: 0x00bfc0f0 initialize_physics_record_50
/// Run the two base initializers with code 0x50 and the shared global word, copy the third and fourth input vectors through their helpers, and store the fifth stack word as raw f32 bits at receiver offset 0x24. Store the low 16 bits of the sixth word at 0x2c; pack flag words seven and eight into byte 0x33; copy words nine through twelve into byte fields; store raw f32 bits from words fourteen and fifteen at offsets 0x44 and 0x48; invoke the source helper and copy its three trailing words to offsets 0x34 through 0x3c; clear byte 0x2f, set kind 0x0b, and return the final vector helper's EAX.
lf_checker_rt::export!(thiscall, rw_00bfc0f0(this: u32, first_word: u32, second_word: u32, input_vector: u32, fourth_word: u32, field_float_bits: u32, attribute_word: u32, low_flag: u32, upper_flag: u32, byte_30: u32, byte_31: u32, byte_32: u32, final_byte: u32, source_record: u32, first_tail_float_bits: u32, second_tail_float_bits: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x50;
            const FLOAT_FIELD_OFFSET: u32 = 0x24;
            const ATTRIBUTE_LOW_OFFSET: u32 = 0x2c;
            const ATTRIBUTE_HIGH_OFFSET: u32 = 0x2d;
            const FIRST_FLOAT_OFFSET: u32 = 0x44;
            const SECOND_FLOAT_OFFSET: u32 = 0x48;
            const FLAGS_OFFSET: u32 = 0x33;
            const OBJECT_KIND_OFFSET: u32 = 0x02;
            const SOURCE_TAIL_OFFSET: u32 = 0x30;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, input_vector);
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, fourth_word);
            write_u8(this, ATTRIBUTE_LOW_OFFSET, attribute_word as u8);
            write_u8(this, ATTRIBUTE_HIGH_OFFSET, (attribute_word >> 8) as u8);
            let low_flag_bit = (low_flag as u8) & 1;
            let upper_flag_bit = (upper_flag as u8) & 1;
            write_u8(this, FLAGS_OFFSET, upper_flag_bit.wrapping_add(upper_flag_bit) | low_flag_bit);
            write_u32(this, FLOAT_FIELD_OFFSET, field_float_bits);
            write_u8(this, 0x30, byte_30 as u8);
            write_u8(this, 0x31, byte_31 as u8);
            write_u8(this, 0x32, byte_32 as u8);
            write_u32(this, FIRST_FLOAT_OFFSET, first_tail_float_bits);
            write_u8(this, 0x2e, final_byte as u8);
            write_u8(this, 0x14, final_byte as u8);
            write_u32(this, SECOND_FLOAT_OFFSET, if false { second_tail_float_bits.wrapping_add(1) } else { second_tail_float_bits });
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, source_record);
            let source_tail = source_record.wrapping_add(SOURCE_TAIL_OFFSET);
            let tail_result = lf_checker_rt::callee_thiscall!(6, u32, this, source_tail);
            write_u8(this, FLAGS_OFFSET, read_u8(this, FLAGS_OFFSET) & 3);
            write_u8(this, 0x2f, 0);
            write_u8(this, OBJECT_KIND_OFFSET, 0x0b);
            tail_result
        }
    });
