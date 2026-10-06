// original: 0x00bf7440 encode_store_triple
/// Encode a record through two helpers sharing a scratch buffer, then store.
///
/// Builds a four-word zeroed scratch buffer and passes it to the thiscall
/// fetcher with ECX = `peer`; its scripted words land in the buffer. Then
/// calls the cdecl encoder on the same buffer and stores its answer to
/// this `+0x24`. Finally copies peer `+0x30/0x34/0x38` to this
/// `+0x18/0x1c/0x20` and returns peer `+0x38`. The fetcher's answer is
/// ignored; the encoder's is observed in this `+0x24`.
///
/// Original: 0x00BF7440 (thiscall, one stack word: peer).
lf_checker_rt::export!(thiscall, rw_00bf7440(this: u32, peer: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const CODE_OFF: u32 = 0x24;
        const SRC_TRIPLE: u32 = 0x30;
        const DST_TRIPLE: u32 = 0x18;
        let mut buf = [0u32; 4];
        lf_checker_rt::callee_thiscall!(1, u32, peer, buf.as_mut_ptr() as u32);
        let code = lf_checker_rt::callee_cdecl!(2, u32, buf.as_ptr() as u32);
        wr32(this + CODE_OFF, code);
        let x = rd32(peer + SRC_TRIPLE);
        let y = rd32(peer + SRC_TRIPLE + 4);
        let z = rd32(peer + SRC_TRIPLE + 8);
        wr32(this + DST_TRIPLE, x);
        wr32(this + DST_TRIPLE + 4, y);
        wr32(this + DST_TRIPLE + 8, z);
        z
    }
});
