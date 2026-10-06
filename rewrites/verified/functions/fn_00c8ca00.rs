// original: 0x00c8ca00 audio_flag_resolve (proposed)
///
/// Rewrites the low flag bits of the double-word at `slot` when it holds a
/// pending pattern: only words whose low three bits are 4 (binary 100) and
/// whose bits 3..4 are both set (0x18) are touched. Of those, selector 0
/// (bits 5..6) clears bit 9 and sets bits 7..8 ((v & ~0x200) | 0x180);
/// selector 1 clears bits 7..8 and sets bit 9 ((v & ~0x180) | 0x200);
/// selectors 2 and 3 leave the word alone. Cdecl, one pointer argument,
/// no meaningful return value.

lf_checker_rt::export!(cdecl, rw_00c8ca00(slot: u32) -> u32 {
    unsafe {

        let p = slot as *mut u32;
        let v = p.read_unaligned();
        if v & 7 == 4 && v & 0x18 == 0x18 {
            let sel = (v >> 5) & 3;
            if sel == 0 {
                p.write_unaligned((v & 0xffff_fdff) | 0x180);
            } else if sel == 1 {
                p.write_unaligned((v & 0xffff_fe7f) | 0x200);
            }
        }
        0
    }
});
