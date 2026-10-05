// original: 0x009D2970 scan_fetch_relocate (proposed)
//
/// Scans pairs for a predicate hit, then fetches a block and relocates it.
///
/// Walks the `count` (`[this+0x0c]`, 16-bit) 8-byte pairs at `[this+0x08]`,
/// calling the predicate (callee 1) with each pair's first dword and `arg1`;
/// a zero answer returns 0 at once. Otherwise stages `[this+0x04]` and `arg0`
/// into frame slots, fetches a 128-byte block through the provider (callee 2
/// with `(buf, 0x80, arg1, 0xFC9C85, 0)` and a constant context; the buffer
/// is out-only frame scratch, so the argument is skipped and the outputs are
/// observed through the copies below), copies block plus staged slots
/// onward, allocates a fresh slot (callee 3 with `0x10`) and copies the
/// staged 136 bytes there, returning the new slot. The trailing security
/// cookie check (callee 4) preserves registers and is called for sequence
/// parity. Thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_009D2970(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0x34;
        const NEG: u32 = 0xffffffff;
        const MAGIC2_FILE: u32 = 0xfc9c85;
        const BUFLEN: u32 = 0x80;
        const WORDS: usize = 0x22;
        const PRED: u32 = 1;
        const PROVIDER: u32 = 2;
        const ALLOC: u32 = 3;
        const COOKIE: u32 = 4;
        const CTX_FILE: u32 = 0x110c0a0;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = (this.wrapping_add(0x0c) as *const u16).read_unaligned() as u32;
        if count != 0 {
            let pairs = rd(this.wrapping_add(8));
            let mut i = 0u32;
            loop {
                let first = rd(pairs.wrapping_add(i.wrapping_mul(8)));
                let ans: u32 = lf_checker_rt::callee_cdecl!(PRED, u32, first, arg1);
                if ans == 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
                    return 0;
                }
                i += 1;
                if (i as i32) >= (count as i32) {
                    break;
                }
            }
        }
        let tag = (this.wrapping_add(4) as *const u16).read_unaligned() as u32;
        // Frame staging: the original first writes (0x34, -1) to the two tail
        // slots, then overwrites them with (arg0, tag) before the copies.
        let _ = (MAGIC, NEG);
        let mut buf = [0u32; 32];
        // Both immediates below are relocated absolute addresses in the original.
        let _: u32 = lf_checker_rt::callee_thiscall!(
            PROVIDER, u32, lf_checker_rt::relocated(CTX_FILE),
            (&mut buf as *mut u32) as u32, BUFLEN, arg1,
            lf_checker_rt::relocated(MAGIC2_FILE), 0);
        let mut stage = [0u32; WORDS];
        stage[..32].copy_from_slice(&buf);
        stage[32] = arg0;
        stage[33] = tag;
        let p: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, this, 0x10);
        core::ptr::copy_nonoverlapping(stage.as_ptr(), p as *mut u32, WORDS);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        p
    }
});
