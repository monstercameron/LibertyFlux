// original: 0x0086CBE0 Script_ProtSlotAllocGlobal

/// Reserve the next protected script slot and return its byte offset.
///
/// The shared slot counter is a signed 32-bit value. Values below 1024 are
/// incremented with wrapping arithmetic; values at or above 1024 invoke the
/// allocator helper with its settings pointer, then return the current
/// counter multiplied by four. The helper call is retained as an intercepted
/// cdecl call. This function takes no arguments and returns the full EAX
/// value.
lf_checker_rt::export!(cdecl, rw_0086CBE0() -> u32 {
    const SLOT_COUNT_VA: u32 = 0x018B_9B30;
    const ALLOC_SETTINGS_VA: u32 = 0x00FC_6FA0;
    const SLOT_LIMIT_SIGNED: i32 = 1024;
    const ALLOC_HELPER: u32 = 1;

    unsafe {
        let counter = lf_checker_rt::global::<u32>(SLOT_COUNT_VA);
        let current = counter.read_unaligned();
        if (current as i32) < SLOT_LIMIT_SIGNED {
            let next = current.wrapping_add(1);
            counter.write_unaligned(next);
            next.wrapping_mul(4)
        } else {
            let settings = lf_checker_rt::global::<u8>(ALLOC_SETTINGS_VA) as u32;
            let _ = lf_checker_rt::callee_cdecl!(ALLOC_HELPER, u32, settings, 0, 0);
            counter.read_unaligned().wrapping_mul(4)
        }
    }
});
