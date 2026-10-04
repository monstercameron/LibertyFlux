// original: 0x00b9f4e0 GET_PED_TYPE
/// Script native `GET_PED_TYPE` (hash 0x18F477E1).
///
/// Forwards two script arguments (a ped handle and an out slot) to the engine. No return slot is written.
export!(cdecl, rw_00b9f4e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
