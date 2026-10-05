// original: 0x00ab6850 stream_find_index (proposed)

/// Find the first slot at or after `start` whose id matches `*key`.
///
/// The object holds a dense id array at `+0` with its length at `+0x80`.
/// Scans slots `start..len` comparing each word against the key word;
/// returns the matching index, or -1 when `start` is already past the end
/// or no slot matches. Both bounds compare signed, as the original's
/// conditional jumps do. Note the argument order: the key pointer comes
/// first, the start index second.
///
/// Original: 0x00ab6850 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab6850(this: u32, key: u32, start: u32) -> u32 {
    unsafe {
        const LEN_OFF: u32 = 0x80;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let len = ((this + LEN_OFF) as *const u32).read_unaligned() as i32;
        let want = (key as *const u32).read_unaligned();
        let mut i = start as i32;
        while i < len {
            let got =
                ((this.wrapping_add((i as u32).wrapping_mul(4))) as *const u32).read_unaligned();
            if got == want {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
