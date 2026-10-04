// original: 0x00bd2240 GET_NUMBER_OF_FIRES_IN_RANGE
/// Native handler `GET_NUMBER_OF_FIRES_IN_RANGE`: count fires near a point: forward x, y, z and radius, return the count.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bd2240(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3),);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans;
        ans
    }
});
