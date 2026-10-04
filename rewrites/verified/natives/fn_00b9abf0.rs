// original: 0x00b9abf0 GET_SPAWN_COORDINATES_FOR_CAR_NODE
/// Script native `GET_SPAWN_COORDINATES_FOR_CAR_NODE` (hash 0x5B386B6C).
///
/// Passes a constant address word (file VA 0x00B9CFE0, which carries a
/// HIGHLOW relocation fixup) and the call-context pointer itself to the
/// engine. No script argument is read and no return slot is written.
export!(cdecl, rw_00b9abf0(ctx: *const u8) -> u32 {
    const TAG_FILE_VA: u32 = 0x00B9CFE0;
    callee_cdecl!(1, u32, relocated(TAG_FILE_VA), ctx as u32)
});
