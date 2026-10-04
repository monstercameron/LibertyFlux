// original: 0x00e63190 dyninit_array4_g117e700
/// Construct 4 strided global objects, then register one teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e63190() -> u32 {
    for i in 0..4u32 {
        callee_thiscall!(0, u32, relocated(0x117E700).wrapping_add(i.wrapping_mul(0x3A84)));
    }
    callee_cdecl!(1, u32, relocated(0xE71340))
});
