// original: 0x00b7ba80 CTaskComplexCarDriveMissionFleeScene::vf20

/// Return zero when the ped's mission flags, associated value, and this
/// task's mode flag all meet the completion condition. Otherwise forward
/// `this` and the ped pointer to the shared drive-to-point implementation
/// and return its result. The dword comparison is unsigned bitwise equality;
/// the two one-byte checks test bit `0x04`.
///
/// Original ABI: thiscall with one pointer argument. The fallback is an
/// unconditional tail jump to the shared implementation.
lf_checker_rt::export!(thiscall, rw_00b7ba80(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_MISSION_FLAGS: u32 = 0x26c;
        const PED_MODE: u32 = 0xb30;
        const TASK_VALUE: u32 = 0x40;
        const TASK_FLAGS: u32 = 0x3f;
        const MODE_BIT: u8 = 0x04;
        let mission_flags = ((ped + PED_MISSION_FLAGS) as *const u8).read();
        let ped_mode = ((ped + PED_MODE) as *const u32).read_unaligned();
        let task_value = ((this + TASK_VALUE) as *const u32).read_unaligned();
        let task_flags = ((this + TASK_FLAGS) as *const u8).read();
        if mission_flags & 0x04 != 0 && ped_mode == task_value && task_flags & MODE_BIT != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, this, ped)
    }
});
