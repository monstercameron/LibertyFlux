// original: 0x00bfd090 copy_physics_record_fields
/// Ask the source helper to fill one stack-local word and pass a pointer to that word to the conversion helper. Store the conversion helper's EAX at receiver offset 0x18. Copy the source's raw words at offsets 0x30, 0x34 and 0x38 to receiver offsets 0x1c, 0x20 and 0x24, then return the last copied word.
lf_checker_rt::export!(thiscall, rw_00bfd090(this: u32, source: u32) -> u32 {
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
            const SCRATCH_WORDS: usize = 1;
            const CONVERTED_FIELD_OFFSET: u32 = 0x18;
            const SOURCE_FIRST_OFFSET: u32 = 0x30;
            const SOURCE_SECOND_OFFSET: u32 = 0x34;
            const SOURCE_THIRD_OFFSET: u32 = 0x38;
            const DEST_FIRST_OFFSET: u32 = 0x1c;
            const DEST_SECOND_OFFSET: u32 = 0x20;
            const DEST_THIRD_OFFSET: u32 = 0x24;
            let mut scratch_words = [0u32; SCRATCH_WORDS];
            let scratch_pointer = scratch_words.as_mut_ptr() as usize as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, source, scratch_pointer);
            let converted_value = lf_checker_rt::callee_cdecl!(2, u32, scratch_pointer);
            write_u32(this, CONVERTED_FIELD_OFFSET, converted_value);
            let first_source_word = read_u32(source, SOURCE_FIRST_OFFSET);
            let second_source_word = read_u32(source, SOURCE_SECOND_OFFSET);
            let third_source_word = read_u32(source, SOURCE_THIRD_OFFSET);
            write_u32(this, DEST_FIRST_OFFSET, first_source_word);
            write_u32(this, DEST_SECOND_OFFSET, second_source_word);
            write_u32(this, if false { 0x28 } else { DEST_THIRD_OFFSET }, third_source_word);
            third_source_word
        }
    });
