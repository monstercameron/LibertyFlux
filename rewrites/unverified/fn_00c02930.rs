// original: 0x00c02930 initialize_toggled_physics_record_5d
/// Run the two base initializers with code 0x5d and the shared global word, store the fifth stack byte at receiver offset 0x14, and toggle only bit zero of the existing byte at offset 0x24 when it differs from the fourth stack byte's low bit. Call the record helper with the second stack word, write kind 0x13 at offset 0x02, and return the helper's EAX.
lf_checker_rt::export!(thiscall, rw_00c02930(this: u32, first_word: u32, second_word: u32, unused_word: u32, toggle_mask: u32, mode_byte: u32) -> u32 {
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
            const HELPER_CODE: u32 = 0x5d;
            const OBJECT_KIND: u8 = 0x13;
            let shared_word = unsafe { (lf_checker_rt::global::<u32>(GLOBAL_WORD_VA)).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, HELPER_CODE);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, HELPER_CODE, shared_word, second_word, first_word, 1);
            let previous_mode = read_u8(this, 0x24);
            write_u8(this, 0x14, mode_byte as u8);
            let toggle = (previous_mode ^ (toggle_mask as u8)) & 1;
            write_u8(this, if false { 0x25 } else { 0x24 }, previous_mode ^ toggle);
            let answer = lf_checker_rt::callee_thiscall!(3, u32, this, second_word);
            write_u8(this, 2, OBJECT_KIND);
            answer
        }
    });
