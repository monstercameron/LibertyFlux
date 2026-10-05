// original: 0x009AAC30 audio_entry_release (proposed)

/// Audio entry release: finds `key` across three id tables and drops one
/// reference from the first match.
///
/// A null `key` returns at once. Otherwise each of the three lookup callees
/// in turn is asked for (`sub`, out-count): it returns an array of 8-byte
/// entries (id dword, refcount dword) and writes the entry count through
/// the out-pointer. The first entry whose id equals `key` has its refcount
/// decremented and the search stops; a miss in all three tables changes
/// nothing. The stack words are (`sub`, `key`): `sub` is forwarded to the
/// callees, `key` is the searched id. The original reuses the `key` argument
/// slot as each out-slot after clearing its low byte; the single out-word
/// below mirrors that (seeded from `key`, low byte cleared before every
/// call) so the call-time snapshots match. Returns nothing.
/// Original: 0x009AAC30 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_009AAC30(this: u32, sub: u32, key: u32) -> u32 {
    unsafe {
        const ENTRY_SIZE: u32 = 8;
        const REF_OFF: u32 = 4;
        const LOOKUP_A: u32 = 1;
        const LOOKUP_B: u32 = 2;
        const LOOKUP_C: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if key == 0 {
            return 0;
        }
        let mut out: u32 = key & 0xffffff00;
        out &= 0xffffff00;
        let arr_a: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP_A,
            u32,
            this,
            sub,
            &mut out as *mut u32 as u32
        );
        let mut i: u32 = 0;
        let n_a = out & 0xff;
        while i < n_a {
            let e = arr_a.wrapping_add(i.wrapping_mul(ENTRY_SIZE));
            if rd32(e) == key {
                wr32(e.wrapping_add(REF_OFF), rd32(e.wrapping_add(REF_OFF)).wrapping_sub(1));
                return 0;
            }
            i += 1;
        }
        out &= 0xffffff00;
        let arr_b: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP_B,
            u32,
            this,
            sub,
            &mut out as *mut u32 as u32
        );
        let mut j: u32 = 0;
        let n_b = out & 0xff;
        while j < n_b {
            let e = arr_b.wrapping_add(j.wrapping_mul(ENTRY_SIZE));
            if rd32(e) == key {
                wr32(e.wrapping_add(REF_OFF), rd32(e.wrapping_add(REF_OFF)).wrapping_sub(1));
                return 0;
            }
            j += 1;
        }
        out &= 0xffffff00;
        let arr_c: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP_C,
            u32,
            this,
            sub,
            &mut out as *mut u32 as u32
        );
        let mut k: u32 = 0;
        let n_c = out & 0xff;
        while k < n_c {
            let e = arr_c.wrapping_add(k.wrapping_mul(ENTRY_SIZE));
            if rd32(e) == key {
                wr32(e.wrapping_add(REF_OFF), rd32(e.wrapping_add(REF_OFF)).wrapping_sub(1));
                return 0;
            }
            k += 1;
        }
        0
    }
});
