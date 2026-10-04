// original: 0x00bbc8c0 NativeImpl_TASK_AIM_GUN_AT_COORD
/// Aim a ped's gun at a coordinate for a duration.
///
/// Returns the state pointer at once when the ped is busy, exactly like the
/// other task natives. Otherwise allocates a task node; when the allocator
/// reports empty, assigns the empty task with kind `0x23`. When a node is
/// available, builds the aim task from the duration (converted to a float and
/// scaled by the constant factor), the unit weight and the first coordinate
/// passed by address, and assigns it with kind `0x23`. Returns the assign
/// call's answer.
export!(cdecl, rw_00bbc8c0(handle: u32, f1: u32, _f2: u32, _f3: u32, dur: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtsi32_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32,
                *global::<u32>(0x18B6F1C), handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let mgr: u32 = callee_thiscall!(2, u32, *global::<u32>(0x167E2A0));
        if mgr == 0 {
            return callee_cdecl!(4, u32, handle, 0, 0x23);
        }
        let k = f32::from_bits(*global::<u32>(0xFE86B4));
        let scaled = _mm_cvtss_f32(_mm_mul_ss(
            _mm_cvtsi32_ss(_mm_set_ss(0.0), dur as i32), _mm_set_ss(k)));
        let mut coord0 = f1;
        let t: u32 = callee_thiscall!(3, u32, mgr, 2, 0,
            &mut coord0 as *mut u32 as u32, scaled.to_bits(), 0, 1, 1, 0x3F800000u32);
        callee_cdecl!(4, u32, handle, t, 0x23)
    }
});
