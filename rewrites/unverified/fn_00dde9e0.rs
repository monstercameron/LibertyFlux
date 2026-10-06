// original: 0x00DDE9E0 UITextField select and tail forward
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Select the child for `mode`, then tail-jump to its virtual slot `+0x1E0`
/// with (`key`, 0): the second argument slot is overwritten with 0 (its own
/// incoming slot, so the stack check is off for this proof) and the call never
/// returns here. Returns the callee's answer.
/// Original: thiscall, two stack words, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDE9E0(this: u32, key: u32, mode: u32) -> u32 {
    unsafe {
        const SELECT: u32 = 1;
        const SLOT: u32 = 0x1E0;
        let child = lf_checker_rt::callee_thiscall!(SELECT, u32, this, mode);
        let vt = ((child) as *const u32).read_unaligned();
        let slot = ((vt + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        f(child, key, 0)
    }
});
