// original: 0x00dfe76f int_format_signed
// rs03f19: signed integer formatting front end (cdecl/3).
//
// Formats `v` into `buf` in the given base through the formatting core
// (stdcall/4): base ten with a negative value selects the signed path, every
// other combination the unsigned one. Returns `buf` unchanged.
export!(cdecl, rw_rs03f19(v: u32, buf: u32, base: u32) -> u32 {
    unsafe {
        if base == 10 && (v as i32) < 0 {
            callee_stdcall!(1, u32, v, buf, 10, 1);
        } else {
            callee_stdcall!(1, u32, v, buf, base, 0);
        }
        buf
    }
});
