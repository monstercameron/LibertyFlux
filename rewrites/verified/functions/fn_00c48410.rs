// original: 0x00c48410 ccamscripted_swap_248 (proposed)
/// Swap the occupant of the slot at `this + SLOT` for `arg`.
///
/// The old occupant (if any) is released (callee 1), then `arg` is
/// stored into the slot and retained (callee 2, called with `arg` in
/// ecx). Returns whatever the retain call returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c48410(this: u32, arg: u32) -> u32 {
    const SLOT: u32 = 0x248;
    const RELEASE: u32 = 1;
    const RETAIN: u32 = 2;
    unsafe {
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        (slot as *mut u32).write_unaligned(arg);
        lf_checker_rt::callee_thiscall!(RETAIN, u32, arg, slot)
    }
});
