// original: 0x009cceb0 NativeImpl_HIGH_FALL_SCREAM
/// Play the HIGH_FALL scream for a handle, resolving it through the pool first.
///
/// A null handle does nothing and returns 0; otherwise the handle is
/// resolved through the pool, 0x570 is added to reach the speech object,
/// and the speech call's result is returned.
export!(cdecl, rw_009cceb0(handle: u32) -> u32 {
    const POOL_THIS: u32 = 0x018B6F1C;
    const SCREAM_EXTRA: u32 = 0x01284530;
    const SPEECH_OFF: u32 = 0x570;
    const TAG: u32 = 0x00E957D0;
    if handle == 0 {
        return 0;
    }
    let pool = unsafe { global::<u32>(POOL_THIS).read() };
    let extra = unsafe { global::<u32>(SCREAM_EXTRA).read() };
    let obj: u32 = callee_thiscall!(
        1, u32, pool, handle, relocated(TAG), 1, 1, extra, 0xFFFFFFFF, 0, 0,
        0x3F800000, 0, 0
    );
    callee_thiscall!(2, u32, obj.wrapping_add(SPEECH_OFF))
});
