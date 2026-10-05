// original: 0x00A4D5F0 vehicle_route_a (proposed)

/// Routes through two virtual calls and forwards everything to a worker.
///
/// Resolves `info = [TABLE_ENTRY(model) + INFO]` (0xCC) where the table entry
/// comes from the global pointer table at `TABLE` indexed by the signed word
/// at `this + MODEL` (0x2E). Calls the source through virtual slot `SRC_SLOT`
/// (0xA0) with `this` in `ecx`; on a null answer uses `path = [this + ALT]`
/// (0x100), otherwise calls the same slot again and chains through virtual
/// slot `CHAIN_SLOT` (0xE0) of the returned object (`path` = that answer).
/// Finally calls the worker (cdecl, `(path, arg0, arg1, arg2, info)`) and
/// returns its answer. Virtual targets load through the objects exactly like
/// the original.
///
/// Original: 0x00A4D5F0 (thiscall, three stack words), three callees.
lf_checker_rt::export!(thiscall, rw_00A4D5F0(
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
