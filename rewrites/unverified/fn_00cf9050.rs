// original: 0x00cf9050 climb_worker_build_by_kind (proposed)

/// Builds a climb worker by kind id: kind 0x11f builds the ladder worker
/// (mapping the state word at `+0x14` of 3 to slot 2 and of 4 to slot 3,
/// anything else to slot 1), kind 0xcb builds the 8.0f-parameter worker, and
/// any other kind returns null. Allocation failure also returns null.
///
/// Original: 0x00cf9050 (thiscall: ecx holds the object, two stack words of
/// which only the first is read).
lf_checker_rt::export!(thiscall, rw_00cf9050(this: u32, kind: u32, _unused: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167e2a0;
        const NEW_CALLEE: u32 = 1;
        const LADDER_CTOR: u32 = 2;
        const PARAM_CTOR: u32 = 3;
        const LADDER_KIND: u32 = 0x11f;
        const PARAM_KIND: u32 = 0xcb;
        const PARAM_FLOAT: u32 = 0x4100_0000; // 8.0f
        const PARAM_SIZE: u32 = 0x64;
        if kind == PARAM_KIND {
            let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
            let obj = lf_checker_rt::callee_thiscall!(NEW_CALLEE, u32, heap);
            if obj == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(PARAM_CTOR, u32, obj, PARAM_SIZE, 0, 0, PARAM_FLOAT);
        }
        if kind != LADDER_KIND {
            return 0;
        }
        let state = ((this + 0x14) as *const u32).read_unaligned();
        let slot = if state == 3 {
            2
        } else if state == 4 {
            3
        } else {
            1
        };
        let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(NEW_CALLEE, u32, heap);
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(LADDER_CTOR, u32, obj, slot)
    }
});
