// original: 0x00b4c840 group_update_active_count

/// Increment the shared active-group count, wrapping as a 32-bit unsigned
/// value, when the shared activity byte is nonzero. The function is void.
/// Both globals are image references and the coordinator contract varies
/// the gate and the count independently across completed trials.
lf_checker_rt::export!(cdecl, rw_00b4c840() -> () {
    const ACTIVITY_FLAG: u32 = 0x0166_6C80;
    const ACTIVE_GROUP_COUNT: u32 = 0x0166_6C84;

    // SAFETY: both addresses are writable image data referenced by the original.
    let active = unsafe { lf_checker_rt::global::<u8>(ACTIVITY_FLAG).read_volatile() };
    if active != 0 {
        let count = unsafe { lf_checker_rt::global::<u32>(ACTIVE_GROUP_COUNT) };
        let current = unsafe { count.read_volatile() };
        unsafe { count.write_volatile(current.wrapping_add(1)) };
    }
});
