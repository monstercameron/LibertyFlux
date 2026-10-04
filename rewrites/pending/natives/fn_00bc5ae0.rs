// original: 0x00bc5ae0 GET_CAR_FORWARD_X
/// Script native `GET_CAR_FORWARD_X` (hash 0x47A21100).
///
/// Forwards two script arguments (a vehicle handle and an out word) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bc5ae0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
