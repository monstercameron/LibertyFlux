// original: 0x00d2fa80 task_route_entry_pair_fetch (proposed)

/// Fetch two consecutive 16-byte route entries selected by a biased index.
///
/// `this` points to the task object: dword at `+0x70` is a pointer to the
/// route table (entry count as i32 at `[0]`, 16-byte entries after it), the
/// signed byte at `+0x98` is an index bias, and the dword at `+0xd8` holds
/// state flags. `out_first`/`out_second` receive one entry each (16 bytes);
/// `adjust` is added to the bias to form the index. The first stack argument
/// is never read.
///
/// The fetch runs only when flag `0x200` is set, flag `0x400` is clear, and
/// the table pointer is non-null; otherwise the outputs are untouched and the
/// result is 0. Let `i = bias + adjust` and `n = count`. When `0 < i < n` the
/// entries at `i` and `i + 1` are copied out. Otherwise the fallback copies
/// entries `i + 1` and `i + 2`, which requires `i + 1 < n` (the second entry
/// is not bounds-checked); if that fails the result is 0 and the outputs are
/// untouched. Success returns 1. All comparisons are signed; address
/// arithmetic wraps mod 2^32.
///
/// Original: 0x00d2fa80 (thiscall: object in ECX, four stack words, callee
/// pops 0x10, boolean result in AL).
lf_checker_rt::export!(thiscall, rw_00d2fa80(this: u32, _unused: u32, out_first: u32, out_second: u32, adjust: u32) -> u8 {
    unsafe {
        const TABLE_PTR: u32 = 0x70;
        const INDEX_BIAS: u32 = 0x98;
        const STATE_FLAGS: u32 = 0xd8;
        const FLAG_READY: u32 = 0x200;
        const FLAG_BUSY: u32 = 0x400;
        const ENTRY_BYTES: i32 = 16;

        let flags = ((this + STATE_FLAGS) as *const u32).read_unaligned();
        if flags & FLAG_READY == 0 {
            return 0;
        }
        let table = ((this + TABLE_PTR) as *const u32).read_unaligned();
        if table == 0 {
            return 0;
        }
        if flags & FLAG_BUSY != 0 {
            return 0;
        }
        let bias = ((this + INDEX_BIAS) as *const i8).read() as i32;
        let index = bias.wrapping_add(adjust as i32);
        let count = (table as *const i32).read_unaligned();
        // Entry pair base: the index itself on the fast path, one past it on
        // the fallback path (whose lower entry only is bounds-checked).
        let base = if index > 0 && index < count {
            index
        } else {
            let next = index.wrapping_add(1);
            if next >= count {
                return 0;
            }
            next
        };
        let first = table.wrapping_add(base.wrapping_mul(ENTRY_BYTES) as u32);
        let second = first.wrapping_add(ENTRY_BYTES as u32);
        for word in 0..4u32 {
            let v = ((first + word * 4) as *const u32).read_unaligned();
            ((out_first + word * 4) as *mut u32).write_unaligned(v);
        }
        for word in 0..4u32 {
            let v = ((second + word * 4) as *const u32).read_unaligned();
            ((out_second + word * 4) as *mut u32).write_unaligned(v);
        }
        1
    }
});
