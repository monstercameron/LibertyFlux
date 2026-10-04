// original: 0x00bbac90 TASK_TOGGLE_PED_THREAT_SCANNER
/// Script native `TASK_TOGGLE_PED_THREAT_SCANNER` (hash 0x5D515C4D).
///
/// Forwards four script arguments to the engine: a character handle and three boolean flags coerced with `arg != 0`. The third flag uses the stack-slot quirk (Quirk (observed): the handler coerces the flag into the low byte of its own incoming stack slot and pushes the whole dword, so the pushed word's high bytes repeat the context pointer. The engine reads only the low byte (Inferred); the full dword is reproduced here for bit-exact outgoing-call matching.), so its pushed word's high bytes repeat the context pointer; the first two flags are coerced into fresh stack temporaries whose upper bytes are zero under the checker's defined stack fill, so they forward as clean 0/1 words. No return slot is written.
export!(cdecl, rw_00bbac90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag3 = u32::from(*args.add(3) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag3;
        callee_cdecl!(1, u32, *args, u32::from(*args.add(1) != 0), u32::from(*args.add(2) != 0), quirked)
    }
});
