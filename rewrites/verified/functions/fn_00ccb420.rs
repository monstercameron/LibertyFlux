// original: 0x00ccb420 CTaskComplexMoveAboutInjured::vf20
/// Advance the injured-move timer, returning 0 after the commit else the subtask.
///
/// Runs the ped helper (thiscall on the ped argument, one zero), then the
/// ped's virtual slot at `+0x14` (thiscall, the ped): a non-zero answer
/// reseeds `this+0x18` with a tiny positive constant. Unless that word is
/// strictly positive (ordered), or it minus the global at 0x011735bc stays
/// strictly positive, the call ends returning the subtask at `this+8`.
/// Otherwise the subtask, unless its flag byte at `+0xc` is already set,
/// runs through its slot at `+0x14` (thiscall, ped, 2, 0); a zero answer
/// ends the call as above, else bit 1 of the flag is set, the timer commit
/// runs (thiscall on `this`, the ped) and 0 is returned. Thiscall, ped arg.
export!(thiscall, rw_00ccb420(this: u32, ped: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        const TIMER_G: u32 = 0x011735bc;
        const SLOT_OFF: u32 = 0x18;
        const SUB_OFF: u32 = 0x08;
        const VT_SLOT: u32 = 0x14;
        const FLAG_OFF: u32 = 0x0c;
        const DONE_BIT: u32 = 2;
        const SEED: u32 = 0x358637bd;
        const PED_VT_OFF: u32 = 0x224;
        let _: u32 = callee_thiscall!(1, u32, ped, 0u32);
        let w = (ped.wrapping_add(PED_VT_OFF) as *const u32).read_unaligned();
        let vt = (w as *const u32).read_unaligned();
        let s = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(s as usize);
        if f2(w, ped) != 0 {
            (this.wrapping_add(SLOT_OFF) as *mut u32).write_unaligned(SEED);
        }
        let x = f32::from_bits((this.wrapping_add(SLOT_OFF) as *const u32).read_unaligned());
        if !(x > 0.0) {
            return (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        }
        let g = f32::from_bits(*global::<u32>(TIMER_G));
        let x2 = sub(x, g);
        (this.wrapping_add(SLOT_OFF) as *mut u32).write_unaligned(x2.to_bits());
        if x2 > 0.0 {
            return (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        }
        let e = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if (e.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt3 = (e as *const u32).read_unaligned();
            let s3 = (vt3.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
            let f3: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(s3 as usize);
            if f3(e, ped, 2u32, 0u32) & 0xff == 0 {
                return (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
            }
            let fl = (e.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
            (e.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
        }
        let _: u32 = callee_thiscall!(4, u32, this, ped);
        0
    }
});
