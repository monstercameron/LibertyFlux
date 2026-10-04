// original: 0x00abc230 sweep_sentinels_down

/// Sweep empty markers down to the tail of a word range by swapping.
///
/// Scans up for the next empty-slot marker while stepping a tail pointer
/// down, swapping the two whenever they have not crossed yet, until the scan
/// meets the tail. Returns the meeting pointer. The tag argument must equal
/// the empty marker: any other value spins the tail step forever, so only
/// the marker is exercised by the checker.
export!(cdecl, rs64_abc230(lo: u32, hi: u32, tag: u32) -> u32 {
    unsafe {
        const EMPTY_FILEVA: u32 = 0x0151_0A90;
        let empty = relocated(EMPTY_FILEVA);
        let mut a = lo;
        let mut b = hi;
        loop {
            while *(a as *const u32) != empty {
                a = a.wrapping_add(4);
            }
            loop {
                b = b.wrapping_sub(4);
                if tag == empty {
                    break;
                }
            }
            if a >= b {
                break;
            }
            let x = *(a as *const u32);
            let y = *(b as *const u32);
            *(a as *mut u32) = y;
            *(b as *mut u32) = x;
            a = a.wrapping_add(4);
        }
        a
    }
});
