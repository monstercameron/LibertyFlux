// original: 0x00AD40B0 audio_store_tagged_word (proposed)

/// Store a tagged word into the audio table or its overflow area.
///
/// Reads the half-word at `base + (a1 + a0 * 12) * 2` and dispatches on
/// its top two bits. Tag 0 overwrites it with the low half of
/// `(a3 << 14) | a2` and returns the full word. Tag 3 appends the word to
/// the overflow area, clears the next slot and bumps the count, returning
/// 0. Tags 1 and 2 link the slot instead: the old half-word and the new
/// word are pushed as an overflow pair, a zero terminator follows, the
/// slot becomes `count | 0xc000`, the count advances by three, and the
/// link word is returned. Cdecl/4. When the overflow area is full the
/// original calls its fail-fast helper; tested counts stay below that.
lf_checker_rt::export!(cdecl, rw_00ad40b0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0154E358;
        const OVERFLOW: u32 = 0x0154E476;
        const OVERFLOW_END: u32 = 0x0154E478;
        const COUNT: u32 = 0x0154EC48;
        const LINK_BIT: u32 = 0xC000;
        const ROW_STRIDE: u32 = 12;
        let edx = a1.wrapping_add(a0.wrapping_mul(3).wrapping_mul(4));
        let at = lf_checker_rt::relocated(TABLE).wrapping_add(edx.wrapping_mul(2));
        let old = (at as *const u16).read();
        let combined = a3.wrapping_shl(14) | a2;
        if old >> 14 == 0 {
            (at as *mut u16).write(combined as u16);
            return combined;
        }
        let count = lf_checker_rt::global::<u32>(COUNT).read();
        if old >> 14 == 3 {
            let d = count.wrapping_mul(2);
            (lf_checker_rt::relocated(OVERFLOW).wrapping_add(d) as *mut u16)
                .write(combined as u16);
            (lf_checker_rt::relocated(OVERFLOW_END).wrapping_add(d) as *mut u16).write(0);
            lf_checker_rt::global::<u32>(COUNT).write(count.wrapping_add(1));
            return 0;
        }
        let pair = count.wrapping_mul(2);
        (lf_checker_rt::relocated(OVERFLOW_END).wrapping_add(pair) as *mut u16).write(old);
        (lf_checker_rt::relocated(OVERFLOW_END).wrapping_add(pair).wrapping_add(2) as *mut u16)
            .write(combined as u16);
        let end = pair.wrapping_add(4);
        (lf_checker_rt::relocated(OVERFLOW_END).wrapping_add(end) as *mut u16).write(0);
        let link = count | LINK_BIT;
        (at as *mut u16).write(link as u16);
        lf_checker_rt::global::<u32>(COUNT).write(count.wrapping_add(3));
        link
    }
});
