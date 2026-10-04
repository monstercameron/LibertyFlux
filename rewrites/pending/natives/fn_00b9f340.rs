// original: 0x00b9f340 GET_KEY_FOR_DUMMY_CHAR_IN_ROOM
/// Script native handler `GET_KEY_FOR_DUMMY_CHAR_IN_ROOM` (hash 0x74672C8C).
///
/// Forwards script arguments 0..1 to the engine worker and returns its answer.
export!(cdecl, rw_00b9f340(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
