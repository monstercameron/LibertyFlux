// original: 0x00bbda10 NativeImpl_TASK_COMBAT_TIMED
/// Queue a timed combat task, tail-calling the assignment.
///
/// Resolves the ped handle and allocates a task node. When the allocator
/// reports empty, assigns the empty task with kind `0x5C`; otherwise builds
/// the combat task from the ped and the time limit (converted to a float and
/// scaled by the constant factor) and assigns it with kind `0x5C`. The
/// original reaches the assignment with a tail jump that reuses its own
/// argument slots; the rewrite issues the same call and returns its answer.
export!(cdecl, rw_00bbda10(a0: u32, handle: u32, time: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtsi32_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let ped: u32 = callee_thiscall!(1, u32,
            *global::<u32>(0x18B6F1C), handle);
        let mgr: u32 = callee_thiscall!(2, u32, *global::<u32>(0x167E2A0));
        if mgr == 0 {
            return callee_cdecl!(4, u32, a0, 0, 0x5C);
        }
        let k = f32::from_bits(*global::<u32>(0xFE86B4));
        let scaled = _mm_cvtss_f32(_mm_mul_ss(
            _mm_cvtsi32_ss(_mm_set_ss(0.0), time as i32), _mm_set_ss(k)));
        let t: u32 = callee_thiscall!(3, u32, mgr, ped, scaled.to_bits());
        callee_cdecl!(4, u32, a0, t, 0x5C)
    }
});
