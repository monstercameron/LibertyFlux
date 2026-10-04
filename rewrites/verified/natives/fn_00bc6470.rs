// original: 0x00bc6470 GET_CAMERA_FROM_NETWORK_ID
/// Script native `GET_CAMERA_FROM_NETWORK_ID` (hash 0x7E656E50).
///
/// Forwards two script arguments (a network id and an out-pointer) to the engine. No return slot is written.
export!(cdecl, rw_00bc6470(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
