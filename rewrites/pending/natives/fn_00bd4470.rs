// original: 0x00bd4470 START_PTFX_ON_VEH
/// Native handler `START_PTFX_ON_VEH`: invokes a shared marshal thunk with the
/// engine callback address and the call context.
export!(cdecl, rw_00bd4470(ctx: *const u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(0xBD6350), ctx as u32);
        0
    }
});
