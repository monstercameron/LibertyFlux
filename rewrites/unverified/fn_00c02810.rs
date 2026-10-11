// original: 0x00c02810 initialize_selector_physics_record_61
/// Run the two base initializers with code 0x61 and the shared global word, then call the record helper with the third stack word. Store the fourth stack word's raw f32 bits at offset 0x28. Build receiver byte 0x25 from the selector low bit, an adjusted low nibble, the prior byte's high two bits and the seventh stack word's low bit. The adjustment is +0xf2 for selector zero, +0xed for selector one, and unchanged otherwise; store the final nibble plus 0x0e or 0x13 at offset 0x14 according to bit 0x20 of the packed byte, then set kind 0x13 at offset 0x02 and return the final byte.
lf_checker_rt::export!(thiscall, rw_00c02810(this: u32, first_word: u32, second_word: u32, tertiary_word: u32, field_float_bits: u32, selector: u32, offset_nibble: u32, trailing_bit: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x61;
            const OBJECT_KIND: u8 = 0x13;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, tertiary_word);
            let selector_byte = selector as u8;
            let mut adjusted = offset_nibble as u8;
            if selector_byte == 0 {
                adjusted = adjusted.wrapping_add(0xf2);
            } else if selector_byte == 1 {
                adjusted = adjusted.wrapping_add(0xed);
            }
            let mut packed = ((selector_byte & 1) << 4) | (adjusted & 0x0f);
            packed = packed.wrapping_add(packed) | (read_u8(this, 0x25) & 0xc0) | ((trailing_bit as u8) & 1);
            write_u32(this, 0x28, field_float_bits);
            write_u8(this, 0x25, packed);
            let low_nibble = (packed >> 1) & 0x0f;
            let final_byte = if packed & 0x20 == 0 { low_nibble.wrapping_add(0x0e) } else { low_nibble.wrapping_add(0x13) };
            write_u8(this, 0x14, if false { final_byte.wrapping_add(1) } else { final_byte });
            write_u8(this, 2, OBJECT_KIND);
            final_byte as u32
        }
    });
