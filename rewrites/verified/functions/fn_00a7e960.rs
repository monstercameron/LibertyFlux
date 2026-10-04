// original: 0x00a7e960 CComplexFallAndGetUpTaskInfo::vf6
/// Serialize `CComplexFallAndGetUpTaskInfo` after its base fields.
///
/// Runs the base serializer, then writes the `+0x18` field with 2 bits,
/// and returns the helper's answer ORed with the base's changed flag.
lf_checker_rt::export!(thiscall, rw_00a7e960(this: u32, stream: u32) -> u32 {
    let base_changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
    unsafe {
        let field = ((this as *const u8).add(0x18) as *const u32).read_unaligned();
        let changed = lf_checker_rt::callee_thiscall!(2, u32, stream, field, 2, 0);
        changed | base_changed as u32
    }
});
