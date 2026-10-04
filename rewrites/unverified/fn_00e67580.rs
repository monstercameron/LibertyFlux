// original: 0x00e67580 init_task_slots_and_register
/// Initialises sixteen task slots, then runs the table-registration call.
///
/// Invokes the shared slot initialiser (thiscall/0, intercepted by the
/// checker) on each of the sixteen 0x20-byte slots in order, then passes
/// the table callback to the registration routine (cdecl/1, intercepted).
/// Returns the registration answer, matching the value the original
/// leaves in EAX.
export!(cdecl, rw_00e67580() -> u32 {
    unsafe {
        const FIRST_SLOT: u32 = 0x012F9338;
        const SLOT_STRIDE: u32 = 0x20;
        const SLOT_COUNT: u32 = 16;
        const REGISTER_ARG: u32 = 0x00E72250;
        let mut slot = relocated(FIRST_SLOT);
        let mut remaining = SLOT_COUNT;
        loop {
            let _: u32 = callee_thiscall!(1, u32, slot);
            slot = slot.wrapping_add(SLOT_STRIDE);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        callee_cdecl!(2, u32, relocated(REGISTER_ARG))
    }
});
