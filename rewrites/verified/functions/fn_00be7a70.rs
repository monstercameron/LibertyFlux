// original: 0x00be7a70 pool_slot_release (proposed)

/// Release a pool slot's reference and evict it when the count hits zero.
///
/// Takes a slot index. Loads the pool manager from the dword at file address
/// `0x16DD5D0`, whose words at `+0x00`/`+0x04`/`+0x0c` are the slot table, a
/// flag-byte array and the slot stride. Returns quietly when bit 0x80 of
/// flag byte `index` is set, or when the slot address
/// (`table + index * stride`) is null (a guard this contract never takes:
/// the table base is always a mapped pointer). Otherwise decrements the
/// reference count at slot `+0x04` and returns while it stays positive;
/// asks callee 1 (cdecl, two words: index, then the dword at file address
/// `0x10496E8`) whether the slot survives and returns when it answers
/// non-zero, else evicts through callee 2 (cdecl, one word: the index). No
/// meaningful return value (`ret: none`).
///
/// Original: cdecl, one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00be7a70(index: u32) -> u32 {
    unsafe {
        const MGR_GLOBAL: u32 = 0x16DD5D0;
        const AUX_GLOBAL: u32 = 0x10496E8;
        const OFF_TABLE: u32 = 0x00;
        const OFF_FLAGS: u32 = 0x04;
        const OFF_STRIDE: u32 = 0x0c;
        const OFF_REFCOUNT: u32 = 0x04;
        const SKIP_BIT: u8 = 0x80;
        const SURVIVES: u32 = 1;
        const EVICT: u32 = 2;

        let mgr = lf_checker_rt::global::<u32>(MGR_GLOBAL).read_unaligned();
        let flags = ((mgr + OFF_FLAGS) as *const u32).read_unaligned();
        if ((index.wrapping_add(flags)) as *const u8).read() & SKIP_BIT != 0 {
            return 0;
        }
        let stride = ((mgr + OFF_STRIDE) as *const u32).read_unaligned();
        let table = ((mgr + OFF_TABLE) as *const u32).read_unaligned();
        let slot = stride.wrapping_mul(index).wrapping_add(table);
        if slot == 0 {
            return 0;
        }
        let rc = ((slot + OFF_REFCOUNT) as *const u32).read_unaligned();
        let rc = rc.wrapping_sub(1);
        ((slot + OFF_REFCOUNT) as *mut u32).write_unaligned(rc);
        if (rc as i32) > 0 {
            return 0;
        }
        let aux = lf_checker_rt::global::<u32>(AUX_GLOBAL).read_unaligned();
        let ok: u32 = lf_checker_rt::callee_cdecl!(SURVIVES, u32, index, aux);
        if ok as u8 != 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(EVICT, u32, index);
        0
    }
});
