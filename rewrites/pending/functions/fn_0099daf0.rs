// original: 0x0099daf0 sorted_insert_record
/// Insert one 16-byte record into the sorted run ending at `pos`.
///
/// Records carry their key at +12. While the key of the record below `pos`
/// is above the new key, that record shifts up one slot; the new record is
/// then stored. Ends with the security-cookie check (stubbed by the checker)
/// whose answer is returned. The sixth word is accepted but never read by
/// the original.
export!(cdecl, rw_0099daf0(pos: u32, v0: u32, v1: u32, v2: u32, v3: u32, _f: u32) -> u32 {
    unsafe {
        let mut at = pos;
        loop {
            let below = *((at.wrapping_sub(4)) as *const u32);
            if v3 >= below {
                break;
            }
            let d = at as *mut u32;
            let s = at.wrapping_sub(16) as *const u32;
            *d = *s;
            *d.add(1) = *s.add(1);
            *d.add(2) = *s.add(2);
            *d.add(3) = *s.add(3);
            at = at.wrapping_sub(16);
        }
        let d = at as *mut u32;
        *d = v0;
        *d.add(1) = v1;
        *d.add(2) = v2;
        *d.add(3) = v3;
        callee_stdcall!(1, u32)
    }
});
