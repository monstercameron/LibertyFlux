// original: 0x00ca11d0 task_ik_init

/// Initialise an IK task from eleven setup words.
///
/// `this` is the task. Stack word 2 is the new target handle: the old handle
/// at `+0x14` is released first (callee 1) when non-null, the new one stored
/// and retained (callee 2), with bit 1 of the flags at `+0x5c` set when a
/// handle is present and cleared otherwise. Words 1/4 go to `+0x64`/`+0x2c`.
/// Word 5 points at a 16-byte aim block copied to `+0x30`; the contract
/// never passes null here (see below). Words 6-8 go to `+0x20`/`+0x24`/
/// `+0x28`, the low byte of word 9 to `+0x60`, word 10 to `+0x58`. Finally
/// the parameter refresh (callee 3) and target re-resolve (callee 4) run,
/// and the re-resolve's answer is returned. Words 0 and 3 are ignored.
///
/// The null-aim branch fills `+0x3c` from an aligned stack scratch slot the
/// rewrite cannot address, so the contract always passes a live aim block;
/// that branch is listed as not covered.
///
/// Original: 0x00ca11d0 (thiscall, eleven stack words).
lf_checker_rt::export!(thiscall, rw_00ca11d0(this: u32, _a0: u32, a1: u32, a2: u32, _a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const HANDLE: u32 = 0x14;
        const FLAGS: u32 = 0x5c;
        const HAS_HANDLE_BIT: u32 = 0x0000_0002;
        let old = rd32(this + HANDLE);
        if old != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, old, this + HANDLE);
            wr32(this + HANDLE, 0);
        }
        wr32(this + HANDLE, a2);
        if a2 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, a2, this + HANDLE);
            wr32(this + FLAGS, rd32(this + FLAGS) | HAS_HANDLE_BIT);
        } else {
            wr32(this + FLAGS, rd32(this + FLAGS) & !HAS_HANDLE_BIT);
        }
        wr32(this + 0x64, a1);
        wr32(this + 0x2c, a4);
        if a5 != 0 {
            wr32(this + 0x30, rd32(a5));
            wr32(this + 0x34, rd32(a5 + 4));
            wr32(this + 0x38, rd32(a5 + 8));
            wr32(this + 0x3c, rd32(a5 + 12));
        } else {
            wr32(this + 0x30, 0);
            wr32(this + 0x34, 0);
            wr32(this + 0x38, 0);
            wr32(this + 0x3c, 0);
        }
        wr32(this + 0x20, a6);
        wr32(this + 0x24, a7);
        wr32(this + 0x28, a8);
        wr8(this + 0x60, (a9 & 0xff) as u8);
        wr32(this + 0x58, a10);
        lf_checker_rt::callee_thiscall!(3, u32, this);
        lf_checker_rt::callee_thiscall!(4, u32, this, rd32(this + 0x24))
    }
});
