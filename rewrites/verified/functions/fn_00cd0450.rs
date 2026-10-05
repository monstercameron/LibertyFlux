// original: 0x00cd0450 CTaskComplexMelee::vf1
/// Start the complex melee task through the task manager.
///
/// Fetches the manager handle (thiscall on the global manager pointer) and
/// returns when null. Otherwise takes the low byte of the mode helper's
/// answer (thiscall on `this`) into the flag helper (thiscall on `this`),
/// then forwards the word at `this+0x30`, bits 1, 2, 0 and 3 of the flag
/// byte at `this+0xf0`, and the flag helper's answer to the melee starter
/// (thiscall on the handle, six arguments). Thiscall, no stack arguments.
export!(thiscall, rw_00cd0450(this: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        const MODE_OFF: u32 = 0x30;
        const FLAG_OFF: u32 = 0xf0;
        let mgr = *global::<u32>(MGR_G);
        let h: u32 = callee_thiscall!(1, u32, mgr);
        if h == 0 {
            return 0;
        }
        let fl = (this.wrapping_add(FLAG_OFF) as *const u8).read();
        let mode: u32 = callee_thiscall!(2, u32, this);
        let flags: u32 = callee_thiscall!(3, u32, this, mode & 0xff);
        let q5 = (this.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
        let q4 = u32::from((fl >> 1) & 1);
        let q3 = u32::from((fl >> 2) & 1);
        let q2 = u32::from(fl & 1);
        let q1 = u32::from((fl >> 3) & 1);
        let _: u32 = callee_thiscall!(4, u32, h, q5, q4, q3, q2, q1, flags);
        0
    }
});
