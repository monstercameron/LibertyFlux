// original: 0x00A4D8C0 CVehicle::vf80

/// Virtual method 80: resets a sub-object, snapshots a global, and raises
/// status flags.
///
/// Calls the first callee with `this` in `ecx` and one zero stack word, then
/// copies the global dword at `RATE` into `this + SNAP` (0x0E38) and calls
/// the second callee with `this + SUB` (0x0EA4) in `ecx` and no stack words.
/// Finally sets bit 6 of the flag byte at `this + FLAG` (0x0F1D), stores 10
/// into the word at `this + COUNT` (0x0ECB) and 1 into the bytes at
/// `this + READY_A` (0x0ECA) and `this + READY_B` (0x0EA0). Returns the second
/// callee's answer; the single straight-line path makes it deterministic.
///
/// Original: 0x00A4D8C0 (thiscall, no stack words), two direct callees.
lf_checker_rt::export!(thiscall, rw_00A4D8C0(this: u32) -> u32 {
    unsafe {
        const SNAP: u32 = 0x0E38;
        const SUB: u32 = 0x0EA4;
        const FLAG: u32 = 0x0F1D;
        const COUNT: u32 = 0x0ECB;
        const READY_A: u32 = 0x0ECA;
        const READY_B: u32 = 0x0EA0;
        const RATE: u32 = 0x011735B4;
        const FLAG_BIT: u8 = 0x40;
        const COUNT_INIT: u16 = 10;
        const RESET_CALLEE: u32 = 1;
        const SUB_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, this, 0);
        let g = lf_checker_rt::global::<u32>(RATE).read_unaligned();
        ((this + SNAP) as *mut u32).write_unaligned(g);
        let ans: u32 =
            lf_checker_rt::callee_thiscall!(SUB_CALLEE, u32, this + SUB);
        let f = (this + FLAG) as *mut u8;
        f.write(f.read() | FLAG_BIT);
        ((this + COUNT) as *mut u16).write_unaligned(COUNT_INIT);
        ((this + READY_A) as *mut u8).write(1);
        ((this + READY_B) as *mut u8).write(1);
        ans
    }
});
