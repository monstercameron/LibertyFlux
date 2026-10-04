// original: 0x005e8190 ADD_FIRST_N_CHARACTERS_OF_STRING_TO_HTML_SCRIPT_OBJECT
/// Script native `ADD_FIRST_N_CHARACTERS_OF_STRING_TO_HTML_SCRIPT_OBJECT`
/// (hash 0x75FC34EF).
///
/// Forwards three script arguments (an HTML script-object handle, a string
/// and a character count) to the engine. No return slot is written.
export!(cdecl, rw_005e8190(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
