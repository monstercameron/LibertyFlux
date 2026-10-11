// original: 0x005F7D40 rage::fragInst::vf61

/// Return unless the signed candidate exceeds the signed high-water value.
/// For a new high-water value, record it and update the call count while that
/// count is below its signed limit.
///
/// The manager pointer is read from the relocated slot; its service pointer
/// is at `+0x2f0`, and the candidate signed `i32` is at service offset `+0x08`.
/// The last-value slot and update-count slot are separate four-byte globals.
/// The first comparison updates the last value only when the candidate is
/// strictly greater as a signed integer. The second increments the signed
/// count only while it is below `0x100`. This thiscall method ignores its
/// receiver and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_005f7d40(_this: u32) -> () {
    const MANAGER_GLOBAL: u32 = 0x018B_7448;
    const HIGH_WATER_GLOBAL: u32 = 0x018B_7434;
    const UPDATE_COUNT_GLOBAL: u32 = 0x018B_7438;
    const MANAGER_SERVICE: u32 = 0x2f0;
    const SERVICE_VALUE: u32 = 0x08;
    const COUNTER_LIMIT: i32 = 0x100;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL));
        let service = read_u32(manager.wrapping_add(MANAGER_SERVICE));
        let candidate = read_u32(service.wrapping_add(SERVICE_VALUE));
        let last_value_slot = lf_checker_rt::global::<u32>(HIGH_WATER_GLOBAL);
        let last_value = core::ptr::read_unaligned(last_value_slot);
        if (candidate as i32) <= (last_value as i32) {
            return;
        }
        core::ptr::write_unaligned(last_value_slot, candidate);

        let update_count_slot = lf_checker_rt::global::<u32>(UPDATE_COUNT_GLOBAL);
        let update_count = core::ptr::read_unaligned(update_count_slot);
        if (update_count as i32) < COUNTER_LIMIT {
            core::ptr::write_unaligned(update_count_slot, update_count.wrapping_add(1));
        }
    }
});
