// original: 0x00BE7CE0 task_emit_to_pair (proposed)
/// Forward three words to the subject's mover and to this task's target.
///
/// `obj` points to this task, `a` to the subject and `b`/`c` are two
/// more words. The gate callee (thiscall on this with `a`) selects by
/// its low byte whether the first half runs: the index callee (thiscall
/// on `[a+0x224]+0x44` with 1) yields an object whose virtual slot
/// `+0x30` is called (thiscall, no stack words), and the result's slot
/// `+0x24` receives (`a`, `b`, `c`). The second half always runs the
/// same two virtual calls starting from the object at `[this+0x14]`.
/// No result.
///
/// Original: 0x00BE7CE0 (thiscall, three stack words). The virtual
/// targets are the checker's planted stubs on both sides.
lf_checker_rt::export!(thiscall, rw_00BE7CE0(obj: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x224;
        const BASE_BIAS: u32 = 0x44;
        const OWN: u32 = 0x14;
        const SLOT_OPEN: u32 = 0x30;
        const SLOT_EMIT: u32 = 0x24;
        const GATE: u32 = 1;
        const INDEX: u32 = 2;
        unsafe fn open(o: u32) -> u32 {
            unsafe {
                let vt = (o as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    ((vt + SLOT_OPEN) as *const u32).read_unaligned() as usize,
                );
                f(o)
            }
        }
        unsafe fn emit(o: u32, a: u32, b: u32, c: u32) -> u32 {
            unsafe {
                let vt = (o as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(
                        ((vt + SLOT_EMIT) as *const u32).read_unaligned() as usize,
                    );
                f(o, a, b, c)
            }
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, obj, a);
        if (ok as u8) != 0 {
            let base = ((a + FIRST) as *const u32).read_unaligned();
            let o1: u32 =
                lf_checker_rt::callee_thiscall!(INDEX, u32, base.wrapping_add(BASE_BIAS), 1);
            let o2 = open(o1);
            let _: u32 = emit(o2, a, b, c);
        }
        let o3 = ((obj + OWN) as *const u32).read_unaligned();
        let o4 = open(o3);
        let _: u32 = emit(o4, a, b, c);
        0
    }
});

