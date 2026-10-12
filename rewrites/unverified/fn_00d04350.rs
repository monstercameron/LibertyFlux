// original: 0x00D04350 CTaskComplexCombatChargeSubtask::vf20

/// Announce a combat charge when the charge flag is set.
///
/// When bit 1 of the flag word at `+0xc` is set, forwards the fixed ten-word
/// descriptor (message id, zeros, -1, more zeros, 1.0, zeros) to the charge
/// announcer (patched thiscall callee) on the sibling block at `a1 + 0x570`.
/// Returns the sub-task word at `+0x8`.
///
/// Original: 0x00D04350 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d04350(this: u32, a1: u32) -> u32 {
    unsafe {
        const ANNOUNCE: u32 = 1;
        const MSG: u32 = 0x00EDF8C4;
        const SIBLING_OFF: u32 = 0x570;
        const ONE_BITS: u32 = 0x3F800000; // 1.0f

        if ((this + 0x0c) as *const u32).read_unaligned() & 2 != 0 {
            lf_checker_rt::callee_thiscall!(ANNOUNCE, u32, a1.wrapping_add(SIBLING_OFF),
                MSG, 0, 0, 0, 0xFFFFFFFFu32, 0, 0, ONE_BITS, 0, 0);
        }
        ((this + 0x08) as *const u32).read_unaligned()
    }
});
