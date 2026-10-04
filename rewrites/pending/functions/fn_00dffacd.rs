// original: 0x00dffacd mbstowcs_s
/// Convert a multibyte string to wide characters (secure variant).
///
/// Forwards all five arguments plus a trailing zero (the default locale) to
/// the locale-aware conversion worker, returning its result.
export!(cdecl, rw_00dffacd(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a0, a1, a2, a3, a4, 0) }
});
