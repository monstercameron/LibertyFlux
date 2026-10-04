// original: 0x0099dab0 insert_each_record
/// Insert every record in `[begin, end)` via the sorted-insert helper.
///
/// Thin loop over the cdecl/6 helper (stubbed by the checker): each 16-byte
/// record is passed by value together with its own address and the forwarded
/// flag. The third word is padding the original never reads. Empty ranges
/// return without calling.
export!(cdecl, rw_0099dab0(begin: u32, end: u32, _w3: u32, flag: u32) -> () {
    unsafe {
        if begin == end {
            return;
        }
        let mut cur = begin;
        loop {
            let s = cur as *const u32;
            callee_cdecl!(1, u32, cur, *s, *s.add(1), *s.add(2), *s.add(3), flag);
            cur = cur.wrapping_add(16);
            if cur == end {
                break;
            }
        }
    }
});
