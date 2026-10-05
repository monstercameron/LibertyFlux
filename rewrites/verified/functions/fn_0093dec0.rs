// original: 0x0093DEC0 stream_pair_route (proposed)

/// Route a pointer range to the range workers, splitting big ones.
///
/// Measures `(end - start) & !3`. Ranges above 64 bytes go in two
/// pieces: the head worker takes `[start, start+64)` and the tail
/// worker `[start+64, end)`, both with a zero middle argument and the
/// context. Smaller ranges go whole to the head worker. Answers the
/// last worker's answer.
lf_checker_rt::export!(cdecl, rw_0093dec0(start: u32, end: u32, ctx: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 1;
        const TAIL: u32 = 2;
        const SPLIT: u32 = 0x40;
        let len = end.wrapping_sub(start) & !3;
        if len > SPLIT {
            let mid = start.wrapping_add(SPLIT);
            let _: u32 = lf_checker_rt::callee_cdecl!(HEAD, u32, start, mid, 0, ctx);
            lf_checker_rt::callee_cdecl!(TAIL, u32, mid, end, 0, ctx)
        } else {
            lf_checker_rt::callee_cdecl!(HEAD, u32, start, end, 0, ctx)
        }
    }
});
