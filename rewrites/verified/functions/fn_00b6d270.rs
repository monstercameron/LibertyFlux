// original: 0x00B6D270 ped_task_ctor_virtual_vector_magnitude
/// Stores an object pointer, a dword and three copied words. For a non-null object it calls the
/// reference helper, then an object virtual method at slot +0xec with a pointer to one
/// uninitialized stack word. The scripted method returns a three-float vector and the task stores
/// sqrt(x*x + y*y + z*z) at +0x20; with a null object that field stays zero. The skipped stack
/// pointer has a one-word pre-call snapshot.
lf_checker_rt::export!(thiscall, rw_00b6d270(task: u32, arg0_object: u32, arg1_value: u32, arg2_words: u32) -> u32 {
    unsafe {
        const VTABLE_FILE_VA: u32 = 0x00EB1DAC;
        const INVALID_WORD: u32 = 0xFFFF_FFFF;
        const BASE_CALLEE: u32 = 1;
        const REFERENCE_CALLEE: u32 = 2;
        const INDIRECT_CALLEE: u32 = 3;
        const FIELD_VTABLE: u32 = 0x0;
        const FIELD_14: u32 = 0x14;
        const FIELD_18: u32 = 0x18;
        const FIELD_1C: u32 = 0x1C;
        const FIELD_20: u32 = 0x20;
        const FIELD_24: u32 = 0x24;
        const FIELD_30: u32 = 0x30;
        const FIELD_34: u32 = 0x34;
        const FIELD_38: u32 = 0x38;
        #[inline(always)] unsafe fn read_word(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_word(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_half(a: u32, v: u16) { unsafe { (a as *mut u16).write_unaligned(v) } }
        #[inline(always)] unsafe fn write_byte(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        let _ = lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, task);
        unsafe { write_word(task.wrapping_add(FIELD_VTABLE), lf_checker_rt::relocated(VTABLE_FILE_VA)); }
        unsafe { write_byte(task.wrapping_add(FIELD_14), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_18), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_1C), arg0_object); }
        unsafe { write_word(task.wrapping_add(FIELD_20), 0); }
        unsafe { write_word(task.wrapping_add(FIELD_24), arg1_value); }
        let copy_30 = unsafe { read_word(arg2_words.wrapping_add(0x0)) };
        unsafe { write_word(task.wrapping_add(FIELD_30), copy_30); }
        let copy_34 = unsafe { read_word(arg2_words.wrapping_add(0x4)) };
        unsafe { write_word(task.wrapping_add(FIELD_34), copy_34); }
        let copy_38 = unsafe { read_word(arg2_words.wrapping_add(0x8)) };
        unsafe { write_word(task.wrapping_add(FIELD_38), copy_38); }
        let reference_0 = unsafe { read_word(task.wrapping_add(FIELD_1C)) };
        if reference_0 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(REFERENCE_CALLEE, u32, reference_0, task.wrapping_add(FIELD_1C));
        }
        if reference_0 != 0 {
            use core::mem::MaybeUninit;
            let output_slot = MaybeUninit::<u32>::uninit();
            let returned_vector = lf_checker_rt::callee_thiscall!(INDIRECT_CALLEE, u32, reference_0, output_slot.as_ptr() as u32);
            let x = f32::from_bits(unsafe { read_word(returned_vector) });
            let y = f32::from_bits(unsafe { read_word(returned_vector.wrapping_add(4)) });
            let z = f32::from_bits(unsafe { read_word(returned_vector.wrapping_add(8)) });
            let x2 = core::hint::black_box(x) * core::hint::black_box(x);
            let y2 = core::hint::black_box(y) * core::hint::black_box(y);
            let z2 = core::hint::black_box(z) * core::hint::black_box(z);
            let xy = core::hint::black_box(x2) + core::hint::black_box(y2);
            let sum = core::hint::black_box(xy) + core::hint::black_box(z2);
            #[target_feature(enable = "sse")]
            unsafe fn sqrt_scalar(value: f32) -> f32 {
                use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};
                _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(value)))
            }
            let magnitude = unsafe { sqrt_scalar(sum) };
            unsafe { write_word(task.wrapping_add(FIELD_20), magnitude.to_bits()); }
        }
        task
    }
});
