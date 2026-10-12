// original: 0x00605630 rage::fragManager::vf0

/// Deleting destructor: run the real destructor, then free the object
/// through the thread allocator when the caller asked for it.
///
/// `this` in ECX, one stack argument holding the MSVC deleting-destructor
/// flag. The real destructor runs first (callee 1, thiscall, no stack
/// arguments, answer ignored). When bit 0 of the flag is set and `this`
/// is non-null, the object is released through the allocator reached via
/// TLS slot 0 (`ALLOC` at +0x08 on the thread state) by calling its
/// release entry (callee 2, thiscall, the object address as its one
/// argument). Returns `this` unchanged. Thiscall.
lf_checker_rt::export!(thiscall, rw_00605630(this: u32, flag: u32) -> u32 {
    unsafe {
        const DTOR_CALLEE: u32 = 1;
        // The release entry is checker callee 2, reached through the planted vtable.
        const ALLOC: u32 = 0x08;
        const FREE_SLOT: u32 = 0x0c;

        let _: u32 = lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flag & 1 != 0 && this != 0 {
            let state = lf_checker_rt::tls_slot(0);
            let alloc = ((state + ALLOC) as *const u32).read();
            let vtab = (alloc as *const u32).read();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((((vtab + FREE_SLOT) as *const u32).read()) as usize);
            let _: u32 = free(alloc, this);
        }
        this
    }
});
