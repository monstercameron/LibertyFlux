// original: 0x0099d6c0 range_sort_dispatch
/// Sort dispatch over 16-byte records.
///
/// When the record span from `first` to `last` covers more than 0x100 bytes
/// (counted with the low nibble masked off, compared as a signed value), the
/// head chunk of 0x100 bytes is sorted through callee 1 and the remainder
/// through callee 2; otherwise the whole span goes through callee 1. The
/// auxiliary value is forwarded untouched.
export!(cdecl, rw_0099d6c0(first: u32, last: u32, aux: u32) -> () {
    unsafe {
        const HEAD: u32 = 0x100;
        let span = last.wrapping_sub(first) & 0xFFFF_FFF0;
        if (span as i32) > HEAD as i32 {
            let mid = first.wrapping_add(HEAD);
            callee_cdecl!(1, u32, first, mid, 0, aux);
            callee_cdecl!(2, u32, mid, last, 0, aux);
        } else {
            callee_cdecl!(1, u32, first, last, 0, aux);
        }
    }
});
