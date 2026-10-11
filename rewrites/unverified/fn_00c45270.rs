// original: 0x00c45270 CIntermezzoEventIntro::vf2
/// Stop an intro event, then clear the active-root flag bit at root+0x228+0x4c8
/// when the active-root helper returns a non-null pointer. The release helper
/// receives this in ECX; the root lookup is cdecl with no stack parameters.
lf_checker_rt::export!(thiscall, rw_00c45270(this: u32) -> u32 {
    const RELEASE_CAMERA: u32 = 1;
    const GET_ACTIVE_ROOT: u32 = 2;
    const ROOT_TARGET: u32 = 0x228;
    const TARGET_FLAGS: u32 = 0x4c8;
    lf_checker_rt::callee_thiscall!(RELEASE_CAMERA, u32, this);
    let root = lf_checker_rt::callee_cdecl!(GET_ACTIVE_ROOT, u32, );
    if root != 0 {
        unsafe {
            let target = ((root + ROOT_TARGET) as *const u32).read_unaligned();
            let flags = (target + TARGET_FLAGS) as *mut u32;
            flags.write_unaligned(flags.read_unaligned() & !1);
        }
    }
    0
});
