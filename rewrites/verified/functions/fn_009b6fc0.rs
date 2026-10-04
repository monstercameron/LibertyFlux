// original: 0x009B6FC0 NativeImpl_GET_VIEWPORT_POSITION_OF_COORD
/// Look up the viewport entry for a coordinate triple and unwrap it.
///
/// Calls the coordinate query with (`a1`, `a0`, `a2`, `a2 + 4`) and unwraps
/// the returned entry sixteen bytes past its base. stdcall, three integers.
lf_checker_rt::export!(stdcall, rw_009B6FC0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ENTRY_INNER: u32 = 0x10;
        let entry: u32 = lf_checker_rt::callee_stdcall!(1, u32, a1, a0, a2, a2.wrapping_add(4));
        lf_checker_rt::callee_thiscall!(2, u32, entry.wrapping_add(ENTRY_INNER))
    }
});
