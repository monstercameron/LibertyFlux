// original: 0x00a8b1d0 pool_dispatch_virtual (proposed)

/// Dispatch an indexed call through the per-kind handler slot.
///
/// `this` is the pool object, `idx` the entry and `extra` an argument the
/// handler receives. The kind byte comes from the shared table at
/// `idx*24+0x17`; kind 0xFF returns 0 with no call. Otherwise the handler
/// is loaded from `this+kind*100+0x28` and invoked with
/// (`idx - this[kind*100+0x58]`, `extra`), returning its answer. The proof
/// windows the table to four entries and plants the stub at the three
/// live slots.
///
/// Original: 0x00A8B1D0 (thiscall, two stack words, register-indirect
/// callee).
lf_checker_rt::export!(thiscall, rw_00a8b1d0(this: u32, idx: u32, extra: u32) -> u32 {
    unsafe {
        const KIND_TABLE: u32 = 0x103e8d0;
        const KIND_STRIDE: u32 = 24;
        const KIND_OFF: u32 = 0x17;
        const KIND_ABSENT: u8 = 0xff;
        const SLOT_STRIDE: u32 = 0x64;
        const HANDLER_OFF: u32 = 0x28;
        const ADJUST_OFF: u32 = 0x58;
        let kind = (lf_checker_rt::global::<u8>(KIND_TABLE)
            .add(idx.wrapping_mul(KIND_STRIDE).wrapping_add(KIND_OFF) as usize))
        .read_unaligned();
        if kind == KIND_ABSENT {
            return 0;
        }
        let base = this
            .wrapping_add((kind as u32).wrapping_mul(SLOT_STRIDE));
        let adjust =
            ((base + ADJUST_OFF) as *const u32).read_unaligned();
        let target =
            ((base + HANDLER_OFF) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(idx.wrapping_sub(adjust), extra)
    }
});
