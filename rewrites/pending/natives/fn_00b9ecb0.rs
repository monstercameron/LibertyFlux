// original: 0x00b9ecb0 GET_CHAR_ANIM_EVENT_TIME
/// GET_CHAR_ANIM_EVENT_TIME: query an animation event time flag.
///
/// Native handler. Forwards ped, anim and event ids to the animation engine, stores the boolean answer through the return slot.
export!(cdecl, rw_00b9ecb0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let answer = callee_cdecl!(1, u32, a0, a1, a2, a3, a4);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
