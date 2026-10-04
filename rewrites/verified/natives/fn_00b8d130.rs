// original: 0x00B8D130 PRINT_STRING_WITH_TWO_LITERAL_STRINGS_NOW
// PRINT_STRING_WITH_TWO_LITERAL_STRINGS_NOW: forwards 5 script words.
export!(cdecl, rw_fn_b8d130(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4)
        );
    }
});
