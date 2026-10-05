// original: 0x00c7aa60 scenario_task_create_and_queue

/// Create a scenario task from a kind/parameter pair and queue it.
/// `spec` points at [kind, param]. Any kind other than 0 or 1 returns
/// at once with the kind in EAX. Otherwise a task object is allocated
/// through the heap singleton: allocation failure yields a null task,
/// else kind 0 constructs via the default constructor callee with tag
/// `0x34` and kind 1 via the parameterised constructor callee with tag
/// `0x59` and the parameter. Finally (task, 4, 0) is queued on the
/// receiver at `[obj+0x224]+0x44`, whose answer is returned.
/// Original: 0x00c7aa60 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c7aa60(spec: u32, obj: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const TAG_DEFAULT: u32 = 0x34;
        const TAG_PARAM: u32 = 0x59;
        const OFF_PARAM: u32 = 4;
        const OFF_RECV: u32 = 0x224;
        const RECV_BIAS: u32 = 0x44;
        const ALLOC: u32 = 1;
        const CTOR_DEFAULT: u32 = 2;
        const CTOR_PARAM: u32 = 3;
        const ENQUEUE: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let kind = rd32(spec) as i32;
        if kind != 0 && kind != 1 {
            return kind as u32;
        }
        let heap = rd32(lf_checker_rt::relocated(HEAP));
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, heap);
        let task = if slot == 0 {
            0
        } else if kind == 0 {
            lf_checker_rt::callee_thiscall!(CTOR_DEFAULT, u32, slot, TAG_DEFAULT)
        } else {
            let param = rd32(spec.wrapping_add(OFF_PARAM));
            lf_checker_rt::callee_thiscall!(CTOR_PARAM, u32, slot, TAG_PARAM, param)
        };
        let recv = rd32(obj.wrapping_add(OFF_RECV)).wrapping_add(RECV_BIAS);
        lf_checker_rt::callee_thiscall!(ENQUEUE, u32, recv, task, 4, 0)
    }
});
