// original: 0x00a018e0 REMOVE_ALL_PICKUPS_OF_TYPE
/// Script native `REMOVE_ALL_PICKUPS_OF_TYPE` (hash 0x03622640).
///
/// Forwards one script argument (a pickup type) to the engine. No return
/// slot is written. (The original cleans its one pushed argument with
/// `(an instruction of the original)`; the effect on the stack pointer is identical to the plain
/// cdecl return here.)
export!(cdecl, rw_00a018e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
