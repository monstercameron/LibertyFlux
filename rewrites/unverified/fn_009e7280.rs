// original: 0x009e7280 task_forward_guarded_b (proposed)

/// Conditionally mark this object, then forward two words to a sibling.
///
/// When the low byte of `a3` is nonzero and bit 0 of `this + 0x29f` is
/// clear, calls the setup callee (`thiscall` on this, no stack words)
/// and sets bit 24 of `this + 0x29c`. Always then forwards `(a1, a2)`
/// to the sibling callee (`thiscall` on this) and returns its answer.
/// `thiscall`, three stack words. Identical shape to 0x009e7070 with a
/// different sibling target.
lf_checker_rt::export!(thiscall, rw_009e7280(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x29f;
        const MARK: u32 = 0x29c;
        const MARK_BIT: u32 = 0x1000000;
        const SETUP: u32 = 1;
        const FORWARD: u32 = 2;
        if (a3 & 0xff) != 0 && (((this + FLAG) as *const u8).read() & 1) == 0 {
            lf_checker_rt::callee_thiscall!(SETUP, u32, this);
            let m = ((this + MARK) as *const u32).read_unaligned();
            ((this + MARK) as *mut u32).write_unaligned(m | MARK_BIT);
        }
        lf_checker_rt::callee_thiscall!(FORWARD, u32, this, a1, a2)
    }
});
