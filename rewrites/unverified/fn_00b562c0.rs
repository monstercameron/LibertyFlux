// original: 0x00B562C0 crmtManagerPriority_build_scaled_output_from_tag_extended_basis

/// Builds a four-word output for a tagged source. It copies three static
/// basis floats, asks the lookup helper for the source associated with
/// `source_tag`, and, when the helper returns a pointer, calls the transform
/// helper with that pointer in ECX. The transform receives the product of the
/// selected object's `f32` at offset `0x0C` and `input_scale`, a four-word
/// temporary, zero, and one. The helper's four words replace the output. A
/// CRT cookie check runs before returning `out`. This is a cdecl function
/// with four stack arguments.
lf_checker_rt::export!(cdecl, rw_00b562c0(out: u32, input_scale: f32, source: u32, source_tag: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const HELPER: u32 = 2;
        const COOKIE_CHECK: u32 = 3;
        const SOURCE_SCALE: u32 = 0x0c;
        const STATIC_BASIS: u32 = 0x011100d0;
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
        let selected = lf_checker_rt::callee_cdecl!(LOOKUP, u32, source, source_tag);
        if selected != 0 {
            let scaled = mul(unsafe { read_f32(selected.wrapping_add(SOURCE_SCALE)) }, input_scale);
            let mut helper_values = [0x00e9_55bcu32, 0x0000_0611u32, 0, 0, 0, 0, 0, 0];
            let _ = lf_checker_rt::callee_thiscall!(
                HELPER, u32, selected, scaled.to_bits(),
                helper_values.as_mut_ptr() as u32, 0, 1
            );
            for index in 0..4u32 {
                unsafe { write_u32(out.wrapping_add(index * 4), helper_values[index as usize + 4]) };
            }
        }
        let _ = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, 0);
        out
    }
});
