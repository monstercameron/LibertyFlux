// original: 0x00685970 forward_to_00686e00
/// Forward a heap word and one stack word to the worker helper.
///
/// Loads the word past the object header from the heap object given as the
/// first word, passes it with the second word and the object pointer on
/// the stack (the helper cleans three stack words), and passes a pointer
/// to a two-word frame record (object pointer, heap object) in the object
/// register. Returns the helper's answer.
export!(thiscall, rw_00685970(this_: u32, obj: *const u32, second: u32) -> u32 {
    unsafe {
        let loaded = *obj.add(1);
        let frame = [this_, obj as u32];
        callee_thiscall!(1, u32, frame.as_ptr() as u32, loaded, second, this_)
    }
});
