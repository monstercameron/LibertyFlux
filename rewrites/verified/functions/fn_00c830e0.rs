// original: 0x00c830e0 scenario_handle_swap (proposed)

/// Replace the handle at `this+0x1c` with `new`, releasing/retaining around it.
///
/// If the current handle is non-null, `c1(current, slot)` runs first (release
/// of the old value, `slot = this+0x1c`). The slot is then set to `new`, and
/// if that is non-null `c2(new, slot)` runs (retain of the new value). Returns
/// nothing meaningful (EAX is a leftover callee result or entry garbage on the
/// empty path; all three clean-range callers ignore it), so the contract
/// compares no return channel.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c830e0(this: u32, new: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0x1c;
        const C1: u32 = 1;
        const C2: u32 = 2;
        let esi = this + SLOT_OFF;
        let cur = (esi as *const u32).read_unaligned();
        if cur != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, cur, esi);
        }
        (esi as *mut u32).write_unaligned(new);
        if new != 0 {
            lf_checker_rt::callee_thiscall!(C2, u32, new, esi);
        }
        0
    }
});
