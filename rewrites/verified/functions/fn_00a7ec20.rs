// original: 0x00a7ec20 CComplexPickUpAndCarryObjectTaskInfo::vf6
/// Serialize `CComplexPickUpAndCarryObjectTaskInfo` after its base.
///
/// Runs the base serializer, writes the `+0x24` field rebased by
/// `0x48` with 2 bits, and returns the helper's answer ORed with the
/// base's changed flag.
lf_checker_rt::export!(thiscall, rw_00a7ec20(this: u32, stream: u32) -> u32 {
    let base_changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
    unsafe {
        let raw = ((this as *const u8).add(0x24) as *const u32).read_unaligned();
        let changed = lf_checker_rt::callee_thiscall!(2, u32, stream,
            raw.wrapping_sub(0x48), 2, 0);
        changed | base_changed as u32
    }
});
