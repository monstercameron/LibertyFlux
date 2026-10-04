// original: 0x00b87590 SET_CAM_POINT_OFFSET_IS_RELATIVE
/// Script native `SET_CAM_POINT_OFFSET_IS_RELATIVE` (hash 0x12F20552).
///
/// Forwards two script arguments (a camera handle and a boolean flag) to the
/// engine. The flag is coerced with `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00b87590(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
