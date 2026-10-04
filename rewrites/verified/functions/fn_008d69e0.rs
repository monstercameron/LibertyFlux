// original: 0x008d69e0 resolve_active_or_default
// Resolves the active record through a readiness gate, a flag byte and a
// positive count; any failure falls back to the default resolver.
export!(cdecl, rw_008d69e0() -> u32 {
    unsafe {
        let ready: u32 = callee_cdecl!(1, u32,);
        if ready & 0xFF == 0 {
            return callee_cdecl!(3, u32, 0);
        }
        if *global::<u8>(0x017F_5EB3) == 0 {
            return callee_cdecl!(3, u32, 0);
        }
        let count = *global::<i32>(0x0103_E4B8);
        if count <= 0 {
            return callee_cdecl!(3, u32, 0);
        }
        let ctx = *global::<u32>(0x018B_6F1C);
        let dev: u32 = callee_thiscall!(2, u32, ctx, count as u32);
        if dev == 0 {
            return callee_cdecl!(3, u32, 0);
        }
        dev
    }
});
