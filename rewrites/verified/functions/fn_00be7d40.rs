// original: 0x00BE7D40 task_emit_float_to_pair (proposed)
/// Forward a subject and a float to the mover and to this task's target.
///
/// Same two-half shape as 0x00BE7CE0 with two stack words: the subject
/// `a` and a float word `f`, which travels through a vector register in
/// the original but is just bits here. The gate/index/open sequence is
/// identical; the emit slot is `+0x28` and takes the float word alone.
/// No result.
///
/// Original: 0x00BE7D40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00BE7D40(obj: u32, a: u32, f: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x224;
        const BASE_BIAS: u32 = 0x44;
        const OWN: u32 = 0x14;
        const SLOT_OPEN: u32 = 0x30;
        const SLOT_EMIT: u32 = 0x28;
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
        unsafe fn emit(o: u32, f: u32) -> u32 {
            unsafe {
                let vt = (o as *const u32).read_unaligned();
                let g: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        ((vt + SLOT_EMIT) as *const u32).read_unaligned() as usize,
                    );
                g(o, f)
            }
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, obj, a);
        if (ok as u8) != 0 {
            let base = ((a + FIRST) as *const u32).read_unaligned();
            let o1: u32 =
                lf_checker_rt::callee_thiscall!(INDEX, u32, base.wrapping_add(BASE_BIAS), 1);
            let o2 = open(o1);
            let _: u32 = emit(o2, f);
        }
        let o3 = ((obj + OWN) as *const u32).read_unaligned();
        let o4 = open(o3);
        let _: u32 = emit(o4, f);
        0
    }
});

