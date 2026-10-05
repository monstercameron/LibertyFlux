// original: 0x00b33e30 peds_record_sweep_min (proposed)

/// Sweep every later 28-byte record against the first, demoting or evicting.
///
/// `begin` points at the first record and `end` one past the last; ranges
/// shorter than two records are untested (the original returns whatever the
/// incoming register held, which a Rust rewrite cannot read) and return 0
/// here. Each later record is compared by its float key (dword at record
/// +0x0c) with the first record's: a strictly larger key goes through callee
/// 1 as (first, record, record+28, scratch copy), after which the record is
/// copied over the first; anything else, unordered NaN included, goes through
/// callee 2 as (record, the seven record words, `w14`). The scratch copy is
/// the original's stack temporary byte for byte: one fill byte (the contract
/// sets the stack fill to 0) followed by its staged words. Returns the last
/// copied record's trailing word, or callee 2's answer.
///
/// Loads run in the original's order so fault behaviour matches access for
/// access.
///
/// Original: 0x00b33e30 (cdecl, four stack words, the third unread).
lf_checker_rt::export!(cdecl, rw_00b33e30(begin: u32, end: u32, _w10: u32, w14: u32) -> u32 {
    unsafe {
        const REC_LEN: u32 = 28;
        const KEY_OFF: u32 = 0x0c;
        const DEMOTE: u32 = 1;
        const EVICT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if begin == end || begin.wrapping_add(REC_LEN) == end {
            return 0;
        }
        let mut ret = 0u32;
        let mut scan = begin.wrapping_add(REC_LEN);
        loop {
            let base_lo = rd64(begin);
            let rec_lo = rd64(scan);
            let rec_w5 = rd32(scan + 0x14);
            let rec_mid = rd64(scan + 8);
            let rec_w4 = rd32(scan + 0x10);
            let rec_w6 = rd32(scan + 0x18);
            let base_mid = rd64(begin + 8);
            let _base_w4 = rd32(begin + 0x10);
            let _ = (base_lo, base_mid);
            let scan_key = f32::from_bits(rd32(scan + KEY_OFF));
            let base_key = f32::from_bits(rd32(begin + KEY_OFF));
            if scan_key > base_key {
                let mut scratch = [0u8; 28];
                scratch[1..5].copy_from_slice(&rec_w6.to_le_bytes());
                scratch[5..9].copy_from_slice(&rec_w5.to_le_bytes());
                scratch[9..17].copy_from_slice(&rec_lo.to_le_bytes());
                scratch[17..25].copy_from_slice(&rec_mid.to_le_bytes());
                scratch[25..28].copy_from_slice(&rec_lo.to_le_bytes()[..3]);
                let _ = lf_checker_rt::callee_cdecl!(DEMOTE, u32, begin, scan,
                    scan.wrapping_add(REC_LEN), scratch.as_ptr() as u32);
                wr64(begin, rec_lo);
                wr64(begin + 8, rec_mid);
                wr32(begin + 0x10, rec_w4);
                wr32(begin + 0x14, rec_w5);
                wr32(begin + 0x18, rec_w6);
                ret = rec_w6;
            } else {
                ret = lf_checker_rt::callee_cdecl!(EVICT, u32, scan, rd32(scan),
                    rd32(scan + 4), rd32(scan + 8), rd32(scan + 12), rd32(scan + 16),
                    rd32(scan + 20), rd32(scan + 24), w14);
            }
            scan = scan.wrapping_add(REC_LEN);
            if scan == end {
                break;
            }
        }
        ret
    }
});
