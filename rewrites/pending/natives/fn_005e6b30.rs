// original: 0x005E6B30 TASK_CHAR_ARREST_CHAR
//
// Forwards the two character handles to the engine task routine.
// No return value.
export!(cdecl, rw_005e6b30(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
