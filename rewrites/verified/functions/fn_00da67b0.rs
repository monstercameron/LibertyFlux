// original: 0x00DA67B0 CTaskComplexShockingEventWatch::vf5

/// Three-argument forward through the subtask's virtual check: unless bit
/// 0 of the subtask's `+0x0c` skips it, run virtual slot 0x14 of the
/// subtask at `+0x08` with the three caller arguments; a refused check
/// reports 0, a passed check sets the subtask's bit 1. Then forward the
/// first argument to the dispatch helper and report 1.
///
/// Only the low byte of the result is set on either path. Original:
/// thiscall, three stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da67b0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const CHECK: u32 = 1;
        const DISPATCH: u32 = 2;
        const CHECK_SLOT: u32 = 0x14;

        let sub = ((this + 0x08) as *const u32).read();
        if ((sub + 0x0C) as *const u8).read() & 1 == 0 {
            let table = (sub as *const u32).read();
            let slot = ((table + CHECK_SLOT) as *const u32).read();
            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let ok = check(sub, a1, a2, a3);
            if (ok & 0xFF) == 0 {
                return 0;
            }
            let marks = (sub + 0x0C) as *mut u32;
            marks.write(marks.read() | 2);
        }
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, a1);
        1
    }
});
