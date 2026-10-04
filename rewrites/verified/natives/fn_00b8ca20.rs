// original: 0x00b8ca20 GET_NTH_INTEGER_IN_STRING
/// Script native `GET_NTH_INTEGER_IN_STRING` (hash 0x301545FD).
///
/// String + index; stores full answer.
///
/// Stores the engine's full 32-bit answer into the return slot.
export!(cdecl, rw_00b8ca20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
