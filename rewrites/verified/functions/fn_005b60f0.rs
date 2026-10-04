// original: 0x005B60F0 Txd_FindSlot
/// Resolves a streaming name to an id, then tail-forwards the id to the
/// texture-dictionary slot lookup.
export!(cdecl, rw_005B60F0(name: u32) -> u32 {
    let id = callee_cdecl!(1, u32, name);
    callee_cdecl!(2, u32, id)
});
