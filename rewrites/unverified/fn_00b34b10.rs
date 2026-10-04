// original: 0x00b34b10 sort_span_key0 (proposed)

/// Sort the span of 28-byte elements from `first` to `limit` with the
/// leading-key ordering: run the prologue callee, then for each element from
/// `first` while below `limit`, when the pivot element's leading key float
/// is ordered-above the element's leading key (NaN never triggers), run the
/// sift callee with the pivot, `first`, the element, the element's 28 bytes
/// by value, a zero flag and the context word. Finish with the epilogue
/// callee and return its answer. Original: 0x00b34b10 (cdecl, five stack
/// words: pivot, first, limit, unused, context).
lf_checker_rt::export!(cdecl, rw_00b34b10(
    pivot: u32,
    first: u32,
    limit: u32,
    _unused: u32,
    ctx: u32,
) -> u32 {
    unsafe {
        const PROLOGUE: u32 = 1;
        const SIFT: u32 = 2;
        const EPILOGUE: u32 = 3;
        const ELEM: u32 = 28;
        let _: u32 = lf_checker_rt::callee_cdecl!(PROLOGUE, u32, pivot, first, ctx);
        let pivot_key =
            f32::from_bits((pivot as *const u32).read_unaligned());
        let mut elem = first;
        while elem < limit {
            let elem_key =
                f32::from_bits((elem as *const u32).read_unaligned());
            if pivot_key > elem_key {
                let p = elem as *const u32;
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    SIFT,
                    u32,
                    pivot,
                    first,
                    elem,
                    p.read_unaligned(),
                    p.add(1).read_unaligned(),
                    p.add(2).read_unaligned(),
                    p.add(3).read_unaligned(),
                    p.add(4).read_unaligned(),
                    p.add(5).read_unaligned(),
                    p.add(6).read_unaligned(),
                    ctx,
                    0
                );
            }
            elem = elem.wrapping_add(ELEM);
        }
        lf_checker_rt::callee_cdecl!(EPILOGUE, u32, pivot, first, ctx)
    }
});
