// original: 0x0093e070 ptr_sort (proposed)

/// Sort the pointer range with a bounded run check and tail passes.
///
/// Calls the build callee with (`base`, `end`, `extra`), then scans slots
/// [`end`, `run_end`) unsigned: a slot whose key is below the first
/// element's key is heap-adjusted through the heap callee with (`base`,
/// 0, index, slot value, `extra`) and overwritten with the first element.
/// Then, while the range span above 4 words lasts, calls the tail callee
/// with (`base`, edge, temp) as edge steps down 4 per call, where temp is
/// `end` with its low byte replaced by `extra`'s low byte (the original
/// keeps that byte in its dead `end` home slot; the rewrite computes the
/// same value directly). Returns the final span masked to words. The
/// fourth word is never read.
///
/// Original: 0x0093e070 (cdecl, five stack words; three direct callees).
lf_checker_rt::export!(cdecl, rw_0093e070(
    base: u32, end: u32, run_end: u32, _pad: u32, extra: u32,
) -> u32 {
    const BUILD: u32 = 1;
    const HEAP_CALLEE: u32 = 2;
    const TAIL: u32 = 3;
    unsafe {
        lf_checker_rt::callee_cdecl!(BUILD, u32, base, end, extra);
        // Note: the index below stays (end - base) / 4 for every slot; the
        // original derives it from the range end, not from the cursor.
        let idx = ((end.wrapping_sub(base)) as i32 >> 2) as u32;
        let mut cur = end;
        if end < run_end {
            loop {
                let slot_val = (cur as *const u32).read_unaligned();
                let first = (base as *const u32).read_unaligned();
                if ((slot_val as *const u32).read_unaligned())
                    < ((first as *const u32).read_unaligned())
                {
                    (cur as *mut u32).write_unaligned(first);
                    lf_checker_rt::callee_cdecl!(HEAP_CALLEE, u32, base, 0u32, idx, slot_val, extra);
                }
                cur = cur.wrapping_add(4);
                if !(cur < run_end) {
                    break;
                }
            }
        }
        let temp = (end & 0xFFFF_FF00) | (extra & 0xFF);
        let mut span = end.wrapping_sub(base);
        let mut acc = span & 0xFFFF_FFFC;
        if (acc as i32) <= 4 {
            return acc;
        }
        let mut edge = end;
        loop {
            lf_checker_rt::callee_cdecl!(TAIL, u32, base, edge, temp);
            span = span.wrapping_sub(4);
            acc = span & 0xFFFF_FFFC;
            edge = edge.wrapping_sub(4);
            if (acc as i32) > 4 {
                continue;
            }
            break;
        }
        acc
    }
});
