// original: 0x00e63980 dyninit_array11_g11d7000
/// Construct 11 strided global objects, then register one teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e63980() -> u32 {
    for i in 0..11u32 {
        callee_thiscall!(0, u32, relocated(0x11D7000).wrapping_add(i.wrapping_mul(0x70)));
    }
    callee_cdecl!(1, u32, relocated(0xE71540))
});
