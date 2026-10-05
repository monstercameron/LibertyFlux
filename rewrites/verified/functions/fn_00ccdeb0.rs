// original: 0x00ccdeb0 CTaskSimpleDead::vf6
/// Return the dead-task blend duration for a ped: -1.0 when the ped's
/// current task (`ped+0x38`) is non-null and still the recorded one
/// (`ped+0x7b4`), otherwise 25.0. Both constants come from read-only data.
/// Takes the ped as its single stack argument (ECX is ignored) and returns
/// the float on the x87 stack.
export!(thiscall, rw_00ccdeb0(_this: u32, ped: u32) -> f32 {
    unsafe {
        const CUR_TASK_OFF: u32 = 0x38;
        const REC_TASK_OFF: u32 = 0x7b4;
        const MATCHED_SECS: u32 = 0x00fe8d94;
        const DEFAULT_SECS: u32 = 0x00fe8b40;
        let cur = (ped.wrapping_add(CUR_TASK_OFF) as *const u32).read_unaligned();
        let bits = if cur != 0
            && cur == (ped.wrapping_add(REC_TASK_OFF) as *const u32).read_unaligned()
        {
            *global::<u32>(MATCHED_SECS)
        } else {
            *global::<u32>(DEFAULT_SECS)
        };
        f32::from_bits(bits)
    }
});
