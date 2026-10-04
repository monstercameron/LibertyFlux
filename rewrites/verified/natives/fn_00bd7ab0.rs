// original: 0x00bd7ab0 GET_NETWORK_ID_FROM_PED
/// Script native `GET_NETWORK_ID_FROM_PED` (hash 0x7BEE5003).
///
/// Forwards two script arguments (a ped handle and an out-pointer slot for
/// the network id) to the engine. No return slot is written.
export!(cdecl, rw_00bd7ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
