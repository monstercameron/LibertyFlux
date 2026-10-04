// original: 0x0094f0b0 timed_cache_update
/// Refresh a 16-slot timed pointer cache from a scripted candidate query.
///
/// When the mode global's low five bits equal 0x13, queries a scripted
/// candidate source (passed the object's position vector, a 30.0 radius and
/// a capacity of 32) and folds each returned candidate id into the cache:
/// an id resolving to null is skipped; an id already cached only refreshes
/// that slot's timestamp; a new id takes the highest empty slot, or evicts
/// the slot with the smallest timestamp when the cache is full (first
/// minimum wins ties). Every newly stored id is passed through two
/// scripted reporter gates, and when the second gate answers zero the id is
/// formatted by a scripted formatter and the text is copied into the
/// object's 0x100 scratch area by a scripted wide-string copy. Finally,
/// slots whose timestamp is more than 0x7530 ticks older than the clock
/// global are cleared. When the mode gate fails the cache is untouched.
export!(thiscall, rw_0094f0b0(this: u32) -> u32 {
    unsafe {
        const GATE_GLOB: u32 = 0x1173604;
        const CLOCK_GLOB: u32 = 0x11735B4;
        const TABLE_GLOB: u32 = 0x1178284;
        const QUERY_THIS: u32 = 0x1177A80;
        const FMT_THIS: u32 = 0x116BFF0;
        const FMT_KEY: u32 = 0xE890B8;
        const QUERY_ID: u32 = 2;
        const LOOKUP_ID: u32 = 1;
        const REPORTER_A_ID: u32 = 3;
        const REPORTER_B_ID: u32 = 4;
        const FORMAT_ID: u32 = 5;
        const WCOPY_ID: u32 = 6;
        const COOKIE_ID: u32 = 7;
        const SLOTS: usize = 16;
        const EXPIRY_TICKS: u32 = 0x7530;

        if (*(relocated(GATE_GLOB) as *const u32) & 0x1F) != 0x13 {
            let _: u32 = callee_cdecl!(COOKIE_ID, u32,);
            return 0;
        }
        let clock = *(relocated(CLOCK_GLOB) as *const u32);
        let outer = callee_cdecl!(LOOKUP_ID, u32, 0);
        let inner = *((outer.wrapping_add(0x20)) as *const u32);
        let center = [
            *((inner.wrapping_add(0x30)) as *const u32),
            *((inner.wrapping_add(0x34)) as *const u32),
            *((inner.wrapping_add(0x38)) as *const u32),
        ];
        // Candidate buffer: 33 words pre-filled with -1; the query
        // overwrites the leading words with candidate ids.
        let mut ids = [0xFFFFFFFFu32; 33];
        let count = callee_thiscall!(
            QUERY_ID,
            u32,
            relocated(QUERY_THIS),
            center.as_ptr() as u32,
            0x41F00000u32,
            0x20u32,
            ids.as_mut_ptr() as u32,
            0u32,
            0u32,
            0u32
        );
        let table = relocated(TABLE_GLOB);
        let slot = |k: usize| (this.wrapping_add((k as u32) * 4)) as *mut u32;
        let stamp = |k: usize| (this.wrapping_add(0x40 + (k as u32) * 4)) as *mut u32;
        let n = count as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let raw = ids[i as usize];
                let idx = (raw & 0xFFFF) as u32;
                let hi = raw >> 16;
                let base = *((table.wrapping_add(idx * 4)) as *const u32);
                let edi = *((base.wrapping_add(hi << 5).wrapping_add(0xC)) as *const u32);
                if edi != 0 {
                    let mut found: i32 = -1;
                    let mut last_empty: i32 = -1;
                    let mut k = 0i32;
                    while k < SLOTS as i32 {
                        let v = *slot(k as usize);
                        if v == edi {
                            found = k;
                            break;
                        }
                        if v == 0 {
                            last_empty = k;
                        }
                        k += 1;
                    }
                    if found >= 0 {
                        // found is always below SLOTS here (the scan only
                        // assigns 0..15); the stamp is refreshed in place.
                        *stamp(found as usize) = clock;
                    } else {
                        let at = if last_empty >= 0 {
                            last_empty as usize
                        } else {
                            // Cache full: evict the smallest timestamp,
                            // first minimum wins ties.
                            let mut min_v = *stamp(0);
                            let mut min_k = 0usize;
                            for k in 1..SLOTS {
                                let v = *stamp(k);
                                if v < min_v {
                                    min_v = v;
                                    min_k = k;
                                }
                            }
                            min_k
                        };
                        *slot(at) = edi;
                        *stamp(at) = clock;
                        let rep = callee_cdecl!(REPORTER_A_ID, u32,);
                        let gate = callee_thiscall!(
                            REPORTER_B_ID,
                            u32,
                            rep.wrapping_add(0x80)
                        );
                        if gate == 0 {
                            let s = callee_thiscall!(
                                FORMAT_ID,
                                u32,
                                relocated(FMT_THIS),
                                edi,
                                relocated(FMT_KEY)
                            );
                            let _: u32 = callee_cdecl!(
                                WCOPY_ID,
                                u32,
                                this.wrapping_add(0x100),
                                s,
                                0xFFFFFFFFu32
                            );
                        }
                    }
                }
                i += 1;
            }
        }
        for k in 0..SLOTS {
            if *slot(k) != 0 && clock > (*stamp(k)).wrapping_add(EXPIRY_TICKS) {
                *slot(k) = 0;
            }
        }
        let _: u32 = callee_cdecl!(COOKIE_ID, u32,);
        0
    }
});
