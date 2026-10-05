// original: 0x00d54600 CCamFree::vf6

/// Reset the free camera's state words and clear its vector, reporting success.
///
/// `this` points to the object. The active byte at `ACTIVE` is set, three
/// state words at `STATE` are zeroed, the shared vector-clear routine runs
/// with `this` in ECX, then a state word at `EXTRA` and a flag byte at
/// `FLAG` are zeroed. Returns 1 in AL (upper EAX passes through, so only AL
/// is compared).
///
/// Original: 0x00d54600 (thiscall, no stack arguments, one call).
lf_checker_rt::export!(thiscall, rw_00d54600(this: u32) -> u32 {
    unsafe {
        /// Active marker set on entry.
        const ACTIVE: u32 = 0x15c;
        /// First of three zeroed state words.
        const STATE: u32 = 0x140;
        /// Trailing state word zeroed after the call.
        const EXTRA: u32 = 0x188;
        /// Trailing flag byte zeroed after the call.
        const FLAG: u32 = 0x18d;
        /// Shared vector-clear routine (intercepted; thiscall, no arguments).
        const CLEAR_VEC: u32 = 1;
        ((this + ACTIVE) as *mut u8).write(1);
        ((this + STATE) as *mut u32).write_unaligned(0);
        ((this + STATE + 4) as *mut u32).write_unaligned(0);
        ((this + STATE + 8) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(CLEAR_VEC, u32, this);
        ((this + EXTRA) as *mut u32).write_unaligned(0);
        ((this + FLAG) as *mut u8).write(0);
        1
    }
});
