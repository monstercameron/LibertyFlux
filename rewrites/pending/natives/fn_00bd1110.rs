// original: 0x00bd1110 REMOVE_ALL_CHAR_WEAPONS
/// Remove every weapon from a character.
///
/// Reads the character handle from script argument 0 and forwards it to the
/// engine implementation. Returns whatever the engine call returned.
export!(cdecl, rw_00bd1110(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
