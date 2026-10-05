// original: 0x0093e410 stream_epoch_tick (proposed)

/// Advance the streaming epoch counter, resetting at 0xFFFF.
///
/// Reads the 16-bit counter at `EPOCH`: below 0xFFFF it is incremented in
/// place, otherwise the reset callee runs and the counter becomes 1.
/// Returns the new counter value in the low 16 bits (the high bits keep
/// whatever the caller left in the register, so only `ax` is compared).
///
/// Original: 0x0093e410 (cdecl, no arguments; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e410() -> u32 {
    const EPOCH: u32 = 0x11A8908;
    const EPOCH_MAX: u16 = 0xFFFF;
    const RESET_CALLEE: u32 = 1;
    unsafe {
        let slot = lf_checker_rt::global::<u16>(EPOCH) as *mut u16;
        let cur = (slot as *const u16).read_unaligned();
        if cur >= EPOCH_MAX {
            lf_checker_rt::callee_cdecl!(RESET_CALLEE, u32,);
            slot.write_unaligned(1);
            1
        } else {
            let next = cur + 1;
            slot.write_unaligned(next);
            next as u32
        }
    }
});
