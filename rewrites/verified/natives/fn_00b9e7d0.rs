// original: 0x00b9e7d0 CREATE_NM_MESSAGE
/// Native handler `CREATE_NM_MESSAGE`: create a natural-motion message: forward the two message words.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00b9e7d0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1),);
        ans
    }
});
