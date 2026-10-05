// original: 0x00a95a30 stream_dispatch_entry_kind (proposed)

/// Dispatch a streaming entry to the handler for its kind byte.
///
/// The entry is `table[idx*3*8]` where `table` is the pointer at `this+0x00`.
/// A kind byte of 0xff at entry `+0x16` means no handler: the table pointer
/// is returned. Otherwise the byte at `+0x17` times 100 indexes a dword in
/// a global handler table, the entry key is fetched (callee 1, thiscall/0 on
/// the entry), and the tabled handler is invoked (cdecl/1) with that key.
///
/// Returns the handler's answer, or the table pointer when no handler runs.
/// Thiscall: object in ecx, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a95a30(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const KIND_OFF: u32 = 0x16;
        const SUBKIND_OFF: u32 = 0x17;
        const NO_HANDLER: u8 = 0xff;
        const HANDLER_TABLE: u32 = 0x01305368;
        const TABLE_SCALE: u32 = 100;
        const KEY_CALLEE: u32 = 1;
        let base = ((this + TABLE) as *const u32).read_unaligned();
        let elem = base + idx.wrapping_mul(3).wrapping_mul(8);
        if ((elem + KIND_OFF) as *const u8).read() == NO_HANDLER {
            return base;
        }
        let key = lf_checker_rt::callee_thiscall!(KEY_CALLEE, u32, elem);
        let slot = (((elem + SUBKIND_OFF) as *const u8).read() as u32)
            .wrapping_mul(TABLE_SCALE);
        let target =
            ((lf_checker_rt::relocated(HANDLER_TABLE) + slot) as *const u32).read_unaligned();
        let handler: extern "cdecl" fn(u32) -> u32 = core::mem::transmute(target as usize);
        handler(key)
    }
});
