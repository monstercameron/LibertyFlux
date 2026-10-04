// original: 0x00BC66A0 IS_CAR_DOOR_DAMAGED
/// Tests whether a vehicle door is damaged (0 or 1 in return slot).
///
/// Forwards the vehicle handle and door index to the engine function,
/// zero-extends its low byte and stores that in the return slot. Returns
/// the return-slot pointer (the original reloads it into `eax`).
lf_rn26_rt::export!(cdecl, rw_00BC66A0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let ans: u32 = lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1));
        let ret = *(ctx as *const *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
