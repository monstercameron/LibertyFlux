// original: 0x00cce140 CTaskComplexMoveAboutInjured::vf5
/// Run the injured-move task's shutdown, returning true.
///
/// The subtask at `this+8`, unless its flag byte at `+0xc` is already set,
/// is shut down through its virtual slot at `+0x14` (thiscall on the
/// subtask, all three stack arguments); a zero answer fails the call with
/// 0, otherwise bit 1 of the flag byte is set. Then the timer commit runs
/// (thiscall on `this`, the first stack argument). Returns 1 in AL.
/// Thiscall with three stack arguments.
export!(thiscall, rw_00cce140(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x0c;
        const VT_SLOT: u32 = 0x14;
        const DONE_BIT: u32 = 2;
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if (sub.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vtab = (sub as *const u32).read_unaligned();
            let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if f(sub, a0, a1, a2) & 0xff == 0 {
                return 0;
            }
            let fl = (sub.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
            (sub.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
        }
        let _: u32 = callee_thiscall!(2, u32, this, a0);
        1
    }
});
