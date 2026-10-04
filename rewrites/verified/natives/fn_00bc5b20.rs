// original: 0x00bc5b20 GET_CAR_HEADING
/// Read a vehicle's heading into an out-parameter.
///
/// Forwards the vehicle handle (argument 0) and the out-pointer (argument 1)
/// to the engine implementation, which writes the heading through it. Returns
/// whatever the engine call returned.
export!(cdecl, rw_00bc5b20(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
