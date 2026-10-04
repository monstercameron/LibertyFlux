// original: 0x00bc5bc0 GET_CAR_MODEL_VALUE
/// Script native `GET_CAR_MODEL_VALUE` (hash 0x29D37792).
///
/// Forwards two script arguments (a model id and an out-pointer) to the
/// engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bc5bc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
