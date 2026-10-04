// original: 0x00899d80 audio_combsort_by_key
/// Sorts an audio record array by unsigned key with a comb sort.
///
/// Orders the eight-byte records at the table base by their key word
/// (offset 4) ascending, using gaps that shrink by a factor of 1.3 with the
/// classic 9/10-to-11 fixup, stopping on a clean pass at gap 1.
export!(thiscall, rw_00899d80(this: u32) -> () {
    unsafe {
        let count = core::ptr::read_unaligned(this.wrapping_add(0x3c) as *const u32);
        if count == 0 {
            return;
        }
        let base = core::ptr::read_unaligned(this.wrapping_add(0x40) as *const u32);
        let mut gap = count;
        loop {
            // Gap shrinks by 1.3: (gap * 10) / 13 via multiply-high magic.
            let scaled = gap.wrapping_mul(10);
            let mut g = (((scaled as u64 * 0x4EC4EC4Fu64) >> 32) >> 2) as u32;
            if g == 9 || g == 10 {
                g = 11;
            }
            if g < 1 {
                g = 1;
            }
            gap = g;
            let mut swapped = false;
            if count != gap {
                let mut i = 0u32;
                while i < count.wrapping_sub(gap) {
                    let a = base.wrapping_add(i.wrapping_mul(8));
                    let b = base.wrapping_add(i.wrapping_add(gap).wrapping_mul(8));
                    let ka = core::ptr::read_unaligned(a.wrapping_add(4) as *const u32);
                    let kb = core::ptr::read_unaligned(b.wrapping_add(4) as *const u32);
                    if ka > kb {
                        let la = core::ptr::read_unaligned(a as *const u32);
                        let lb = core::ptr::read_unaligned(b as *const u32);
                        core::ptr::write_unaligned(a as *mut u32, lb);
                        core::ptr::write_unaligned(a.wrapping_add(4) as *mut u32, kb);
                        core::ptr::write_unaligned(b as *mut u32, la);
                        core::ptr::write_unaligned(b.wrapping_add(4) as *mut u32, ka);
                        swapped = true;
                    }
                    i = i.wrapping_add(1);
                }
            }
            if gap == 1 && !swapped {
                break;
            }
        }
    }
});
