// original: 0x00BC8190 STOP_VEHICLE_ALWAYS_RENDER
/// F24 STOP_VEHICLE_ALWAYS_RENDER: forwards args[0], void.
export!(cdecl, rn10_stop_vehicle_always_render(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args);
        0
    }
});
