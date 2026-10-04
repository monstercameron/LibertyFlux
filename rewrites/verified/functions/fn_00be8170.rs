// original: 0x00BE8170 task_mover_init (proposed)
/// Initialise a mover task from the subject and the global tick.
///
/// `obj` points to the task object and `target` to the subject. The
/// subject is prepared first (thiscall on the target with 1, result
/// ignored); then the link at `+0x14` is copied to `+0x20`, the global
/// tick is stored at `+0x1c`, the ready byte at `+0x24` is set, the
/// compute callee runs on the mover at `[target+0x78]` with (0, 0, the
/// 4.0 weight, -1), and its handle is stored at `+0x18`. Returns the
/// compute callee's answer.
///
/// Original: 0x00BE8170 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00BE8170(obj: u32, target: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x14;
        const HANDLE: u32 = 0x18;
        const STAMP: u32 = 0x1c;
        const LINK_COPY: u32 = 0x20;
        const READY: u32 = 0x24;
        const MOVER: u32 = 0x78;
        const WEIGHT: u32 = 0x4080_0000;
        const TICK_GLOBAL: u32 = 0x011735B4;
        const PREPARE: u32 = 1;
        const COMPUTE: u32 = 2;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, target, 1);
        let link = ((obj + LINK) as *const u32).read_unaligned();
        let tick = (lf_checker_rt::global::<u32>(TICK_GLOBAL)).read_unaligned();
        ((obj + STAMP) as *mut u32).write_unaligned(tick);
        ((obj + LINK_COPY) as *mut u32).write_unaligned(link);
        ((obj + READY) as *mut u8).write(1);
        let mover = ((target + MOVER) as *const u32).read_unaligned();
        let h: u32 =
            lf_checker_rt::callee_thiscall!(COMPUTE, u32, mover, 0, 0, WEIGHT, 0xffff_ffff);
        ((obj + HANDLE) as *mut u32).write_unaligned(h);
        h
    }
});

