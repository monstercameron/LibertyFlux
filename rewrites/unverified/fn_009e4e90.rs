// original: 0x009e4e90 CTaskSimpleMoveDoNothing::vf1

/// Allocate and initialise a move-do-nothing task, or null on failure.
///
/// Resolves the allocator (`thiscall` on the manager global, no stack
/// words); a null result returns 0. Otherwise initialises the object
/// (`thiscall` on it with 1), stores the payload from `[this + 0x20]`
/// at +0x20, plants the two vtable literals at +0 and +0x14 (written as
/// immediates by the original, so the rewrite holds the same constants)
/// and returns the object. `thiscall`, no stack words.
lf_checker_rt::export!(thiscall, rw_009e4e90(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0167e2a0;
        const PAYLOAD: u32 = 0x20;
        const VTABLE: u32 = 0x00e98794;
        const VTABLE2: u32 = 0x00e987e8;
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let obj = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
        if obj == 0 {
            return 0;
        }
        let payload = ((this + PAYLOAD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(INIT, u32, obj, 1u32);
        ((obj + PAYLOAD) as *mut u32).write_unaligned(payload);
        (obj as *mut u32).write_unaligned(VTABLE);
        ((obj + 0x14) as *mut u32).write_unaligned(VTABLE2);
        obj
    }
});
