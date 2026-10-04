// original: 0x00a7ec50 CComplexSlideIntoCoverInfo::vf6
/// Serialize `CComplexSlideIntoCoverInfo` after its base fields.
///
/// Runs the base serializer, resolves a limit with no stack arguments,
/// writes it with 5 bits, and returns the helper's answer ORed with
/// the base's changed flag.
lf_checker_rt::export!(thiscall, rw_00a7ec50(this: u32, stream: u32) -> u32 {
    let base_changed = lf_checker_rt::callee_thiscall!(1, u32, this, stream) as u8;
    let limit = lf_checker_rt::callee_cdecl!(2, u32,);
    let changed = lf_checker_rt::callee_thiscall!(3, u32, stream, limit, 5, 0);
    changed | base_changed as u32
});
