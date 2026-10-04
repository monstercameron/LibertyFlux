// original: 0x00b9a7e0 GET_NEXT_CLOSEST_CAR_NODE_FAVOUR_DIRECTION
/// Script native `GET_NEXT_CLOSEST_CAR_NODE_FAVOUR_DIRECTION` (hash 0x6E3906E4).
///
/// Passes the call context itself plus a relocated engine address to the
/// engine. The address is pushed as an immediate with a HIGHLOW relocation
/// entry at the push site (Verified: raw `.reloc` bytes and the worker's
/// own parse), so it follows the image base; it is derived with
/// `relocated()` like any other original address. The handler never reads
/// the argument array. No return slot is written.
export!(cdecl, rw_00b9a7e0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xB9BC10), ctx as u32) }
});
