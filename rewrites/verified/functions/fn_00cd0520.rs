// original: 0x00cd0520 CTaskSimpleMoveMeleeMovement::vf1
/// Start the melee-move task through the task manager.
///
/// Fetches the manager handle (thiscall on the global manager pointer) and,
/// when non-null, forwards the mode word at `this+0x20` and the low bit of
/// the flag byte at `this+0x30` to the melee-move starter (thiscall on the
/// handle). Thiscall with no stack arguments.
export!(thiscall, rw_00cd0520(this: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        const MODE_OFF: u32 = 0x20;
        const FLAG_OFF: u32 = 0x30;
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(1, u32, mgr);
        if h != 0 {
            let bit = (this.wrapping_add(FLAG_OFF) as *const u8).read() & 1;
            let mode = (this.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
            let _: u32 = callee_thiscall!(2, u32, h, mode, u32::from(bit));
        }
        0
    }
});
