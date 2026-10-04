// original: 0x00dfc3e0 atof
/// `atof` front end: forward the string with a zero second argument.
export!(cdecl, rw_00dfc3e0(text: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, text, 0) }
});
