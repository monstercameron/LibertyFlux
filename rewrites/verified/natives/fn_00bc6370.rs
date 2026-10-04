// original: 0x00bc6370 GET_RANDOM_CAR_OF_TYPE_IN_AREA_NO_SAVE
/// Script native handler `GET_RANDOM_CAR_OF_TYPE_IN_AREA_NO_SAVE`.
///
/// Forwards 6 script arguments to the engine function; no return slot.
/// Float arguments pass through by value as raw bits (bit-exact copies).
/// handler function: `0x00bc6370`, engine call site: `0x00bc63aa`.
export!(cdecl, rw_bc6370(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let x0 = *args.add(0);
        let y0 = *args.add(1);
        let x1 = *args.add(2);
        let y1 = *args.add(3);
        let model = *args.add(4);
        let car_out = *args.add(5);
        callee_cdecl!(1, u32, x0, y0, x1, y1, model, car_out)
    }
});
