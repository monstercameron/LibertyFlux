// original: 0x00BB7590 REQUEST_COLLISION_FOR_MODEL
// REQUEST_COLLISION_FOR_MODEL: forwards the model word. No return.
export!(cdecl, rw_fn_bb7590(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args);
    }
});
