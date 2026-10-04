// original: 0x00b9feb0 IS_CHAR_TOUCHING_VEHICLE
/// Script native handler `IS_CHAR_TOUCHING_VEHICLE`.
///
/// Forwards 2 script arguments to the engine function and stores the
/// low byte of its answer (zero-extended) into the return slot.
/// handler function: `0x00b9feb0`, engine call site: `0x00b9febd`.
export!(cdecl, rw_b9feb0(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let ped = *args.add(0);
        let vehicle = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, ped, vehicle);
        *(*ctx).ret_ptr = answer & 0xFF;
        (*ctx).ret_ptr as u32
    }
});
