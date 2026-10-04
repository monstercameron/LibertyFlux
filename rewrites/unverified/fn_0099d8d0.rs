// original: 0x0099d8d0 heapify_16
/// Heapify pass over 16-byte records.
///
/// With `count = (hi - lo) >> 4` (signed), returns immediately when fewer
/// than two records are present. Otherwise it sifts the record at index
/// `(count - 2) / 2` through callee 1, then walks down to index 0, copying
/// each 16-byte record to a stack slot and invoking callee 1 with the base,
/// the index, the count, the record words and the auxiliary value.
export!(cdecl, rw_0099d8d0(lo: u32, hi: u32, aux: u32) -> () {
    unsafe {
        let count = (hi.wrapping_sub(lo) as i32) >> 4;
        if count < 2 {
            return;
        }
        let mut i = (count - 2) >> 1;
        let mut rec = lo.wrapping_add((i << 4) as u32);
        loop {
            let s = rec as *const u32;
            callee_cdecl!(1, u32, lo, i as u32, count as u32, *s, *s.add(1), *s.add(2), *s.add(3), aux);
            if i == 0 {
                break;
            }
            i -= 1;
            rec = rec.wrapping_sub(0x10);
        }
    }
});
