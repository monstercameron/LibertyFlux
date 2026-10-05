// original: 0x0093f1c0 stream_selected_flagged_word (proposed)

/// Return a word from the selected object when its flag bit is set.
///
/// Calls the selected-slot getter; a null object returns null. Otherwise
/// tests bit `FLAG_BIT` of the flag byte at offset `OBJ_FLAGS`: when clear
/// returns null, when set returns the word at offset `OBJ_WORD`.
///
/// Original: 0x0093f1c0 (cdecl, no arguments read; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093f1c0() -> u32 {
    const SELECTED_GETTER: u32 = 1;
    const OBJ_FLAGS: u32 = 0x26C;
    const FLAG_BIT: u8 = 4;
    const OBJ_WORD: u32 = 0xB30;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(SELECTED_GETTER, u32,);
        if obj == 0 {
            return 0;
        }
        let flags = ((obj + OBJ_FLAGS) as *const u8).read();
        if flags & FLAG_BIT == 0 {
            return 0;
        }
        ((obj + OBJ_WORD) as *const u32).read_unaligned()
    }
});
