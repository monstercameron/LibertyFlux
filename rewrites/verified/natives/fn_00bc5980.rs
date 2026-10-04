// original: 0x00bc5980 GET_CAR_COLOURS
/// Native handler `GET_CAR_COLOURS`: read a vehicle's colours: forward the vehicle handle and the two out-pointers (results go through the pointers).
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bc5980(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2),);
        ans
    }
});
