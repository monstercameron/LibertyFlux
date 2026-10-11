// original: 0x00a2a110 player_ped_reset_task_state

/// Resolve the task manager at `this+0x228`, inspect byte `+0x3d0` and
/// counter `+0x3d8` at its `+0x70` subobject, and when bit 1 is set and the
/// counter is nonzero, clear the counter and store the exact 1.0f bit pattern
/// in the reset global. The method returns the resolved manager address.
lf_checker_rt::export!(thiscall, rw_a2a110(this: u32) -> u32 {
    const MANAGER_FIELD: u32 = 0x0228;
    const STATE_FLAGS: u32 = 0x03d0;
    const ACTIVE_COUNT: u32 = 0x03d8;
    const RESET_VALUE_GLOBAL: u32 = 0x0103234c;
    let stored = unsafe { ((this + MANAGER_FIELD) as *const u32).read_unaligned() };
    let mut result = stored;
    let manager = stored.wrapping_add(0x70);
    let flags = unsafe { ((manager + STATE_FLAGS) as *const u8).read_unaligned() };
    if flags & 2 != 0 {
        let count = unsafe { ((manager + ACTIVE_COUNT) as *const u32).read_unaligned() };
        if count != 0 {
            unsafe {
                ((manager + ACTIVE_COUNT) as *mut u32).write_unaligned(0);
                lf_checker_rt::global::<u32>(RESET_VALUE_GLOBAL).write_unaligned(0x3f80_0000);
            }
            result = manager;
        }
    }
    result
});
