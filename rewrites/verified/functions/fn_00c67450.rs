// original: 0x00C67450 cutscene_actor_adopt_match (proposed)
//
// thiscall (ecx = this, no stack arguments). Re-resolves the actor's live
// handles, then scans a global directory for a record matching this actor
// and adopts its appearance fields.
//
// Steps: flag word at +0x314 is set to 1; hook A (object slot +0x40) runs;
// hook B (slot +0xa0) resolves a handle, falling back to the stored word at
// +0x100 when it answers null, and a non-null answer is re-resolved and
// passed through hook C (its slot +0xe0); hook D (direct) runs when the
// final handle is null. Bit 1 of the byte at +0xf4 is set, hook E (direct,
// one argument, 0) runs, ten words at +0x2b8..+0x2dc are filled with -1 and
// the word at +0x2b4 is cleared.
//
// Then the directory at the global word 0x012E22A4 is scanned: count at
// +8 (skipped when not positive), flag bytes at +4, stride at +0xc, base at
// +0. For each index with the 0x80 flag bit clear, the candidate is
// stride*index+base; a null candidate, a kind byte at +0x10b8 other than 2,
// or a u16 key at +0x2e differing from this actor's key skips it. The first
// match copies four bytes at +0xf94.. into +0x2ad.., the word at +0x10c8
// into +0x2b4, runs hook F (direct, on the candidate) storing its low byte
// at +0x2b2, runs hook G (direct) and sets +0x2b1.
//
// Returns whatever eax holds at the end (the last hook result, or that
// result with its low word replaced by the last examined key when the scan
// ends without a match).
lf_checker_rt::export!(thiscall, rw_00C67450(this: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0x314;
        const SLOT_A: u32 = 0x40;
        const SLOT_B: u32 = 0xa0;
        const SLOT_C: u32 = 0xe0;
        const STORED: u32 = 0x100;
        const MODE: u32 = 0xf4;
        const MODE_BIT: u8 = 2;
        const SLOTS: u32 = 0x2b8;
        const NSLOTS: u32 = 10;
        const PICK: u32 = 0x2b4;
        const KEY: u32 = 0x2e;
        const DIR: u32 = 0x012E22A4;
        const FLAG_SKIP: u8 = 0x80;
        const KIND_OFF: u32 = 0x10b8;
        const KIND_WANT: u8 = 2;
        const COPY_SRC: u32 = 0xf94;
        const COPY_DST: u32 = 0x2ad;
        const NCOPY: u32 = 4;
        const LINK_SRC: u32 = 0x10c8;
        const TAG: u32 = 0x2b2;
        const DONE: u32 = 0x2b1;
        const HOOK_A: u32 = 1;
        const HOOK_B: u32 = 2;
        const HOOK_C: u32 = 3;
        const HOOK_D: u32 = 4;
        const HOOK_E: u32 = 5;
        const HOOK_F: u32 = 6;
        const HOOK_G: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(this + STAMP, 1);
        let vt = rd32(this);
        let hook_a: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + SLOT_A) as usize);
        hook_a(this);
        let hook_b: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + SLOT_B) as usize);
        let mut handle = hook_b(this);
        if handle == 0 {
            handle = rd32(this + STORED);
        } else {
            let rec = hook_b(this);
            let hook_c: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(rec) + SLOT_C) as usize);
            handle = hook_c(rec);
        }
        if handle == 0 {
            lf_checker_rt::callee_thiscall!(HOOK_D, u32, this);
        }
        wr8(this + MODE, rd8(this + MODE) | MODE_BIT);
        let e5 = lf_checker_rt::callee_thiscall!(HOOK_E, u32, this, 0);
        let mut eax = e5;
        for i in 0..NSLOTS {
            wr32(this + SLOTS + i * 4, 0xFFFF_FFFF);
        }
        wr32(this + PICK, 0);

        let dir = rd32(lf_checker_rt::relocated(DIR));
        let count = rd32(dir + 8) as i32;
        if count > 0 {
            let flags = rd32(dir + 4);
            let stride = rd32(dir + 12);
            let base = rd32(dir);
            let want = rd16(this + KEY);
            let mut d = 0i32;
            loop {
                if rd8(flags.wrapping_add(d as u32)) & FLAG_SKIP == 0 {
                    let rec = stride.wrapping_mul(d as u32).wrapping_add(base);
                    if rec != 0
                        && rd8(rec + KIND_OFF) == KIND_WANT
                        && rd16(rec + KEY) == want
                    {
                        for i in 0..NCOPY {
                            wr8(this + COPY_DST + i, rd8(rec + COPY_SRC + i));
                        }
                        wr32(this + PICK, rd32(rec + LINK_SRC));
                        let tag: u32 = lf_checker_rt::callee_thiscall!(HOOK_F, u32, rec);
                        wr8(this + TAG, tag as u8);
                        eax = lf_checker_rt::callee_thiscall!(HOOK_G, u32, this);
                        wr8(this + DONE, 1);
                        return eax;
                    }
                    if rec != 0 && rd8(rec + KIND_OFF) == KIND_WANT {
                        eax = (eax & 0xFFFF_0000) | rd16(rec + KEY);
                    }
                }
                d += 1;
                if !(d < count) {
                    break;
                }
            }
        }
        eax
    }
});
