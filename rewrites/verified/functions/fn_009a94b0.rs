// original: 0x009a94b0 audSpeechManager::vf0
/// Destroy the speech manager, freeing it when the flags ask.
///
/// Runs the destructor (stubbed, thiscall/0 on this object); when bit 0
/// of `flags` is set the object is also freed (stubbed, cdecl/1).
/// Returns the object pointer. Thiscall, one stack word.
export!(thiscall, rw_009A94B0(this: u32, flags: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let _: u32 = callee_cdecl!(2, u32, this);
        }
        this
    }
});
