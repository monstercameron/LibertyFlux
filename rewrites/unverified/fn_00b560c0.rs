// original: 0x00B560C0 crmtManagerPriority_build_scaled_output

/// Builds a four-word output at `out`. It first copies three static basis
/// floats, then calls the transform helper with `source`, the product of the
/// `f32` at `source + 0x0C` and `input_scale`, the source pointer again, a
/// four-word temporary, and the constants zero and one. The helper's four
/// output words replace all four output words. Float multiplication keeps the
/// original operand order. A final CRT cookie check is made before returning
/// `out`. The three static values are read through the relocated image base.
/// This is a cdecl function with three stack arguments.
lf_checker_rt::export!(cdecl, rw_00b560c0(out: u32, input_scale: f32, source: u32) -> u32 {
    unsafe {
        const SOURCE_SCALE: u32 = 0x0c;
        const HELPER: u32 = 1;
        const COOKIE_CHECK: u32 = 2;
        const STATIC_BASIS: u32 = 0x01b4b2a0;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 { f32::from_bits(unsafe { read_u32(address) }) }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let basis = lf_checker_rt::global::<u32>(STATIC_BASIS);
        for index in 0..3u32 {
            let value = unsafe { basis.add(index as usize).read_unaligned() };
            unsafe { write_u32(out.wrapping_add(index * 4), value) };
        }
        let scaled = mul(unsafe { read_f32(source.wrapping_add(SOURCE_SCALE)) }, input_scale);
        let source_word = source;
        let mut helper_values = [0x00e9_5580u32, 0x0000_0510u32, 0, 0];
        let _ = lf_checker_rt::callee_thiscall!(
            HELPER, u32, source_word, scaled.to_bits(), source_word,
            helper_values.as_mut_ptr() as u32, 0, 1
        );
        for index in 0..4u32 {
            unsafe { write_u32(out.wrapping_add(index * 4), helper_values[index as usize]) };
        }
        let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, 0);
        out
    }
});
