// original: 0x00c878c0 audio_entry_tag (proposed)
///
/// Tags an entry with `tag`. `obj` points at a record whose byte at +4
/// selects the kind; only kinds 0x0a, 6 and 4 continue (anything else
/// returns 1 with no further effect). Callee 1 (thiscall) must accept
/// `obj` (non-zero low byte), then callee 2 (cdecl) derives the key from
/// `key`, callee 3 (the entry search, thiscall) looks it up, and when
/// that misses, callee 4 (the acquire above, thiscall, called with the
/// key and a zero second word) opens it instead. The found entry's
/// half-word at +0x56 is set to the low half of `tag`. Always returns 1
/// in al. Thiscall, three stack arguments (key, obj, tag).

lf_checker_rt::export!(thiscall, rw_00c878c0(this: u32, key: u32, obj: u32, tag: u32) -> u8 {
    unsafe {

        const TAG_OFF: u32 = 0x56;
        let kind = ((obj.wrapping_add(4)) as *const u8).read();
        if kind != 0x0a && kind != 6 && kind != 4 {
            return 1;
        }
        let ok = lf_checker_rt::callee_thiscall!(1, u32, this, obj);
        if ok & 0xff == 0 {
            return 1;
        }
        let derived = lf_checker_rt::callee_cdecl!(2, u32, key);
        let mut entry = lf_checker_rt::callee_thiscall!(3, u32, this, derived);
        if entry == 0 {
            entry = lf_checker_rt::callee_thiscall!(4, u32, this, derived, 0);
            if entry == 0 {
                return 1;
            }
        }
        ((entry.wrapping_add(TAG_OFF)) as *mut u16).write_unaligned(tag as u16);
        1
    }
});
