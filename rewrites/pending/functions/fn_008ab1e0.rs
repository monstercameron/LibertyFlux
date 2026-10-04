// original: 0x008AB1E0 audio_sub_forward
/// Refresh this object, then tail-forward to the shared routine operating
/// on the sub-object at offset 0xFC, returning its result.
export!(thiscall, rw_008AB1E0(obj: *mut u8) -> u32 {
    callee_thiscall!(1, u32, obj as u32);
    callee_thiscall!(2, u32, (obj as u32).wrapping_add(0xFC))
});
