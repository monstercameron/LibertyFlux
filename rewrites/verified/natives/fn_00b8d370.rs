// original: 0x00b8d370 PRINT_WITH_NUMBER_NOW
/// Native handler `PRINT_WITH_NUMBER_NOW`: print a message with a number: forward the text key, number, time and flags.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00b8d370(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3),);
        ans
    }
});
