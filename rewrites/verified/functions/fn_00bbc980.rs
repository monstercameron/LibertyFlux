// original: 0x00bbc980 NativeImpl_ATTACH_PED_TO_SHIMMY_EDGE
/// Attach a ped to a shimmy edge and queue the shimmy task.
///
/// Returns the state pointer at once when the ped is busy. Otherwise tunes
/// the ped through its own function table (edge setup over the coordinate
/// record, then two weight applications of the scaled width), allocates a
/// task node and, unless the allocator reports empty, builds the shimmy task
/// and assigns it with kind `0x3D`. Returns the assign call's answer.
export!(cdecl, rw_00bbc980(handle: u32, f0: u32, f1: u32, f2: u32, w: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let pool = *global::<u32>(0x18B6F1C);
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32, pool, handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let ped: u32 = callee_thiscall!(2, u32, pool, handle);
        let k = f32::from_bits(*global::<u32>(0xFE8728));
        let scaled = _mm_cvtss_f32(_mm_mul_ss(
            _mm_set_ss(f32::from_bits(w)), _mm_set_ss(k))).to_bits();
        let mut coords = [f0, f1, f2];
        let vtab = *(ped as *const u32);
        let setup: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((vtab + 8) as *const u32));
        let _: u32 = setup(ped, coords.as_mut_ptr() as u32, 0, 0);
        let weigh: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtab + 0xC) as *const u32));
        let _: u32 = weigh(ped, scaled);
        let _: u32 = callee_thiscall!(5, u32, ped, scaled);
        let _: u32 = callee_thiscall!(6, u32, ped, scaled);
        let mgr: u32 = callee_thiscall!(7, u32, *global::<u32>(0x167E2A0));
        if mgr == 0 {
            return callee_cdecl!(9, u32, handle, 0, 0x3D);
        }
        let t: u32 = callee_thiscall!(8, u32, mgr, 0x80, 0);
        callee_cdecl!(9, u32, handle, t, 0x3D)
    }
});
