// original: 0x00B87A50 SET_GAME_CAMERA_CONTROLS_ACTIVE
// SET_GAME_CAMERA_CONTROLS_ACTIVE: forwards the coerced bool flag.
export!(cdecl, rw_fn_b87a50(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let arg = ((*args != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, arg);
    }
});
