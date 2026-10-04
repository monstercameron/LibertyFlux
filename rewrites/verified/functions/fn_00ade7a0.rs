// original: 0x00ade7a0 sort_dispatch (proposed)

/// Sort a range, splitting large ranges into an insertion-sorted head
/// and a recursively handled tail.
///
/// `first` and `last` bound a word array, `comp` is the comparator
/// passed through to the callees, the fourth argument is unread. When
/// the range spans more than 16 words, the insertion-sort callee
/// (id 1) sorts the first 16 words and the tail callee (id 2) handles
/// the rest; otherwise the insertion-sort callee sorts the whole
/// range. Both callees take (first, last, 0, comparator).
///
/// Edge cases: an empty range still calls the insertion-sort callee
/// once; the 16-word boundary itself takes the single-call path.
///
/// Original: cdecl, four stack words, two direct callees (ids 2 and 3,
/// cdecl, four arguments each; id 1 only supplies the comparator stub
/// address passed through to them). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade7a0(first: u32, last: u32, comp: u32, _unused: u32) -> u32 {
    const HEAD_WORDS: u32 = 16;
    const HEAD_BYTES: u32 = HEAD_WORDS * 4;
    let span = last.wrapping_sub(first) & 0xFFFF_FFFC;
    if span > HEAD_BYTES {
        let mid = first.wrapping_add(HEAD_BYTES);
        lf_checker_rt::callee_cdecl!(2, u32, first, mid, 0, comp);
        lf_checker_rt::callee_cdecl!(3, u32, mid, last, 0, comp);
    } else {
        lf_checker_rt::callee_cdecl!(2, u32, first, last, 0, comp);
    }
    0
});
