// original: 0x00d547b0 CCamShake::vf0

/// Camera-object deleting destructor (slot 0 of the class virtual table).
///
/// Runs the class destructor, then frees the object through the game's heap
/// when bit 0 of `flags` is set, and returns `this`. `this` points to the
/// object; the destructor runs first with `this` in ECX and returns nothing
/// observed. When the flag bit is set, the heap manager pointer is read from
/// the global word `HEAP_MGR` (a relocated data address) and the object is
/// handed to the heap free routine with the manager in ECX. With the bit
/// clear the object is only destroyed, never freed.
///
/// Original: 0x00d547b0 (thiscall, one stack argument, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d547b0(this: u32, flags: u32) -> u32 {
    unsafe {
        /// Class destructor (intercepted; thiscall, no arguments).
        const CLASS_DTOR: u32 = 1;
        /// Game heap free (intercepted; thiscall, frees its stack argument).
        const HEAP_FREE: u32 = 2;
        /// Heap manager pointer (global data word; relocated at load).
        const HEAP_MGR: u32 = 0x012fb1a0;
        lf_checker_rt::callee_thiscall!(CLASS_DTOR, u32, this);
        if flags & 1 != 0 {
            let mgr = (lf_checker_rt::global::<u32>(HEAP_MGR) as *const u32).read();
            lf_checker_rt::callee_thiscall!(HEAP_FREE, u32, mgr, this);
        }
        this
    }
});
