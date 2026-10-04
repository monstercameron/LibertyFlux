// original: 0x0094f470 nearest_candidate_report
/// Find the candidate nearest to the object's position and report it.
///
/// Queries a scripted candidate source (passed the object's position
/// vector, a 30.0 radius and a capacity of 32), then scores each returned
/// candidate id by Euclidean distance from the object's position to the
/// candidate's scaled grid position (signed 16-bit components times the
/// 0.125 horizontal and 0.015625 vertical scale constants). The closest
/// candidate wins, starting from the 9999.9 limit constant; ties keep the
/// earlier candidate and NaN distances never win. When a winner exists it
/// is formatted by a scripted formatter and the text is copied into the
/// object's 0x100 scratch area by a scripted wide-string copy.
export!(thiscall, rw_0094f470(this: u32) -> u32 {
    unsafe {
        const TABLE_GLOB: u32 = 0x1178284;
        const BEST_GLOB: u32 = 0xE89220;
        const S1_GLOB: u32 = 0xFE87A4;
        const S2_GLOB: u32 = 0xFE8720;
        const QUERY_THIS: u32 = 0x1177A80;
        const FMT_THIS: u32 = 0x116BFF0;
        const FMT_KEY: u32 = 0xE890C4;
        const LOOKUP_ID: u32 = 1;
        const QUERY_ID: u32 = 2;
        const FORMAT_ID: u32 = 5;
        const WCOPY_ID: u32 = 6;
        const COOKIE_ID: u32 = 7;

        let outer = callee_cdecl!(LOOKUP_ID, u32, 0);
        let inner = *((outer.wrapping_add(0x20)) as *const u32);
        let f0 = *((inner.wrapping_add(0x30)) as *const f32);
        let f1 = *((inner.wrapping_add(0x34)) as *const f32);
        let f2 = *((inner.wrapping_add(0x38)) as *const f32);
        let center = [f0.to_bits(), f1.to_bits(), f2.to_bits()];
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
        let s1 = *(relocated(S1_GLOB) as *const f32);
        let s2 = *(relocated(S2_GLOB) as *const f32);
        let mut best = *(relocated(BEST_GLOB) as *const f32);
        let mut winner = 0u32;
        let n = count as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let raw = ids[i as usize];
                let idx = (raw & 0xFFFF) as u32;
                let hi = raw >> 16;
                let base = *((table.wrapping_add(idx * 4)) as *const u32);
                let cand = *((base.wrapping_add(hi << 5).wrapping_add(0xC)) as *const u32);
                if cand != 0 {
                    let rec = base.wrapping_add(hi << 5);
                    let px = *((rec.wrapping_add(0x14)) as *const i16) as f32 * s1;
                    let py = *((rec.wrapping_add(0x16)) as *const i16) as f32 * s1;
                    let pz = *((rec.wrapping_add(0x18)) as *const i16) as f32 * s2;
                    let dx = f0 - px;
                    let dy = f1 - py;
                    let dz = f2 - pz;
                    // Order matches the original: (dy^2 + dx^2) + dz^2.
                    let d2 = dy * dy + dx * dx;
                    let d2 = d2 + dz * dz;
                    let dist = d2.sqrt();
                    // comiss+jbe keeps the incumbent unless strictly beaten.
                    if dist < best {
                        best = dist;
                        winner = cand;
                    }
                }
                i += 1;
            }
        }
        if winner != 0 {
            let s = callee_thiscall!(
                FORMAT_ID,
                u32,
                relocated(FMT_THIS),
                winner,
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
        let _: u32 = callee_cdecl!(COOKIE_ID, u32,);
        0
    }
});
