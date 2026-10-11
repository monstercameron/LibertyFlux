// original: 0x009C26A0 Anims_GetIndexFromName

/// Resolve an animation name to an index, then forward that index to the
/// animation lookup helper and return its EAX result. Both outgoing calls
/// are cdecl; the first takes the caller's name pointer and the second takes
/// the index returned by the first helper.
lf_checker_rt::export!(cdecl, rw_009c26a0(name: u32) -> u32 {
    let index = lf_checker_rt::callee_cdecl!(1, u32, name);
    lf_checker_rt::callee_cdecl!(2, u32, index)
});
