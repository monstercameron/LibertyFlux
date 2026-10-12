// original: 0x00D15920 CTaskSimplePlayUpperCombatAnim::make_abortable

/// Reset the upper-combat animation player's state and clear its marker.
/// The task object arrives in ECX; three incoming stack words are ignored and
/// removed by the thiscall epilogue. The function calls the registry with
/// the embedded player at +0x18 and sentinel -1, resets the animation player
/// on the task object, clears the marker at +0x14, and returns true in AL.
lf_checker_rt::export!(thiscall, rw_D15920(this: u32, _arg0: u32, _arg1: u32, _arg2: u32) -> u32 {
    const PLAYER_SUBOBJECT: u32 = 0x18;
    const MARKER_WORD: u32 = 0x14;
    const NO_REGISTRY_SLOT: u32 = u32::MAX;
    let _registry_result = lf_checker_rt::callee_thiscall!(1, u32, this + PLAYER_SUBOBJECT, NO_REGISTRY_SLOT);
    let _reset_result = lf_checker_rt::callee_thiscall!(2, u32, this);
    unsafe { core::ptr::write_unaligned((this + MARKER_WORD) as *mut u32, 0); }
    1
});
