// original: 0x009e3ad0 ped_apply_params6 (proposed)

/// Store two float params, apply four int params, notify when flagged.
///
/// Writes `a5`/`a6` to `this + 0xd48`/`+ 0xd4c`, forwards `(a1..a4, 0)`
/// to the apply callee (`thiscall` on this), then folds the mode word at
/// `this + 0x1e2` into the result exactly as the original does (low word
/// replaced by `word >> 13`). When bit 13 of that word is set, sets bit 0
/// of `this + 0x24` and calls the notify callee (`thiscall` on
/// `[this + 0x7b0]` with `(8, 1)`), whose answer becomes the result.
/// `thiscall`, six stack words.
lf_checker_rt::export!(thiscall, rw_009e3ad0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const OUT1: u32 = 0xd48;
        const OUT2: u32 = 0xd4c;
        const MODE: u32 = 0x1e2;
        const MODE_BIT: u32 = 13;
        const NOTIFY_OBJ: u32 = 0x7b0;
        const NOTIFY_FLAG: u32 = 0x24;
        const APPLY: u32 = 1;
        const NOTIFY: u32 = 2;
        ((this + OUT1) as *mut u32).write_unaligned(a5);
        ((this + OUT2) as *mut u32).write_unaligned(a6);
        let mut r = lf_checker_rt::callee_thiscall!(APPLY, u32, this, a1, a2, a3, a4, 0u32);
        let w = ((this + MODE) as *const u16).read_unaligned() as u32;
        r = (r & 0xffff0000) | (w >> MODE_BIT);
        if ((w >> MODE_BIT) & 1) == 1 {
            let obj = ((this + NOTIFY_OBJ) as *const u32).read_unaligned();
            let f = ((this + NOTIFY_FLAG) as *const u32).read_unaligned();
            ((this + NOTIFY_FLAG) as *mut u32).write_unaligned(f | 1);
            r = lf_checker_rt::callee_thiscall!(NOTIFY, u32, obj, 8u32, 1u32);
        }
        r
    }
});
