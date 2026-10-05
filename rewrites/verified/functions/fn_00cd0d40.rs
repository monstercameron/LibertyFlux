// original: 0x00cd0d40 CTaskComplexMelee::vf18
/// Report the melee task's follow-up request, returning the sink's answer.
///
/// Runs the reference cleanup (thiscall on `this`), returns 0 when the
/// disable bit (0x20 of `this+0xf0`) is set, and asks the subtask at
/// `this+8` through its virtual slot at `+0xc`: unless it answers 0x11d
/// or 0x1b1 the call returns 0, otherwise the stack argument, 0x1b1 and 0
/// go to the follow-up sink (thiscall on `this`), whose answer is
/// returned. Thiscall with one stack argument.
export!(thiscall, rw_00cd0d40(this: u32, a0: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0xf0;
        const DISABLE_BIT: u8 = 0x20;
        const VT_SLOT: u32 = 0x0c;
        const KIND: u32 = 0x1b1;
        const WANT_A: u32 = 0x11d;
        let _: u32 = callee_thiscall!(1, u32, this);
        if (this.wrapping_add(FLAG_OFF) as *const u8).read() & DISABLE_BIT != 0 {
            return 0;
        }
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        let vtab = (sub as *const u32).read_unaligned();
        let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let ans = f(sub);
        if ans != WANT_A && ans != KIND {
            return 0;
        }
        callee_thiscall!(3, u32, this, a0, KIND, 0u32)
    }
});
