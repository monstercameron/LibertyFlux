// original: 0x009B7510 NativeImpl_GET_CAM_ROT
/// Read the camera rotation and scale it to degrees.
///
/// Resolves the rotation entry for `a0`, copies its first word to `out`,
/// multiplies the first three words by the radians-to-degrees constant and
/// writes them over `out`, and writes the frame's leftover word (zero under
/// the checker's stack fill) at `out + 12`. Returns the first word.
/// stdcall, selector and out-pointer.
lf_checker_rt::export!(stdcall, rw_009B7510(a0: u32, out: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00E93F0C;
        const DEG_PER_RAD_SLOT: u32 = 0x00E7C2A8;
        const ENTRY_INNER: u32 = 0x10;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let mut slot = 0u32;
        let r: u32 = lf_checker_rt::callee_stdcall!(1, u32, a0, (&mut slot as *mut u32) as u32, lf_checker_rt::relocated(VTABLE_WORD));
        let e: u32 = lf_checker_rt::callee_thiscall!(2, u32, r.wrapping_add(ENTRY_INNER));
        let w0 = (e as *const u32).read_unaligned();
        let w1 = ((e + 4) as *const u32).read_unaligned();
        let w2 = ((e + 8) as *const u32).read_unaligned();
        let k = f32::from_bits((lf_checker_rt::global::<u32>(DEG_PER_RAD_SLOT) as *const u32).read_unaligned());
        (out as *mut u32).write_unaligned(w0);
        (out as *mut f32).write_unaligned(mul(f32::from_bits(w0), k));
        ((out + 4) as *mut f32).write_unaligned(mul(f32::from_bits(w1), k));
        ((out + 8) as *mut f32).write_unaligned(mul(f32::from_bits(w2), k));
        ((out + 12) as *mut u32).write_unaligned(0);
        w0
    }
});
