// original: 0x00D4DF30 CTaskSimpleDuck::vf5

// Event handler (vf5): acknowledges duck events, flags anything else.
///
/// The second stack argument selects the path. On event 2 the first argument
/// goes in ECX with (0, -1) to intercepted callee 1 and the result is 1. On
/// event 1, when the signed word at `+0x1c` exceeds -1 and the third argument
/// is non-null, that object is queried through its vtable slot at `+4`
/// (intercepted callee 2); a 0x31 answer with 0x137 at the object's `+0x10`
/// additionally triggers vtable slot `+0x34` (intercepted callee 3). Either
/// way callee 1 then runs as on event 2 and the result is 1. Any other event
/// sets the byte at `+0x2a` and yields 0.
///
/// Original: 0x00D4DF30 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d4df30(this: u32, a0: u32, ev: u32, a2: u32) -> u32 {
    unsafe {
        const DIRECT: u32 = 1;
        const QUERY: u32 = 2;
        const TRIGGER: u32 = 3;
        const QUERY_SLOT: u32 = 4;
        const TRIGGER_SLOT: u32 = 0x34;
        const WANT_CODE: u32 = 0x31;
        const WANT_STATE: u32 = 0x137;
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let addr = ((vt + slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj)
            }
        }
        if ev == 2 {
            lf_checker_rt::callee_thiscall!(DIRECT, u32, a0, 0, 0xFFFFFFFF);
            return 1;
        }
        if ev == 1 {
            if ((this + 0x1c) as *const i16).read_unaligned() > -1 && a2 != 0 {
                if vcall0(a2, QUERY_SLOT) == WANT_CODE
                    && ((a2 + 0x10) as *const u32).read_unaligned() == WANT_STATE
                {
                    vcall0(a2, TRIGGER_SLOT);
                }
            }
            lf_checker_rt::callee_thiscall!(DIRECT, u32, a0, 0, 0xFFFFFFFF);
            return 1;
        }
        ((this + 0x2a) as *mut u8).write(1);
        0
    }
});
