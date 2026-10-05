// original: 0x00A4D800 vehicle_route_b (proposed)

/// Routes through two virtual calls and forwards everything to a worker.
///
/// Same shape as the neighbouring route function: resolves `info` from the
/// global pointer table at `TABLE` by the signed word at `this + MODEL`
/// (0x2E), calls the source through virtual slot `SRC_SLOT` (0xA0) with
/// `this` in `ecx` (null answer: `path = [this + ALT]`, 0x100; otherwise the
/// slot is called again and the answer is chained through its virtual slot
/// `CHAIN_SLOT`, 0xE0), then calls this function's own worker (cdecl,
/// `(path, arg0, arg1, arg2, info)`) and returns its answer. Virtual targets
/// load through the objects exactly like the original.
///
/// Original: 0x00A4D800 (thiscall, three stack words), three callees.
lf_checker_rt::export!(thiscall, rw_00A4D800(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
) -> u32 {
    unsafe {
        const MODEL: u32 = 0x2E;
        const TABLE: u32 = 0x01295CD8;
        const INFO: u32 = 0xCC;
        const ALT: u32 = 0x100;
        const SRC_SLOT: u32 = 0xA0;
        const CHAIN_SLOT: u32 = 0xE0;
        const WORK_CALLEE: u32 = 3;
        let model = ((this + MODEL) as *const i16).read_unaligned() as i32;
        let entry = lf_checker_rt::relocated(TABLE)
            .wrapping_add((model * 4) as u32);
        let info = ((entry + INFO) as *const u32).read_unaligned();
        let vtable = (this as *const u32).read_unaligned();
        let src: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + SRC_SLOT) as *const u32).read_unaligned() as usize,
        );
        let first = src(this);
        let path = if first == 0 {
            ((this + ALT) as *const u32).read_unaligned()
        } else {
            let second = src(this);
            let vtable2 = (second as *const u32).read_unaligned();
            let chain: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(
                    ((vtable2 + CHAIN_SLOT) as *const u32).read_unaligned()
                        as usize,
                );
            chain(second)
        };
        lf_checker_rt::callee_cdecl!(WORK_CALLEE, u32, path, a0, a1, a2, info)
    }
});
