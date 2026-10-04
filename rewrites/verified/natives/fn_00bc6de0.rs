// original: 0x00bc6de0 IS_VEHICLE_EXTRA_TURNED_ON
/// Test whether a vehicle extra is turned on.
///
/// Forwards the vehicle handle and extra id (arguments 0-1) to the engine,
/// keeps only the low byte of its boolean answer and stores it in the return
/// slot. Returns the return-slot address, as the original leaves it in `eax`.
export!(cdecl, rw_00bc6de0(ctx: u32) -> u32 {
    unsafe {
        let base = ctx as *const u32;
        let ret_slot = *base as *mut u32;
        let args = *base.add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
