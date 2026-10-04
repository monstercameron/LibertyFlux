// original: 0x0099dd80 aud_speech_audio_entity_deleting_dtor
/// Deleting destructor of the speech audio entity.
///
/// Runs the teardown through callee 1, frees the object through callee 2
/// when the low bit of the flag word is set, and returns the object pointer.
export!(thiscall, rw_0099dd80(this: u32, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if (flags & 1) != 0 {
            callee_cdecl!(2, u32, this);
        }
        this
    }
});
