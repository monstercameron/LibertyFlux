// original: 0x00B8B010 ADD_WIDGET_FLOAT_READ_ONLY
/// Script native handler `ADD_WIDGET_FLOAT_READ_ONLY`.
///
/// Reads the argument array at ctx+8, passes arg0 integer, arg1 integer, calls the engine worker
/// (intercepted by the checker), and writes the engine answer to the return slot.
export!(cdecl, rw_00b8b010(ctx: *const u8) -> u32 {
    unsafe {
        let args = *(ctx.add(8) as *const *const u32);
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let ans: u32 = callee_cdecl!(1, u32, a0, a1,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans;
        ans
    }
});
