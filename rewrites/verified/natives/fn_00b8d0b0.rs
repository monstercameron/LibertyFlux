// original: 0x00B8D0B0 PRINT_STRING_WITH_LITERAL_STRING
// PRINT_STRING_WITH_LITERAL_STRING: forwards 4 script words to the engine.
export!(cdecl, rw_fn_b8d0b0(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
    }
});
