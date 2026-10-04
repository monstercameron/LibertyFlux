// original: 0x00d2bb00 CTaskComplexMoveGetOntoMainNavMesh::vf1
/// Factory: allocate through the global heap; when that fails return null,
/// else placement-construct with (`[this+0x18]`, `this+0x30`, `[this+0x40]`)
/// and return the constructor's answer.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2bb00(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const HEAP_GLOB: u32 = 0x0167e2a0;
        let heap = lf_checker_rt::global::<u32>(HEAP_GLOB).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, heap);
        if obj == 0 {
            return 0;
        }
        let f = ((this + 0x18) as *const u32).read_unaligned();
        let w = ((this + 0x40) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CTOR, u32, obj, f, this + 0x30, w)
    }
});
