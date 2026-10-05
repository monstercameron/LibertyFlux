// original: 0x00ccb4a0 CTaskComplexOnFire::vf20
/// Advance the on-fire task, returning the follow-up's answer or the subtask.
///
/// Fires the ignite helper (thiscall on the fixed manager address, the ped
/// argument) when the ped's flag at `+0x118` is set, or its byte at `+0x219`
/// is non-zero while the global timer still exceeds `[this+0x14]+0xbb8`.
/// Then the pair check runs (cdecl, ped and `[this+0x18]`), the subtask at
/// `this+8` must answer 0x38f through its slot at `+0xc`, and either bit 21
/// of `[ped+0x28]` is clear or the pair check answered non-zero. The
/// subtask, unless its flag byte at `+0xc` is already set, shuts down
/// through its slot at `+0x14` (thiscall, ped, 1, 0); a zero answer, like
/// any earlier miss, returns the subtask, otherwise bit 1 of the flag is
/// set and the follow-up runs through thiscall slot `+0x48` on `this`.
/// Thiscall with one stack argument.
export!(thiscall, rw_00ccb4a0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER_G: u32 = 0x011735b4;
        const MGR_C: u32 = 0x012e2420;
        const SUB_OFF: u32 = 0x08;
        const VT_SLOT: u32 = 0x0c;
        const VT_SLOT2: u32 = 0x14;
        const FIN_SLOT: u32 = 0x48;
        const FLAG_OFF: u32 = 0x0c;
        const DONE_BIT: u32 = 2;
        const STAMP_OFF: u32 = 0x14;
        const GRACE: u32 = 0xbb8;
        const WANT: u32 = 0x38f;
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        let f1 = (ped.wrapping_add(0x118) as *const u8).read() & 1 != 0;
        let mut fire = f1;
        if !fire && (ped.wrapping_add(0x219) as *const u8).read() != 0 {
            let stamp = (this.wrapping_add(STAMP_OFF) as *const u32).read_unaligned();
            let now = *global::<u32>(TIMER_G);
            if now > stamp.wrapping_add(GRACE) {
                fire = true;
            }
        }
        if fire {
            let _: u32 = callee_thiscall!(1, u32, relocated(MGR_C), ped);
        }
        let w18 = (this.wrapping_add(0x18) as *const u32).read_unaligned();
        let ok: u32 = callee_cdecl!(2, u32, ped, w18);
        let vt = (sub as *const u32).read_unaligned();
        let s = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s as usize);
        if f(sub) != WANT {
            return sub;
        }
        let b21 = (ped.wrapping_add(0x28) as *const u32).read_unaligned() >> 0x15 & 1;
        if b21 != 0 && ok & 0xff == 0 {
            return sub;
        }
        if (sub.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt2 = (sub as *const u32).read_unaligned();
            let s2 = (vt2.wrapping_add(VT_SLOT2) as *const u32).read_unaligned();
            let f2: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(s2 as usize);
            if f2(sub, ped, 1u32, 0u32) & 0xff == 0 {
                return sub;
            }
            let fl = (sub.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
            (sub.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
        }
        let vtt = (this as *const u32).read_unaligned();
        let st = (vtt.wrapping_add(FIN_SLOT) as *const u32).read_unaligned();
        let ft: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(st as usize);
        ft(this, ped)
    }
});
