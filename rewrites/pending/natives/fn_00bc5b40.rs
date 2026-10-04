// original: 0x00BC5B40 GET_CAR_HEALTH
/// Gets vehicle health into the script out-parameter.
///
/// Forwards the vehicle handle and the out-slot word to the engine
/// function, which writes health through the out-slot.
lf_rn26_rt::export!(cdecl, rw_00BC5B40(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
