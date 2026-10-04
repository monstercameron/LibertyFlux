// original: 0x00b95270 TAN
/// Native handler `TAN`: passes one float (bits) to its engine
/// function and stores the float result (x87 ST0) to the return slot.
export!(cdecl, rw_00b95270(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let slot = *ctx as *mut f32;
        let r: f32 = callee_cdecl!(1, f32, *args);
        *slot = r;
        0
    }
});
