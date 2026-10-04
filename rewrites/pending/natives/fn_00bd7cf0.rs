// original: 0x00bd7cf0 GET_VEHICLE_FROM_NETWORK_ID
/// Script native `GET_VEHICLE_FROM_NETWORK_ID` (hash 0x794E4A82).
///
/// Resolves a network id to a vehicle: forwards two script words to
/// the engine. No return slot is written.
export!(cdecl, rw_00bd7cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
