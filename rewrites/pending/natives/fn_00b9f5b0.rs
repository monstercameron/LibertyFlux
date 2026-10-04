// original: 0x00b9f5b0 GIVE_PED_HELMET
/// Give a ped a helmet: forward the script argument to the engine. No
/// return value.
export!(cdecl, rw_00b9f5b0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args);
        0
    }
});
