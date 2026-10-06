// original: 0x00d6ac00 CReplayButtonBar::vf2
/// Flush the pending button and repaint the bar (original 0x00D6AC00,
/// thiscall/0).
///
/// When the dword at `this+0x28` is nonzero it is passed with the inner
/// object at `[this+0x1c]+8` to callee 2, and the slot is cleared. Then each
/// of the `count` entries named by the word at `this+0x20` is sent virtual
/// slot 8 (callee 1, thiscall/0) with no null check: a null
/// element would fault on both sides. Loop entry tests 0 against the count
/// as UNSIGNED (`jae`); the continue test is signed (`jl`) on the
/// zero-extended count, which agrees with unsigned for every value since
/// both operands are non-negative. Counts above 2 and null elements are
/// untested (see below). Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6ac00(this_ptr: u32) -> u32 {
    unsafe {
        const ARRAY_OFF: u32 = 0x1c;
        const COUNT_OFF: u32 = 0x20;
        const PENDING_OFF: u32 = 0x28;
        const INNER_OFF: u32 = 8;
        const SLOT_OFF: u32 = 8;
        let pending =
            ((this_ptr + PENDING_OFF) as *const u32).read_unaligned();
        if pending != 0 {
            let base =
                ((this_ptr + ARRAY_OFF) as *const u32).read_unaligned();
            let inner =
                ((base + INNER_OFF) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(2, u32, inner, pending);
            ((this_ptr + PENDING_OFF) as *mut u32).write_unaligned(0);
        }
        let count = ((this_ptr + COUNT_OFF) as *const u16).read_unaligned();
        if (count as u32) > 0 {
            let base =
                ((this_ptr + ARRAY_OFF) as *const u32).read_unaligned();
            let mut i: u32 = 0;
            loop {
                let obj = ((base + i * 4) as *const u32).read_unaligned();
                let vtab = (obj as *const u32).read_unaligned();
                let target =
                    ((vtab + SLOT_OFF) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                f(obj);
                i += 1;
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
        0
    }
});
