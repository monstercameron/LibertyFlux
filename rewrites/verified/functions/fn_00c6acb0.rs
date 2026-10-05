// original: 0x00c6acb0 stream_acquire_slot (proposed)

/// Acquire a streaming slot from the bank at +0x404, strictly or not.
///
/// First every live handle is verified: any acceptance returns 0 at
/// once. Otherwise the bank is scanned for the best priority above
/// the bar (-1 when `strict` is nonzero, else 8), except the pinned
/// handle, which wins outright once past priority 0x14. The winner is
/// acquired and verified; a final rejection, an empty bank, or no
/// winner returns 0. Only the low byte of the answer is defined.
///
/// Original: thiscall with one stack word, eight call sites, reads
/// three globals, al-only return.
lf_checker_rt::export!(thiscall, rw_00c6acb0(this: u32, strict: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5CD8;
        const LOCK: u32 = 0x012B_4138;
        const PINNED: u32 = 0x012F_A434;
        const PRIO_OFF: u32 = 0x9C;
        const PIN_BAR: u32 = 0x14;
        const LOOSE_BAR: u32 = 8;
        const COUNT: u32 = 1;
        const FETCH: u32 = 2;
        const VERIFY: u32 = 3;
        const ACQUIRE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let table = lf_checker_rt::relocated(TABLE);
        let lock = rd32(lf_checker_rt::relocated(LOCK));
        let pinned = rd32(lf_checker_rt::relocated(PINNED));
        let bank = this.wrapping_add(0x404);
        let mut bar = if strict != 0 { 0xFFFF_FFFF } else { LOOSE_BAR };
        let mut hits = 0u32;
        let mut i = 0u32;
        let mut n: u32 = lf_checker_rt::callee_thiscall!(COUNT, u32, bank);
        if (n as i32) > 0 {
            loop {
                let h: u32 = lf_checker_rt::callee_thiscall!(FETCH, u32, bank, i);
                let v: u32 = lf_checker_rt::callee_cdecl!(VERIFY, u32, h, lock);
                if (v & 0xFF) != 0 {
                    hits = hits.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                n = lf_checker_rt::callee_thiscall!(COUNT, u32, bank);
                if !((i as i32) < (n as i32)) {
                    break;
                }
            }
            if (hits as i32) > 0 {
                return 0;
            }
        }
        let total: u32 = lf_checker_rt::callee_thiscall!(COUNT, u32, bank);
        if (total as i32) <= 0 {
            return 0;
        }
        let mut best = 0xFFFF_FFFFu32;
        let mut j = 0u32;
        while (j as i32) < (total as i32) {
            let h: u32 = lf_checker_rt::callee_thiscall!(FETCH, u32, bank, j);
            let obj = rd32(table.wrapping_add(h.wrapping_mul(4)));
            if h == pinned {
                if (rd32(obj.wrapping_add(PRIO_OFF)) as i32) > (PIN_BAR as i32) {
                    best = h;
                    break;
                }
            }
            let pri = rd32(obj.wrapping_add(PRIO_OFF));
            if (pri as i32) > (bar as i32) {
                best = h;
                bar = pri;
            }
            j = j.wrapping_add(1);
        }
        if (best as i32) < 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(ACQUIRE, u32, best, lock);
        let v: u32 = lf_checker_rt::callee_cdecl!(VERIFY, u32, best, lock);
        ((v & 0xFF) != 0) as u32
    }
});
