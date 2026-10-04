// original: 0x00d2b290 CTaskSimplePathfindProblem::vf0
/// Deleting destructor: stamp vtable slot `+0`, run the base destructor,
/// then free `this` through the global heap when bit 0 of `flags` is set.
/// Returns `this`.
///
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00d2b290(this: u32, flags: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee1f3c;
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        const HEAP_GLOB: u32 = 0x0167e2a0;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & 1 != 0 {
            let heap = lf_checker_rt::global::<u32>(HEAP_GLOB).read_unaligned();
            lf_checker_rt::callee_thiscall!(FREE, u32, heap, this);
        }
        this
    }
});
