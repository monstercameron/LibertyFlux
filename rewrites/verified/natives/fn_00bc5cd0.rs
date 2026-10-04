// original: 0x00bc5cd0 GET_CAR_UPRIGHT_VALUE
/// Script native `GET_CAR_UPRIGHT_VALUE` (hash 0x326E2886).
///
/// Forwards two script arguments (a vehicle handle and an out-pointer) to the engine. No return slot is written.
export!(cdecl, rw_00bc5cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
