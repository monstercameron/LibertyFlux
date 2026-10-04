// original: 0x00d29850 targeting_clear_slot (proposed)
/// If the target slot at `this+0x238` is non-null, release it through the
/// release helper and clear it. Returns the helper's answer, or the incoming
/// eax when the slot was already null.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d29850(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x238;
        const RELEASE: u32 = 1;
        let slot = this + SLOT;
        let cur = (slot as *const u32).read_unaligned();
        if cur != 0 {
            let r: u32 = lf_checker_rt::callee_stdcall!(RELEASE, u32, slot);
            (slot as *mut u32).write_unaligned(0);
            r
        } else {
            cur
        }
    }
});
