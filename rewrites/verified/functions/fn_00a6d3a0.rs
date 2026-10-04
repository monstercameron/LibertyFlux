// original: 0x00a6d3a0 CTaskSimplePauseSystemTimer::vf0

/// Deleting destructor of the pause-system-timer task.
///
/// Installs the base task vtable (`BASE_VTABLE`) on the object, runs the
/// base destructor (callee 1) with the object pointer, and when the low bit
/// of the `freeing` flag is set releases the object through the allocator
/// reached via `ALLOCATOR_ANCHOR` (callee 2, called with the anchor's value
/// in ecx and the object pointer as its stack argument). Returns the object
/// pointer in all cases.
///
/// The vtable stamp is a file VA the loader relocates, so it is written
/// through `relocated`, not as a raw constant.
///
/// Original: thiscall, one stack word (`freeing`), callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a6d3a0(this: u32, freeing: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0x00e9_edbc;
        const ALLOCATOR_ANCHOR: u32 = 0x0167_e2a0;
        const FREE_FLAG: u32 = 1;
        const BASE_DTOR: u32 = 1;
        const RELEASE: u32 = 2;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if freeing & FREE_FLAG != 0 {
            let anchor = (lf_checker_rt::relocated(ALLOCATOR_ANCHOR) as *const u32)
                .read_unaligned();
            lf_checker_rt::callee_thiscall!(RELEASE, u32, anchor, this);
        }
        this
    }
});
