// original: 0x00AED220 kv_range_sort (proposed)

/// Sort a key/value range: heap pass, fill sweep, then pop-down loop.
///
/// `base` is the range start, `hi` the working end, `limit` the sweep end.
/// First the heap-pass callee runs over (`base`, `hi`, `extra`). Then every
/// entry from `hi` up to `limit` whose key is below the base key (unsigned)
/// is overwritten with the base pair, notifying the callee with (`base`,
/// 0, (`hi` - `base`) / 8, old key, old value, `ctx`). Finally the pop
/// callee runs as (`base`, cursor, `ctx`) while the cursor, stepping down
/// by 8 from `hi`, stays more than 8 above `base` (compared after masking
/// the low three bits). `ctx` is the same fifth stack word as `extra` (the
/// original reloads it from its argument slot after the heap pass). Returns
/// nothing meaningful.
///
/// Original: 0x00AED220 (cdecl, five stack words, three direct callees).
lf_checker_rt::export!(cdecl, rw_00aed220(base: u32, hi: u32, limit: u32, _a: u32, ctx: u32) -> () {
    unsafe {
        const ENTRY: u32 = 8;
        const HEAP_CALLEE: u32 = 1;
        const NOTIFY_CALLEE: u32 = 2;
        const POP_CALLEE: u32 = 3;
        lf_checker_rt::callee_cdecl!(HEAP_CALLEE, u32, base, hi, ctx);
        if hi < limit {
            let mut cur = hi;
            loop {
                let ekey = ((cur) as *const u32).read_unaligned();
                if ekey < ((base) as *const u32).read_unaligned() {
                    let eval = ((cur.wrapping_add(4)) as *const u32).read_unaligned();
                    let bkey = ((base) as *const u32).read_unaligned();
                    let bval = ((base.wrapping_add(4)) as *const u32).read_unaligned();
                    ((cur) as *mut u32).write_unaligned(bkey);
                    ((cur.wrapping_add(4)) as *mut u32).write_unaligned(bval);
                    let span = (hi.wrapping_sub(base) as i32 >> 3) as u32;
                    lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, base, 0, span, ekey, eval, ctx);
                }
                cur = cur.wrapping_add(ENTRY);
                if cur >= limit {
                    break;
                }
            }
        }
        let mut cursor = hi;
        while ((cursor.wrapping_sub(base) as i32) & !7) > 8 {
            lf_checker_rt::callee_cdecl!(POP_CALLEE, u32, base, cursor, ctx);
            cursor = cursor.wrapping_sub(ENTRY);
        }
    }
});
