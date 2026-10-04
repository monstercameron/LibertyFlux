// original: 0x009faab0 playstat_object_reinit_tail
/// Reinitialise the object, then hand off to the shared finaliser.
///
/// Runs the object's reinitialiser and jumps to the shared finaliser,
/// whose answer becomes this function's result. (Only the 16-byte entry
/// body is verified here; the bytes past its tail jump belong to the next
/// function and never run from this entry.)
export!(thiscall, rw_009faab0(this_ptr: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        callee_thiscall!(2, u32, this_ptr)
    }
});
