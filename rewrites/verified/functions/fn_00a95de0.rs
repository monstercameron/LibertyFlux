// original: 0x00a95de0 filemem_init_slot_table

/// Initialise the slot table globals, then notify each live slot in turn.
///
/// Writes zero to the table count at `0x01305318`, -1 to the nine tag
/// words after it, and (at the end) zero to the epoch at `0x01305340`,
/// while shifting the flags at `0x012fb3b4` right by 11. When the table's
/// first flag byte is set, walks the 0xa0-byte records from `0x012fb3c8`
/// while the record pointer stays below `0x01305328` (both addresses are
/// below 2^31, so the original's signed bound matches an unsigned one)
/// and the record's flag byte stays set; each record whose byte at `+0x8c`
/// is set is reported to the notify callee with its address and index,
/// then announced to the broadcast callee.
///
/// Original: 0x00A95DE0 (cdecl, no arguments; two direct callees).
lf_checker_rt::export!(cdecl, rw_00a95de0() -> u32 {
    unsafe {
        /// Table count, tag words, record flags/bound, epoch (file VAs).
        const COUNT: u32 = 0x01305318;
        const TAGS: u32 = 0x0130531c;
        const NTAGS: u32 = 9;
        const RECORDS: u32 = 0x012fb3c8;
        const BOUND: u32 = 0x01305328;
        const STRIDE: u32 = 0xa0;
        const LIVE_OFF: u32 = 0x8c;
        const FLAGS: u32 = 0x012fb3b4;
        const FLAG_SHR: u32 = 11;
        const EPOCH: u32 = 0x01305340;
        const NOTIFY: u32 = 1;
        const BROADCAST: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(lf_checker_rt::relocated(COUNT), 0);
        for k in 0..NTAGS {
            wr32(lf_checker_rt::relocated(TAGS).wrapping_add(k.wrapping_mul(4)), 0xffffffff);
        }
        if ((lf_checker_rt::relocated(RECORDS)) as *const u8).read() != 0 {
            let bound = lf_checker_rt::relocated(BOUND);
            let mut rec = lf_checker_rt::relocated(RECORDS);
            let mut idx: u32 = 0;
            while (rec as i32) < (bound as i32) {
                if ((rec.wrapping_add(LIVE_OFF)) as *const u8).read() != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(NOTIFY, u32, rec, idx);
                    let _: u32 = lf_checker_rt::callee_cdecl!(BROADCAST, u32,);
                }
                rec = rec.wrapping_add(STRIDE);
                idx = idx.wrapping_add(1);
                if ((rec) as *const u8).read() == 0 {
                    break;
                }
            }
        }
        let f = lf_checker_rt::relocated(FLAGS);
        wr32(f, rd32(f) >> FLAG_SHR);
        wr32(lf_checker_rt::relocated(EPOCH), 0);
        0
    }
});
