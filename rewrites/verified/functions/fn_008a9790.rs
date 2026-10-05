// original: 0x008A9790 audio_lock_guard_store (proposed)
// Guard address skipped in the contract (each side builds the guard in its
// own frame); guard contents compared at destructor time.

/// Under the voice-list lock, record `arg` as the latest index.
///
/// Builds a lock guard on the stack over the critical section at
/// `this+0x3210` (guard constructor `0x403fa0`, destructor `0x403fd0`). When
/// `arg` is not `0xffff`, the low word of the previous index slot
/// `this+0x320c` is copied to `this+arg*4` and `arg` becomes the new slot
/// value. No return value. Original is thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_008A9790(this: u32, arg: u32) -> u32 {
    const GUARD_CTOR: u32 = 1;
    const GUARD_DTOR: u32 = 2;
    const CS: u32 = 0x3210;
    const LAST: u32 = 0x320c;
    const NONE: u32 = 0xffff;
    unsafe {
        let mut guard = [0u32; 2];
        lf_checker_rt::callee_thiscall!(GUARD_CTOR, u32, &mut guard as *mut _ as u32, this + CS);
        if arg != NONE {
            let prev = ((this + LAST) as *const u16).read_unaligned();
            ((this + arg * 4) as *mut u16).write_unaligned(prev);
            ((this + LAST) as *mut u32).write_unaligned(arg);
        }
        lf_checker_rt::callee_thiscall!(GUARD_DTOR, u32, &mut guard as *mut _ as u32);
    }
    0
});
