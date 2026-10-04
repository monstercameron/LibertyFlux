// original: 0x00d2b200 CTaskComplexUseDropDownOnRoute::vf0
/// Deleting destructor: run the destructor, then free `this` through the
/// global heap when bit 0 of `flags` is set. Returns `this`.
///
/// Thiscall, one stack word. Same shape as 0x00d2b1d0.
lf_checker_rt::export!(thiscall, rw_00d2b200(this: u32, flags: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const FREE: u32 = 2;
        const HEAP_GLOB: u32 = 0x0167e2a0;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flags & 1 != 0 {
            let heap = lf_checker_rt::global::<u32>(HEAP_GLOB).read_unaligned();
            lf_checker_rt::callee_thiscall!(FREE, u32, heap, this);
        }
        this
    }
});
