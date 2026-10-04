// original: 0x00BE7FC0 anim_setup_checked_a (proposed)
/// Set up a blend slot when the pilot check accepts, else mark it failed.
///
/// `obj` points to the task object, `target` to the subject and `arg`
/// selects the variant. The check callee (cdecl, kind 0x0b, arg) runs
/// first: a zero answer stores 1 in the failed byte at `+0x24` and
/// returns 0 with no further calls. Otherwise the compute callee runs
/// (thiscall on the mover at `[target+0x78]`, with kind 0x0b, arg, the
/// 16.0 weight and -1), its handle is stored at `+0x18`, and the attach
/// callee links this object (thiscall on the handle, with 1, the
/// completion callback and this); the failed byte is cleared. Returns the
/// attach callee's answer, or 0 on the failed path.
///
/// Original: 0x00BE7FC0 (thiscall, two stack words). Twin of 0x00BE8020
/// with a different weight, slots and callback.
lf_checker_rt::export!(thiscall, rw_00BE7FC0(obj: u32, target: u32, arg: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x0b;
        const WEIGHT: u32 = 0x4180_0000;
        const HANDLE: u32 = 0x18;
        const FAILED: u32 = 0x24;
        const MOVER: u32 = 0x78;
        const CALLBACK: u32 = 0x00BE4D40;
        const CHECK: u32 = 1;
        const COMPUTE: u32 = 2;
        const ATTACH: u32 = 3;
        let ok: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, KIND, arg);
        if ok == 0 {
            ((obj + FAILED) as *mut u8).write(1);
            return 0;
        }
        let mover = ((target + MOVER) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(
            COMPUTE, u32, mover, KIND, arg, WEIGHT, 0xffff_ffff
        );
        ((obj + HANDLE) as *mut u32).write_unaligned(h);
        let out: u32 = lf_checker_rt::callee_thiscall!(
            ATTACH, u32, h, 1, lf_checker_rt::relocated(CALLBACK), obj
        );
        ((obj + FAILED) as *mut u8).write(0);
        out
    }
});

