// original: 0x005BB760 NativeImpl_SET_PLAYER_SETTINGS_GENRE_2
/// Return one when a nonnegative input equals either of the two global
/// player-setting identifiers; return zero for negative inputs and all
/// other values. The signedness check is on the 32-bit input.
///
/// The original is cdecl with one 32-bit argument and an 8-bit result.
lf_checker_rt::export!(cdecl, rw_005bb760(value: u32) -> u8 {
    const PLAYER_SETTING_A_VA: u32 = 0x012f_a458;
    const PLAYER_SETTING_B_VA: u32 = 0x012f_a3bc;
    if (value as i32) < 0 {
        return 0;
    }
    let first = unsafe { lf_checker_rt::global::<u32>(PLAYER_SETTING_A_VA).read() };
    let second = unsafe { lf_checker_rt::global::<u32>(PLAYER_SETTING_B_VA).read() };
    u8::from(value == first || value == second)
});
