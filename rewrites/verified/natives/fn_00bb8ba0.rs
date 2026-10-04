// original: 0x00bb8ba0 GET_PED_AT_HEAD_OF_QUEUE
/// Script native `GET_PED_AT_HEAD_OF_QUEUE` (hash 0x09FE0380).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb8ba0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
