// original: 0x00b34be0 sort_span_16b (proposed)

/// Sort the span of 16-byte elements from `first` to `limit` with the
/// leading-key ordering: run the prologue callee, then for each element from
/// `first` while below `limit`, when the pivot element's leading key float
/// is ordered-above the element's leading key (NaN never triggers),
/// overwrite the element with the pivot's 16 bytes and run the run callee
/// with the pivot, a zero flag, the span length `(first - pivot) / 16`
/// (signed), the overwritten element's old 16 bytes by value, and the
/// context word. Then, while the span from the pivot to the shrinking `first`
/// still exceeds one element, run the drain callee and shrink `first` by one
/// element. Returns the last callee answer in execution order. Original:
/// 0x00b34be0 (cdecl, five stack words: pivot, first, limit, unused,
/// context).
lf_checker_rt::export!(cdecl, rw_00b34be0(
    pivot: u32,
    first: u32,
    limit: u32,
    _unused: u32,
    ctx: u32,
) -> u32 {
    unsafe {
        const PROLOGUE: u32 = 1;
        const RUN: u32 = 2;
        const DRAIN: u32 = 3;
        const ELEM: u32 = 16;
        const N_COPY: usize = 4;
        let mut answer: u32 =
            lf_checker_rt::callee_cdecl!(PROLOGUE, u32, pivot, first, ctx);
        let pivot_key =
            f32::from_bits((pivot as *const u32).read_unaligned());
        let mut elem = first;
        while elem < limit {
            let elem_key =
                f32::from_bits((elem as *const u32).read_unaligned());
            if pivot_key > elem_key {
                let p = elem as *const u32;
                let old = [
                    p.read_unaligned(),
                    p.add(1).read_unaligned(),
                    p.add(2).read_unaligned(),
                    p.add(3).read_unaligned(),
                ];
                core::ptr::copy_nonoverlapping(
                    pivot as *const u32,
                    elem as *mut u32,
                    N_COPY,
                );
                let span =
                    (first.wrapping_sub(pivot) as i32 / ELEM as i32) as u32;
                answer = lf_checker_rt::callee_cdecl!(
                    RUN, u32, pivot, 0, span, old[0], old[1], old[2], old[3], ctx
                );
            }
            elem = elem.wrapping_add(ELEM);
        }
        let mut head = first;
        while ((head.wrapping_sub(pivot) & 0xffff_fff0) as i32) > ELEM as i32 {
            answer =
                lf_checker_rt::callee_cdecl!(DRAIN, u32, pivot, head, ctx);
            head = head.wrapping_sub(ELEM);
        }
        answer
    }
});
