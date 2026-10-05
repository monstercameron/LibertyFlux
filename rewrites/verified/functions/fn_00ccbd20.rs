// original: 0x00ccbd20 CTaskComplexOnFire::vf19
/// Start the on-fire task for a ped, arming its timers.
///
/// Stamps the global timer into `this+0x14`. When the ped's current task
/// (`ped+0x38`) is null or differs from the recorded one (`ped+0x7b4`), the
/// fire check runs (cdecl, the ped, 3, 0, 0): a zero answer ends the call
/// through the 0x38f setter (thiscall on `this`). Otherwise, and when the
/// tasks already match, a mismatch also runs the ignite helper (thiscall
/// on the ped, 1, 1) before the 0x83f setter (thiscall on `this`) ends the
/// call. Returns the last helper's answer. Thiscall, ped argument.
export!(thiscall, rw_00ccbd20(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER_G: u32 = 0x011735b4;
        const STAMP_OFF: u32 = 0x14;
        const CUR_OFF: u32 = 0x38;
        const REC_OFF: u32 = 0x7b4;
        const KIND_A: u32 = 0x38f;
        const KIND_B: u32 = 0x83f;
        (this.wrapping_add(STAMP_OFF) as *mut u32).write_unaligned(*global::<u32>(TIMER_G));
        let cur = (ped.wrapping_add(CUR_OFF) as *const u32).read_unaligned();
        let rec = (ped.wrapping_add(REC_OFF) as *const u32).read_unaligned();
        if cur == 0 || cur != rec {
            let ok: u32 = callee_cdecl!(1, u32, ped, 3u32, 0u32, 0u32);
            if ok & 0xff == 0 {
                return callee_thiscall!(2, u32, this, ped, KIND_A);
            }
        }
        let cur2 = (ped.wrapping_add(CUR_OFF) as *const u32).read_unaligned();
        let rec2 = (ped.wrapping_add(REC_OFF) as *const u32).read_unaligned();
        if cur2 == 0 || cur2 != rec2 {
            let _: u32 = callee_thiscall!(3, u32, ped, 1u32, 1u32);
        }
        callee_thiscall!(4, u32, this, ped, KIND_B)
    }
});
