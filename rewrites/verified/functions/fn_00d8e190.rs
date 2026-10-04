// original: 0x00d8e190 audio_release_slot_a
/// Releases the sub-object in slot +8 when present, via the release helper.
export!(thiscall, rw_00d8e190(this: *const u32) -> u32 {
    unsafe {
        let p = *this.add(2);
        if p != 0 {
            callee_thiscall!(1, u32, p, 0)
        } else {
            0
        }
    }
});
