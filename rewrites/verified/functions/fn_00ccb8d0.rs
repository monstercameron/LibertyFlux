// original: 0x00ccb8d0 CTaskComplexHitResponse::vf19
/// Dispatch the hit-response start by the kind at `this+0x14`.
///
/// Kinds 0 to 3 select one of four handler pairs (the original switches
/// through a relocated jump table); any other kind returns 0. The chosen
/// pair fetches the manager handle (thiscall on the global pointer),
/// returns 0 when null, and otherwise runs its case handler (thiscall on
/// the handle), returning that answer. The single stack argument is unused.
/// Thiscall.
export!(thiscall, rw_00ccb8d0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        const KIND_OFF: u32 = 0x14;
        let kind = (this.wrapping_add(KIND_OFF) as *const u32).read_unaligned();
        if kind > 3 {
            return 0;
        }
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(1, u32, mgr);
        if h == 0 {
            return 0;
        }
        match kind {
            0 => callee_thiscall!(2, u32, h),
            1 => callee_thiscall!(3, u32, h),
            2 => callee_thiscall!(4, u32, h),
            _ => callee_thiscall!(5, u32, h),
        }
    }
});
