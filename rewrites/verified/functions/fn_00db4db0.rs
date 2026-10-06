// original: 0x00DB4DB0 UIMouseCursor::vf87

/// Slot 87 of the mouse-cursor virtual table: guarded tail call.
///
/// `this` is the cursor object. The flag word at `GUARD` is set while a
/// preparation helper runs (scripted on both sides) and cleared again,
/// then control passes to another routine with the same object by a tail
/// jump (intercepted as a tail call), whose answer is returned. The
/// rewrite performs that last step as a call that forwards the object and
/// the answer.
///
/// Original: 0x00DB4DB0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db4db0(this: u32) -> u32 {
    unsafe {
        const GUARD: u32 = 0x1dc;
        const PREPARE: u32 = 1;
        const TAIL: u32 = 2;
        ((this + GUARD) as *mut u32).write_unaligned(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, this);
        ((this + GUARD) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
