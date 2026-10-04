// original: 0x00ad5190 audio_scan_tables_windowed
// ---------------------------------------------------------------------------
// 0x00AD5190: scan two index tables through a sliding float window.
// ---------------------------------------------------------------------------
// Twelve outer by twelve middle steps sweep two counters from 0 to 5500 in
// steps of 500; each step centres a float window (value minus 3000, width
// 500). For every step pair, two tables are scanned: a stride-16 entry table
// whose three indices map through a shared word table, and the stride-8 work
// list from the neighbouring producer. A row fires only when all four of its
// mapped values land strictly inside the current windows (outer pair against
// the outer window, inner pair against the middle window); each firing row
// is reported to one helper with the step counters, the row number and a
// table tag. Takes no arguments and returns 6000.
export!(cdecl, rw_00ad5190() -> u32 {
    unsafe {
        const STEP: i32 = 500;
        const LIMIT: i32 = 6000;

        let c0 = f32::from_bits(*global::<u32>(0xfe8c78));
        let c1 = f32::from_bits(*global::<u32>(0xfe8c2c));
        let n1 = *global::<i32>(0x1550eac);
        let n2 = *global::<i32>(0x154e300);
        let entries = global::<u8>(0x1550eb4);
        let work = global::<u8>(0x154e308);
        let keys = global::<u8>(0x158e860);
        // Signed first key and signed/unsigned second key of table row r.
        let key0 = |r: i32| -> i32 {
            let off = (r as u32).wrapping_mul(8) as usize;
            *(keys.add(off) as *const i16) as i32
        };
        let key1s = |r: i32| -> i32 {
            let off = (r as u32).wrapping_mul(8) as usize;
            *(keys.add(off + 2) as *const i16) as i32
        };
        let key1u = |r: i32| -> u32 {
            let off = (r as u32).wrapping_mul(8) as usize;
            *(keys.add(off + 2) as *const u16) as u32
        };

        let mut ebp = 0u32;
        let mut outer = 0i32;
        while outer < LIMIT {
            let s5 = (outer as f32) - c0;
            let s6 = s5 + c1;
            let mut ebx = 0u32;
            let mut middle = 0i32;
            while middle < LIMIT {
                let s2 = (middle as f32) - c0;
                let s3 = s2 + c1;
                let mut edi = 0i32;
                while edi < n1 {
                    let row = entries.add((edi as usize) * 16);
                    let i0 = *(row as *const i16) as i32;
                    let i2 = *(row.add(2) as *const i16) as i32;
                    let i4 = *(row.add(4) as *const i16) as i32;
                    let hi = key0(i2) as f32;
                    let lo = key0(i0) as f32;
                    let hi2 = key1s(i4) as f32;
                    let lo2 = key1s(i0) as f32;
                    if hi > s5 && s6 > lo && hi2 > s2 && s3 > lo2 {
                        callee_cdecl!(1, u32, ebp, ebx, edi as u32, 1);
                    }
                    edi += 1;
                }
                let mut esi = 0i32;
                while esi < n2 {
                    let row = work.add((esi as usize) * 8);
                    let j0 = *(row as *const i16) as i32;
                    let j1 = *(row.add(2) as *const i16) as i32;
                    let j2 = *(row.add(4) as *const i16) as i32;
                    let f4 = key0(j0) as f32;
                    let f0 = key0(j1) as f32;
                    let cc = key1u(j0);
                    let dd = key1u(j2);
                    // The original reduces the unsigned pair but converts the
                    // result as a signed halfword.
                    let lo = ((cc.min(dd) as u16) as i16) as f32;
                    let hi = ((cc.max(dd) as u16) as i16) as f32;
                    if f0 > s5 && s6 > f4 && hi > s2 && s3 > lo {
                        callee_cdecl!(2, u32, ebp, ebx, esi as u32, 2);
                    }
                    esi += 1;
                }
                middle += STEP;
                ebx += 1;
            }
            outer += STEP;
            ebp += 1;
        }
        LIMIT as u32
    }
});
