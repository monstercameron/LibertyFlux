// original: 0x0086CC80 Script_ProtSlotPeek2

/// Return the byte size of the next protected script slot.
///
/// The shared slot count is read as a 32-bit word and transformed with
/// wrapping `count * 4 + 4`; the function has no arguments or writes.
lf_checker_rt::export!(cdecl, rw_0086CC80() -> u32 {
    const SLOT_COUNT_VA: u32 = 0x018B_9B30;
    unsafe {
        lf_checker_rt::global::<u32>(SLOT_COUNT_VA)
            .read_unaligned()
            .wrapping_mul(4)
            .wrapping_add(4)
    }
});
