// original: 0x00c484c0 ccamcinematic_dtor (proposed)
/// Tear down a cinematic camera, tail-jumping to the base destructor.
///
/// Plants the `CCamCinematic` vtable, resets the two shared scale
/// globals to 1.0, calls the member destructor (callee 1), then
/// tail-calls the base destructor (callee 2) and returns its result.
///
/// Original: thiscall, no stack arguments, ends in a tail jump.
lf_checker_rt::export!(thiscall, rw_00c484c0(this: u32) -> u32 {
    const VTABLE_CCAMCINEMATIC: u32 = 0x00ec95a0;
    const SCALE_A: u32 = 0x01032350;
    const SCALE_B: u32 = 0x0103234c;
    const ONE: u32 = 0x3f80_0000;
    const MEMBER_DTOR: u32 = 1;
    const BASE_DTOR: u32 = 2;
    unsafe {
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_CCAMCINEMATIC));
        lf_checker_rt::global::<u32>(SCALE_A).write_unaligned(ONE);
        lf_checker_rt::global::<u32>(SCALE_B).write_unaligned(ONE);
        lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, this);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
