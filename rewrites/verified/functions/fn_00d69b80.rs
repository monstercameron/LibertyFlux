// original: 0x00d69b80 CReplayRolloverMessage::vf0
/// Deleting destructor slot of the rollover message (original 0x00D69B80, thiscall/1, scalar deleting
/// destructor).
///
/// Runs the real destructor through callee 1, then frees the object through
/// callee 2 when the low bit of `flags` is set. Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00d69b80(this_ptr: u32, flags: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
        if (flags & 1) != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, this_ptr);
        }
        this_ptr
    }
});
