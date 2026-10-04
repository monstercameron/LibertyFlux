// original: 0x00BE8080 anim_setup_direct (proposed)
/// Set up a blend slot unconditionally and link this object to it.
///
/// `obj` points to the task object and `target` to the subject. The
/// compute callee runs on the mover at `[target+0x78]` with the slot
/// words at `+0x68`/`+0x6c`, the 4.0 weight and -1; its handle is stored
/// at `+0x60`, and the attach callee links this object (thiscall on the
/// handle, with 1, the completion callback and this). Returns the attach
/// callee's answer. Straight-line code, no branches.
///
/// Original: 0x00BE8080 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00BE8080(obj: u32, target: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x68;
        const SLOT_B: u32 = 0x6c;
        const WEIGHT: u32 = 0x4080_0000;
        const HANDLE: u32 = 0x60;
        const MOVER: u32 = 0x78;
        const CALLBACK: u32 = 0x00BE4D70;
        const COMPUTE: u32 = 1;
        const ATTACH: u32 = 2;
        let mover = ((target + MOVER) as *const u32).read_unaligned();
        let a = ((obj + SLOT_A) as *const u32).read_unaligned();
        let b = ((obj + SLOT_B) as *const u32).read_unaligned();
        let h: u32 =
            lf_checker_rt::callee_thiscall!(COMPUTE, u32, mover, a, b, WEIGHT, 0xffff_ffff);
        ((obj + HANDLE) as *mut u32).write_unaligned(h);
        lf_checker_rt::callee_thiscall!(
            ATTACH, u32, h, 1, lf_checker_rt::relocated(CALLBACK), obj
        )
    }
});

