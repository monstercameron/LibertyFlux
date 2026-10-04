// original: 0x00c5cde0 CTaskComplexDestroyCarArmed::vf0

/// Deleting destructor of the armed destroy-car complex task.
///
/// Runs the class destructor through callee 1, then frees the object through
/// the global heap (callee 2) when the low bit of `flag` is set. Returns `this`.
/// The heap handle is read from its global slot each time. `flag` is read as
/// one byte.
///
/// Original: 0x00c5cde0 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c5cde0(this: u32, flag: u32) -> u32 {
    unsafe {
        const DTOR_CALLEE: u32 = 1;
        const DELETE_CALLEE: u32 = 2;
        const HEAP_SLOT: u32 = 0x167e2a0;
        lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if (flag & 1) != 0 {
            let heap = lf_checker_rt::global::<u32>(HEAP_SLOT).read();
            lf_checker_rt::callee_thiscall!(DELETE_CALLEE, u32, heap, this);
        }
        this
    }
});

