// original: 0x00c69010 table_member_check
// Test whether an id belongs to a table. When the mode flag is set the
// answer comes from hashing a base value modulo a global and probing a
// small word table (2 or 3 probes depending on the byte at +0x712);
// otherwise a seeded 42-row table search like rw_00c681b0 decides.
export!(thiscall, rw_00c69010(obj: u32, id: u32) -> u32 {
    unsafe {
        const IDIV_TAB: u32 = 0x169d1f8;
        const IDIV_MOD: u32 = 0x169e2f4;
        const SEED: u32 = 0x169e3dc;
        const PA: u32 = 0x169c47c;
        const PB: u32 = 0x169c478;
        const WORDS: u32 = 0x169c488;
        const WSTRIDE: u32 = 0x50;
        const COUNTS: u32 = 0x169e248;
        const COUNTS_END: u32 = 0x169e2f0;
        const FLAGS: u32 = 0x168ace8;
        let flag: u32 = callee_cdecl!(1, u32,);
        if flag & 0xff != 0 {
            let base: u32 = callee_thiscall!(2, u32, obj);
            let m = *global::<u32>(IDIV_MOD) as i32;
            let n = if *((obj as *const u8).add(0x712)) == 0 { 3 } else { 2 };
            let mut c = 0i32;
            while c < n {
                let q = (c as u32).wrapping_add(base);
                let rem = (q as i32) % m;
                let w = *((relocated(IDIV_TAB).wrapping_add((rem as u32).wrapping_mul(2))) as *const u16) as u32;
                if w == id {
                    return 1;
                }
                c += 1;
            }
            return 0;
        }
        let seed = *global::<u32>(SEED);
        if seed == 0 {
            return 0;
        }
        let a = *global::<u32>(PA);
        let b = *global::<u32>(PB);
        let t = (*((seed as *const u8).add(0x20)) & 0x7f) as u32;
        let off = a.wrapping_add(b.wrapping_mul(2)).wrapping_mul(0x47).wrapping_add(t).wrapping_mul(0x2a);
        let mut flagp = relocated(FLAGS).wrapping_add(off);
        let mut cntp = relocated(COUNTS);
        let mut row = relocated(WORDS);
        let endp = relocated(COUNTS_END);
        loop {
            if *((flagp) as *const u8) != 0 {
                let n = *(cntp as *const i32);
                if n > 0 {
                    let mut w = row as *const u16;
                    let mut k = 0i32;
                    while k < n {
                        if *w as u32 == id {
                            return 1;
                        }
                        w = w.add(1);
                        k += 1;
                    }
                }
            }
            cntp = cntp.wrapping_add(4);
            row = row.wrapping_add(WSTRIDE);
            flagp = flagp.wrapping_add(1);
            if !(cntp < endp) {
                break;
            }
        }
        0
    }
});
