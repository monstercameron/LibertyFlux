// original: 0x00a95290 stream_unlink_slot_pair (proposed)

/// Unlink a streaming slot from the global slot table and mark it empty.
///
/// `this+0x10` and `this+0x12` hold two linked slot indexes (16-bit each).
/// The global slot table pointer is read from its global; the entry indexed
/// by the first index (`table + idx0*3*8`) receives the second index at its
/// `+0x12` halfword, and the entry indexed by the second index receives the
/// first index at its `+0x10` halfword, stitching the neighbours together.
/// The slot's own index word at `+0x10` is then set to all-ones (empty).
///
/// Returns the global table pointer (the value left in eax). Thiscall:
/// object in ecx, no stack words.
lf_checker_rt::export!(thiscall, rw_00a95290(this: u32) -> u32 {
    unsafe {
        const IDX0: u32 = 0x10;
        const IDX1: u32 = 0x12;
        const TABLE_GLOBAL: u32 = 0x012fb3a8;
        const STRIDE_MUL: u32 = 3;
        const ENTRY_STRIDE: u32 = 8;
        const EMPTY: u32 = 0xffffffff;
        let idx0 = ((this + IDX0) as *const u16).read_unaligned() as u32;
        let idx1 = ((this + IDX1) as *const u16).read_unaligned() as u32;
        let table = (lf_checker_rt::global::<u32>(TABLE_GLOBAL)).read_unaligned();
        ((table + idx0 * STRIDE_MUL * ENTRY_STRIDE + IDX1) as *mut u16)
            .write_unaligned(idx1 as u16);
        let table2 = (lf_checker_rt::global::<u32>(TABLE_GLOBAL)).read_unaligned();
        ((table2 + idx1 * STRIDE_MUL * ENTRY_STRIDE + IDX0) as *mut u16)
            .write_unaligned(idx0 as u16);
        ((this + IDX0) as *mut u32).write_unaligned(EMPTY);
        table2
    }
});
