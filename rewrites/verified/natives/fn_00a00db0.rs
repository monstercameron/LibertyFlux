// original: 0x00a00db0 GET_OBJECT_HEALTH
/// Native handler `GET_OBJECT_HEALTH`: read an object's health: forward the object handle and the out-pointer.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00a00db0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1),);
        ans
    }
});
