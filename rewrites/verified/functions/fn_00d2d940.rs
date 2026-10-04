// original: 0x00d2d940 CTaskComplexMoveGetOntoMainNavMesh::vf19
/// Refresh the task for subject `s`: free the handles at `+0x50`/`+0x54`
/// when set, rebuild the context at `+0x54` from `[s+0x20]+0x30` with
/// weight 30.0, mark `+0x58`, and forward (`0x11a`, `s`) to the shared
/// routine, returning its answer.
///
/// Thiscall, one stack word (pointer).
lf_checker_rt::export!(thiscall, rw_00d2d940(this: u32, s: u32) -> u32 {
    unsafe {
        const FREE: u32 = 1;
        const BUILD: u32 = 2;
        const ROUTINE: u32 = 3;
        const TAG: u32 = 0x11a;
        const WEIGHT: u32 = 0x41f00000;
        for off in [0x50u32, 0x54] {
            let h = ((this + off) as *const u32).read_unaligned();
            if h != 0 {
                lf_checker_rt::callee_cdecl!(FREE, u32, h);
                ((this + off) as *mut u32).write_unaligned(0);
            }
        }
        let base = ((s + 0x20) as *const u32).read_unaligned().wrapping_add(0x30);
        let ctx: u32 = lf_checker_rt::callee_cdecl!(BUILD, u32, base, WEIGHT, s);
        ((this + 0x54) as *mut u32).write_unaligned(ctx);
        ((this + 0x58) as *mut u32).write_unaligned(1);
        lf_checker_rt::callee_thiscall!(ROUTINE, u32, this, TAG, s)
    }
});
