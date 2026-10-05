// original: 0x00cce060 CTaskComplexInjuredOnGround::vf5
/// Run the injured-on-ground shutdown, returning 1 on success, else 0.
///
/// When the second stack argument is 2, the animation slot is driven with
/// -1000.0 (thiscall on `this`), the ped helper runs (thiscall on the
/// first argument, 1), and the subtask at `this+8`, unless null or flagged
/// at `+0xc`, shuts down through its slot at `+0x14` (thiscall, first and
/// third arguments around a 2); the call returns 1 either way. Otherwise
/// the third argument must be non-null and answer one of 0xa, 9, 9, 0x78,
/// 0x12 across up to five asks through its slot at `+4`, and the subtask,
/// unless null or flagged, must shut down through the same shared slot
/// (thiscall, all three stack arguments) with a non-zero answer; then the
/// slot is driven with -50.0 and the call returns 1, else 0 in AL.
/// Thiscall with three stack arguments.
export!(thiscall, rw_00cce060(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x0c;
        const VT_SLOT: u32 = 0x14;
        const DONE_BIT: u32 = 2;
        const HOT_F: u32 = 0xc47a0000;
        const COLD_F: u32 = 0xc1000000;
        if a1 == 2 {
            let _: u32 = callee_thiscall!(1, u32, this, HOT_F);
            let _: u32 = callee_thiscall!(2, u32, a0, 1u32);
            let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
            if sub != 0 && (sub.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
                let vt = (sub as *const u32).read_unaligned();
                let s = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(s as usize);
                if f(sub, a0, 2u32, a2) & 0xff != 0 {
                    let fl = (sub.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
                    (sub.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
                }
            }
            return 1;
        }
        if a2 == 0 {
            return 0;
        }
        let vt4 = (a2 as *const u32).read_unaligned();
        let s4 = (vt4.wrapping_add(4) as *const u32).read_unaligned();
        let f4: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(s4 as usize);
        if f4(a2) != 0x0a
            && f4(a2) != 9
            && f4(a2) != 9
            && f4(a2) != 0x78
            && f4(a2) != 0x12
        {
            return 0;
        }
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if sub != 0 && (sub.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let s = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(s as usize);
            if f(sub, a0, a1, a2) & 0xff == 0 {
                return 0;
            }
            let fl = (sub.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
            (sub.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
        }
        let _: u32 = callee_thiscall!(6, u32, this, COLD_F);
        1
    }
});
