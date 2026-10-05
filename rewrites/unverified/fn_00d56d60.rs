// original: 0x00d56d60 ccam_mark_active

/// Fetch the shared camera context, attach it, and set the active bit.
///
/// `this` points to the object. The shared context fetch runs with `this` in
/// ECX; its answer is passed as the stack argument to the attach routine
/// (also with `this` in ECX); then bit 0 of the flag byte at `FLAGS` is set.
/// Returns the attach routine's answer.
///
/// Original: 0x00d56d60 (thiscall, no stack arguments, two calls).
lf_checker_rt::export!(thiscall, rw_00d56d60(this: u32) -> u32 {
    unsafe {
        /// Flag byte carrying the active bit.
        const FLAGS: u32 = 0x27c;
        /// Active bit set after attaching.
        const ACTIVE_BIT: u8 = 1;
        /// Shared context fetch (intercepted; thiscall, no arguments).
        const FETCH_CTX: u32 = 1;
        /// Attach routine (intercepted; thiscall, one stack argument).
        const ATTACH: u32 = 2;
        let ctx = lf_checker_rt::callee_thiscall!(FETCH_CTX, u32, this);
        let r = lf_checker_rt::callee_thiscall!(ATTACH, u32, this, ctx);
        let f = ((this + FLAGS) as *mut u8).read();
        ((this + FLAGS) as *mut u8).write(f | ACTIVE_BIT);
        r
    }
});
