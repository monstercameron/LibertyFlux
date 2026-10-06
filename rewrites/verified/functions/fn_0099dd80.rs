// original: 0x0099DD80 audSpeechAudioEntity::~audSpeechAudioEntity__deleting

/// Deleting destructor of the speech audio entity (vtable slot 0).
///
/// Runs the entity teardown (callee 1, thiscall/0) and, when the low bit of
/// the flags word is set, releases the object itself through the heap free
/// (callee 2, cdecl/1). Returns the object pointer in all cases. Thiscall
/// with one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_0099DD80(this: u32, flags: u32) -> u32 {
    unsafe {
        const TEARDOWN_CALLEE: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const DELETE_FLAG: u32 = 1;
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        if (flags & DELETE_FLAG) != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, this);
        }
        this
    }
});
