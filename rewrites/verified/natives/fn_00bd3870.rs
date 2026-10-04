// original: 0x00bd3870 CREATE_CHECKPOINT
/// Script native `CREATE_CHECKPOINT` (hash 0x41F27499).
///
/// Forwards eight script arguments (an integer type tag and seven float bit-patterns). The original shuffles the floats through stack temporaries to lay out two by-value vectors; the net effect is an in-order forward to the engine and stores its full 32-bit answer into the return slot.
///
/// Unlike the boolean natives, this handler keeps the whole 32-bit answer (`mov`, not `movzx`).
export!(cdecl, rw_00bd3870(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
