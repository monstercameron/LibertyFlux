// original: 0x00A82FB0 fn_00a82fb0_task_status_predicate

/// Return the status predicate while preserving the source word's upper bytes.
///
/// Reads the full 32-bit status at `this + 0x14`. Values 5, 6 and 7 set the
/// low return byte to one; every other value clears only that byte. The high
/// three bytes of EAX therefore remain the high three bytes of the field.
lf_checker_rt::export!(thiscall, rw_00a82fb0(this: u32) -> u32 {
    const TASK_STATUS: u32 = 0x14;
    const READY: u32 = 5;
    const ACTIVE: u32 = 6;
    const COMPLETE: u32 = 7;

    let status = unsafe { ((this + TASK_STATUS) as *const u32).read_unaligned() };
    let truth = u32::from(matches!(status, READY | ACTIVE | COMPLETE));
    (status & 0xFFFF_FF00) | truth
});
