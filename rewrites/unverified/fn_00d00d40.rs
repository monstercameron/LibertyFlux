// original: 0x00d00d40 CTaskComplexBeArrestedAndDrivenAway::vf1

/// Clone helper for CTaskComplexBeArrestedAndDrivenAway: allocates a fresh object from the game heap and
/// copy-constructs it from field `+0x14` of `this`.
///
/// `this` (ECX) is the source object. The global game heap pointer is loaded
/// and the allocator (callee 1, thiscall, no stack args) runs with ECX =
/// heap. When it returns null the function returns 0. Otherwise the dword at
/// `this + 0x14` is pushed and the copy constructor (callee 2, thiscall, one
/// stack arg) runs with ECX = fresh object; its return value (in EAX) is the
/// function's result.
///
/// Original: 0x00d00d40 (thiscall, no stack args).
lf_checker_rt::export!(thiscall, rw_00d00d40(this: u32) -> u32 {
    unsafe {
        const HEAP_GLOBAL: u32 = 0x0167_E2A0;
        const SRC_FIELD: u32 = 0x14;
        const ALLOC_CALLEE: u32 = 1;
        const COPY_CALLEE: u32 = 2;
        let heap = lf_checker_rt::global::<u32>(HEAP_GLOBAL).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, heap);
        if fresh == 0 {
            return 0;
        }
        let src = (this.wrapping_add(SRC_FIELD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(COPY_CALLEE, u32, fresh, src)
    }
});
