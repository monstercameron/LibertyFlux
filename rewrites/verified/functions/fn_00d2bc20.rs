// original: 0x00d2bc20 CTaskComplexUseLadderOnRoute::vf1
/// Factory: allocate through the global heap; when that fails return null,
/// else placement-construct with (`[this+0x38]`, `this+0x20`, `[this+0x30]`,
/// `[this+0x34]`) and return the constructor's answer.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2bc20(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const HEAP_GLOB: u32 = 0x0167e2a0;
        let heap = lf_checker_rt::global::<u32>(HEAP_GLOB).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, heap);
        if obj == 0 {
            return 0;
        }
        let f0 = ((this + 0x38) as *const u32).read_unaligned();
        let f2 = ((this + 0x30) as *const u32).read_unaligned();
        let a3 = ((this + 0x34) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CTOR, u32, obj, f0, this + 0x20, f2, a3)
    }
});
