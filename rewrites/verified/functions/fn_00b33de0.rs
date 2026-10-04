// original: 0x00b33de0 dispatch_range_split_256 (proposed)

/// Process an address range, splitting off a 256-byte head when it is long.
///
/// `first` and `last` bound the range, `extra` is forwarded untouched. The
/// length `(last - first)` rounded down to 16 bytes decides, compared as a
/// signed 32-bit value: when it exceeds 256 the head `[first, first + 256)`
/// goes through the main callee and the tail `[first + 256, last)` through
/// the tail callee; otherwise (including a backwards range, which compares
/// negative) the whole range goes through the main callee. Both callees also
/// take a zero word and `extra`. Returns whatever the last callee call
/// returned.
///
/// Original: 0x00b33de0 (cdecl, three stack words; two cdecl callees of four
/// words each).
lf_checker_rt::export!(cdecl, rw_00b33de0(first: u32, last: u32, extra: u32) -> u32 {
    const SPLIT: u32 = 0x100;
    const MAIN: u32 = 1;
    const TAIL: u32 = 2;
    let len = last.wrapping_sub(first) & !0xF;
    if len as i32 > SPLIT as i32 {
        let mid = first.wrapping_add(SPLIT);
        lf_checker_rt::callee_cdecl!(MAIN, u32, first, mid, 0u32, extra);
        lf_checker_rt::callee_cdecl!(TAIL, u32, mid, last, 0u32, extra)
    } else {
        lf_checker_rt::callee_cdecl!(MAIN, u32, first, last, 0u32, extra)
    }
});
