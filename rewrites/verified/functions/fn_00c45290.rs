// original: 0x00c45290 CIntermezzoEventStuntJump::vf2
/// Release the stunt-jump event's camera and set the shared camera blend rate
/// to the exact single-precision bits for 0.2. The release helper is thiscall
/// with no stack arguments; the method itself returns void.
lf_checker_rt::export!(thiscall, rw_00c45290(this: u32) -> u32 {
    const RELEASE_CAMERA: u32 = 1;
    const CAMERA_BLEND_RATE: u32 = 0x01048ae8;
    const RATE_BITS: u32 = 0x3e4ccccd;
    lf_checker_rt::callee_thiscall!(RELEASE_CAMERA, u32, this);
    unsafe {
        lf_checker_rt::global::<u32>(CAMERA_BLEND_RATE).write_unaligned(RATE_BITS);
    }
    0
});
