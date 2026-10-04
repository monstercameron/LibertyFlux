// original: 0x00b39570 task_mover_setup (proposed)

/// Set up a task mover for `obj` with the `blend` factor: record the fixed
/// mover kind and clear the mover serial, run the prepare callee, then
/// unless the `flags` word has the skip bit, run the adjust callee with the
/// `rate` word. Always run the settle callee, then build the mover
/// descriptor (the blend factor in its two float slots, zeros elsewhere) and
/// run the attach callee with `obj` twice, the descriptor, a zero word and
/// the flags word twice. Returns the attach callee's answer. Original:
/// 0x00b39570 (cdecl, four stack words: obj, rate, blend, flags).
lf_checker_rt::export!(cdecl, rw_00b39570(obj: u32, rate: u32, blend: u32, flags: u32) -> u32 {
    unsafe {
        const PREPARE: u32 = 1;
        const ADJUST: u32 = 2;
        const SETTLE: u32 = 3;
        const ATTACH: u32 = 4;
        const KIND_GLOBAL: u32 = 0x010459b4;
        const SERIAL_GLOBAL: u32 = 0x016624b0;
        const MOVER_KIND: u32 = 0x400;
        const SKIP_BIT: u32 = 2;
        lf_checker_rt::global::<u32>(KIND_GLOBAL).write(MOVER_KIND);
        lf_checker_rt::global::<u32>(SERIAL_GLOBAL).write(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(PREPARE, u32, obj, blend);
        if flags & SKIP_BIT == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(ADJUST, u32, obj, rate);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(SETTLE, u32,);
        lf_checker_rt::callee_cdecl!(
            ATTACH, u32, obj, obj, 0, blend, 0, blend, 0, 0, 0, flags, flags
        )
    }
});
