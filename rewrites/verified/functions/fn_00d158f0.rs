// original: 0x00D158F0 CTaskSimpleLoadAnimPlayAnim::make_abortable

/// Remove the task's active animation player and clear its animation marker.
/// The task object arrives in ECX; three incoming stack words are ignored and
/// are removed by the thiscall epilogue. The function calls the registry with
/// the embedded player at +0x18 and sentinel -1, calls the player-release
/// helper with the task object and the exact float payload -4.0, clears the
/// marker at +0x14, and returns true in AL.
lf_checker_rt::export!(thiscall, rw_D158F0(this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u32 {
    const PLAYER_SUBOBJECT: u32 = 0x18;
    const MARKER_WORD: u32 = 0x14;
    const NO_REGISTRY_SLOT: u32 = u32::MAX;
    const RELEASE_BLEND_BITS: u32 = 0xC080_0000;
    let _registry_result = lf_checker_rt::callee_thiscall!(1, u32, this + PLAYER_SUBOBJECT, NO_REGISTRY_SLOT);
    let _release_result = lf_checker_rt::callee_thiscall!(2, u32, this, RELEASE_BLEND_BITS);
    unsafe { core::ptr::write_unaligned((this + MARKER_WORD) as *mut u32, 0); }
    1
});
