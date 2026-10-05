// original: 0x009a8710 flag_lookup_or_default
/// Resolve the flag key for `id` and hand it to the flag registrar.
///
/// When `id` is null the default builder (stubbed, cdecl/1) supplies
/// the key base; otherwise `id` is used directly. Either way 0xd31 is
/// added and the sum is passed with `slot` to the registrar (stubbed,
/// cdecl/2, key first). Stdcall, two stack words, no result.
export!(stdcall, rw_009A8710(id: u32, slot: u32) -> u32 {
    unsafe {
        const KEY_BIAS: u32 = 0xd31;
        let base = if id == 0 { callee_cdecl!(1, u32, 0) } else { id };
        let _: u32 = callee_cdecl!(2, u32, base.wrapping_add(KEY_BIAS), slot);
        0
    }
});
