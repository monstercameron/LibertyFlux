// original: 0x00ba2710 SET_ROOM_FOR_CHAR_BY_KEY
/// Script native `SET_ROOM_FOR_CHAR_BY_KEY` (hash 0x620C26D8).
///
/// Forwards two script arguments (a character handle and a room key) to
/// the engine. No return slot is written.
export!(cdecl, rw_00ba2710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});

// Honesty mutants below: verification controls, not deliverables. Each one
// is deliberately wrong and the checker must catch it. They carry no
// `// original:` marker so finalize.py does not split them into out/.

/// Mutant of the thunk rewrite: forwards a shifted pointer. Must fail on
/// the `calls` channel (argument and snapshot differ).
export!(cdecl, mut_00a01950_badfwd(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, 0, (ctx as u32).wrapping_add(4))
});

/// Mutant of the emergency-services flag rewrite: writes the inverted bit.
/// Must fail on the `globals` channel.
export!(cdecl, mut_005e7300_inv(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        *global::<u8>(0x105c6ea) = u8::from(*args == 0);
        args as u32
    }
});

/// Mutant of the CAN_PED_SHIMMY_IN_DIRECTION rewrite: stores the full
/// engine answer instead of its low byte. Must fail on the `heap` channel
/// (scripted answers with zero low byte but nonzero full word).
export!(cdecl, mut_00bb89a0_fullstore(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        slot as u32
    }
});
