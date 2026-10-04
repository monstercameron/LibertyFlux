// original: 0x00dfc52b strtol_base10
/// Parse a string as a base-10 long.
///
/// Forwards the input string to the three-argument string-to-long
/// conversion with a null end-pointer and base 10, returning its result.
/// (Behaviour inferred from the outgoing call shape: `(string, NULL, 10)`.)
export!(cdecl, rw_00dfc52b(s: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, s, 0, 10) }
});
