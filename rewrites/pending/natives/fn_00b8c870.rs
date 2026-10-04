// original: 0x00b8c870 GET_FIRST_N_CHARACTERS_OF_LITERAL_STRING
/// Script native `GET_FIRST_N_CHARACTERS_OF_LITERAL_STRING` (hash 0x42D249E3).
///
/// Forwards two script arguments (a string and a count) to the engine substring helper and stores its full answer into the return slot.
export!(cdecl, rw_00b8c870(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
