// original: 0x00bf73a0 fetch_decode_store_b
/// Decode a record through two helpers sharing a scratch buffer, then store.
///
/// Builds a four-word zeroed scratch buffer and passes it to the cdecl
/// loader with this `+0x2c`; the loader's scripted words land in the buffer.
/// Then calls the thiscall decoder with ECX = `peer` and the same buffer.
/// Finally copies this `+0x30/0x34/0x38` to peer `+0x30/0x34/0x38` and
/// returns this `+0x38`. Neither helper's answer is read.
///
/// Original: 0x00BF73A0 (thiscall, one stack word: peer).
lf_checker_rt::export!(thiscall, rw_00bf73a0(this: u32, peer: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const KEY_OFF: u32 = 0x2c;
        const TRIPLE: u32 = 0x30;
        let mut buf = [0u32; 4];
        lf_checker_rt::callee_cdecl!(1, u32, buf.as_mut_ptr() as u32,
            rd32(this + KEY_OFF));
        lf_checker_rt::callee_thiscall!(2, u32, peer, buf.as_ptr() as u32);
        let x = rd32(this + TRIPLE);
        let y = rd32(this + TRIPLE + 4);
        let z = rd32(this + TRIPLE + 8);
        wr32(peer + TRIPLE, x);
        wr32(peer + TRIPLE + 4, y);
        wr32(peer + TRIPLE + 8, z);
        z
    }
});
