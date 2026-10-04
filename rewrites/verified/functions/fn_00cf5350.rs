// original: 0x00cf5350 climb_task_vector_blend (proposed)

/// Blends a climb task's vector input: fetches the drive float for the sixth
/// argument through the drive helper (which writes it to a stack slot whose
/// address the comparison skips while the float itself is scripted), forms
/// `(drive - *out) * gain` with the gain at `+0x70` in that order, commits
/// all eleven inputs through the blend callee, stores the drive float to the
/// output word, and returns the commit's result.
///
/// Arguments: (out0, out1, base, key, tag, drive_in, out, extra, v2, v3, v4).
///
/// Original: 0x00cf5350 (thiscall: ecx holds the object, eleven stack words).
lf_checker_rt::export!(thiscall, rw_00cf5350(
    this: u32, a8: u32, a_c: u32, a10: u32, a14: u32, a18: u32,
    a1c: u32, a20: u32, a24: u32, a28: u32, a2c: u32, a30: u32,
) -> u32 {
    unsafe {
        const DRIVE_CALLEE: u32 = 1;
        const BLEND_CALLEE: u32 = 2;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        // The drive helper's out-slot; its address is skipped, its content
        // scripted, so the rewrite reads the stub-written word directly.
        let mut slot: u32 = 0;
        lf_checker_rt::callee_cdecl!(
            DRIVE_CALLEE, u32, &mut slot as *mut u32 as u32, a1c, a14, a18);
        let drive = f32::from_bits(slot);
        let old = f32::from_bits((a20 as *const u32).read_unaligned());
        let gain = f32::from_bits(((this + 0x70) as *const u32).read_unaligned());
        let mixed = mul(sub(drive, old), gain);
        let out = lf_checker_rt::callee_thiscall!(
            BLEND_CALLEE, u32, this, a8, a_c, a10, mixed.to_bits(), a24, a28, a2c, a30);
        (a20 as *mut u32).write_unaligned(drive.to_bits());
        out
    }
});
