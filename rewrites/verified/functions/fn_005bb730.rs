// original: 0x005BB730 player_setting_identifier_b_matches
/// Compare this object's identifier with the second player-setting identifier.
/// If the global object pointer is null, call the shared fallback getter and
/// compare its result instead. The stored identifier and fallback result are
/// unsigned 32-bit values; the function returns a one-byte equality result.
///
/// The original is a no-argument cdecl function. Its only outgoing call is a
/// direct zero-argument cdecl helper used on the null-object path.
lf_checker_rt::export!(cdecl, rw_005bb730() -> u8 {
    const OBJECT_PTR_VA: u32 = 0x018b_6eb0;
    const IDENTIFIER_VA: u32 = 0x012f_a3bc;
    const FALLBACK_HELPER_ID: u32 = 1;
    const IDENTIFIER_OFFSET: u32 = 0x14;

    let object = unsafe { lf_checker_rt::global::<u32>(OBJECT_PTR_VA).read() };
    let expected = unsafe { lf_checker_rt::global::<u32>(IDENTIFIER_VA).read() };
    let actual = if object == 0 {
        lf_checker_rt::callee_cdecl!(FALLBACK_HELPER_ID, u32,)
    } else {
        unsafe { core::ptr::read_unaligned(object.wrapping_add(IDENTIFIER_OFFSET) as *const u32) }
    };
    (actual == expected) as u8
});
