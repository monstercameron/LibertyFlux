// original: 0x00bd7bc0 GET_PED_FROM_NETWORK_ID
/// Script native `GET_PED_FROM_NETWORK_ID` (hash 0x69F11716).
///
/// Forwards two script arguments (network id and flags) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bd7bc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        )
    }
});
