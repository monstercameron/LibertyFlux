// original: 0x00b87a50 SET_GAME_CAMERA_CONTROLS_ACTIVE
// SET_GAME_CAMERA_CONTROLS_ACTIVE: forwards the coerced bool flag.
export!(cdecl, rw_fn_b87a50(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let arg = (ctx as u32 & 0xFFFFFF00) | ((*args != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, arg);
    }
});
