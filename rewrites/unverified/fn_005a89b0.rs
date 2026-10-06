// original: 0x005A89B0 reset_slots_165 (proposed)

/// Reset every input slot, then report the last reset result.
///
/// Calls the slot-reset callee 165 times (`SLOT_COUNT` = 0xA5). Before each
/// call the shared slot-index global holds that call's zero-based index
/// (0..165), so the global ends at 164. Each call passes (1, 0): reset flag
/// set, no extra argument. The return value is whatever the final call
/// returned; the function takes no arguments and keeps the stack balanced
/// (cdecl, the caller pops both words each round).
lf_checker_rt::export!(cdecl, rw_005A89B0() -> u32 {
    unsafe {
        const SLOT_INDEX: u32 = 0x0103_0BA4;
        const SLOT_COUNT: u32 = 0xA5;
        const RESET_CALLEE: u32 = 1;
        let mut last = 0u32;
        for i in 0..SLOT_COUNT {
            lf_checker_rt::global::<u32>(SLOT_INDEX).write(i);
            last = lf_checker_rt::callee_cdecl!(RESET_CALLEE, u32, 1u32, 0u32);
        }
        last
    }
});
