// original: 0x00c69f70 stream_drop_far_rows (proposed)

/// Drop the ids of every row within range of `pos` from the id array.
///
/// Two row tables (read from their globals) are scanned top-down.
/// Each table gives a row base, a flag-byte array, a count and a
/// stride; flagged-out (high bit) and null rows are skipped. For a
/// live row the squared distance from `pos` (two floats) to the
/// row's anchor (two floats at its coord block +0x30/+0x34) is
/// compared against the 100.0 threshold: rows strictly inside have
/// their sign-extended halfword id at row offset 0x2E removed from
/// the id array through the shared remover. Float order is the
/// original's: dy*dy + dx*dx.
///
/// Original: thiscall with one stack word (two-float position), two
/// call sites of one callee, reads three globals.
lf_checker_rt::export!(thiscall, rw_00c69f70(this: u32, pos: u32) -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x018B_6F1C;
        const TABLE_B: u32 = 0x018B_6F10;
        const LIMIT: u32 = 0x00FE_8BB0;
        const REMOVE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        unsafe fn scan(this: u32, pos: u32, tab: u32, limit: f32) {
            unsafe {
                let base = rd32(tab);
                let flags = rd32(tab.wrapping_add(4));
                let count = rd32(tab.wrapping_add(8));
                let stride = rd32(tab.wrapping_add(0xC));
                if (count as i32) == 0 {
                    return;
                }
                let mut i = (count as i32).wrapping_sub(1);
                loop {
                    let f = unsafe { (flags.wrapping_add(i as u32) as *const u8).read() };
                    if f & 0x80 == 0 {
                        let row = stride.wrapping_mul(i as u32).wrapping_add(base);
                        if row != 0 {
                            let co = rd32(row.wrapping_add(0x20));
                            let dx = sub(rdf(co.wrapping_add(0x30)), rdf(pos));
                            let dy = sub(rdf(co.wrapping_add(0x34)), rdf(pos.wrapping_add(4)));
                            let d2 = add(mul(dy, dy), mul(dx, dx));
                            if limit > d2 {
                                let id =
                                    ((row.wrapping_add(0x2E) as *const u16).read_unaligned()
                                        as i16) as u32;
                                lf_checker_rt::callee_thiscall!(REMOVE, u32, this, id);
                            }
                        }
                    }
                    if i == 0 {
                        break;
                    }
                    i = i.wrapping_sub(1);
                }
            }
        }

        let limit = rdf(lf_checker_rt::relocated(LIMIT));
        scan(this, pos, rd32(lf_checker_rt::relocated(TABLE_A)), limit);
        scan(this, pos, rd32(lf_checker_rt::relocated(TABLE_B)), limit);
        0
    }
});
