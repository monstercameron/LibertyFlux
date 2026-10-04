// original: 0x00b33d00 dispatch_records_split_16 (proposed)

/// Process an array of 28-byte records, splitting off a 16-record head.
///
/// `first` and `last` bound the array, `extra` is forwarded untouched. The
/// signed record count `(last - first) / 28` decides: when it exceeds 16 the
/// head `[first, first + 448)` goes through the main callee and the tail
/// `[first + 448, last)` through the tail callee; otherwise the whole array
/// goes through the main callee. Both callees also take a zero word and
/// `extra`. Returns whatever the last callee call returned.
///
/// Original: 0x00b33d00 (cdecl, three stack words; two cdecl callees of four
/// words each; the count uses the compiler's magic-number signed division
/// by 28).
lf_checker_rt::export!(cdecl, rw_00b33d00(first: u32, last: u32, extra: u32) -> u32 {
    const REC: i32 = 28;
    const SPLIT_COUNT: i32 = 16;
    const MAIN: u32 = 1;
    const TAIL: u32 = 2;
    let count = last.wrapping_sub(first) as i32 / REC;
    if count > SPLIT_COUNT {
        let mid = first.wrapping_add(SPLIT_COUNT as u32 * REC as u32);
        lf_checker_rt::callee_cdecl!(MAIN, u32, first, mid, 0u32, extra);
        lf_checker_rt::callee_cdecl!(TAIL, u32, mid, last, 0u32, extra)
    } else {
        lf_checker_rt::callee_cdecl!(MAIN, u32, first, last, 0u32, extra)
    }
});
