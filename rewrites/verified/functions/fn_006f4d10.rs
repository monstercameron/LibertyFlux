// original: 0x006F4D10 input_seq_header_build (proposed)

/// Build a 24-byte big-endian sequence header at `this` from sequence globals.
///
/// Calls the shared bit-writer callee twice (first with the source buffer at
/// `a0 + 0x18`, then with a constant), stamps the marker bytes 3 (at `+3`)
/// and 2 (at `+5`), then advances a 64-bit counter global by
/// `counter = counter_lo * STEP + counter_hi` (full 64-bit multiply-add with
/// `STEP = 0x5CDCFAA7`) and stores the new low byte at `+4`.
///
/// The remaining bytes are big-endian copies: the dword at `*a1` into
/// `+6..+9`, the low two bytes of `a2` into `+0xA..+0xB`, a counter dword
/// global into `+0xC..+0xF`, a counter word global into `+0x10..+0x11`, a
/// second dword global into `+0x12..+0x15`, and a second word global into
/// `+0x16..+0x17`. Bytes `+0..+2` are left to the bit-writer callee.
///
/// Returns the high byte of the second word global. `a3` is unread.
/// Thiscall with four stack words; the function also spills one global into
/// its incoming arg0 slot, which the rewrite cannot address, so the proof
/// runs with the stack check off.
lf_checker_rt::export!(thiscall, rw_006F4D10(this: u32, a0: u32, a1: u32, a2: u32, _a3: u32) -> u32 {
    unsafe {
        const BITWRITER: u32 = 1;
        const STEP: u64 = 0x5CDC_FAA7;
        const G_SEQ_LO: u32 = 0x0111_0458;
        const G_SEQ_HI: u32 = 0x0111_045C;
        const G_CNT_D0: u32 = 0x0111_03A0;
        const G_CNT_W1: u32 = 0x0111_03A4;
        const G_CNT_D1: u32 = 0x0111_03A8;
        const G_CNT_W0: u32 = 0x0111_03AC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn put_be32(dst: u32, v: u32) {
            unsafe {
                wr8(dst, (v >> 24) as u8);
                wr8(dst + 1, (v >> 16) as u8);
                wr8(dst + 2, (v >> 8) as u8);
                wr8(dst + 3, v as u8);
            }
        }
        #[inline(always)]
        unsafe fn put_be16(dst: u32, v: u16) {
            unsafe {
                wr8(dst, (v >> 8) as u8);
                wr8(dst + 1, v as u8);
            }
        }

        lf_checker_rt::callee_cdecl!(BITWRITER, u32, this, a0.wrapping_add(0x18), 0x10, 0);
        lf_checker_rt::callee_cdecl!(BITWRITER, u32, this, 1, 8, 0x10);
        wr8(this + 3, 3);

        let lo = rd32(lf_checker_rt::relocated(G_SEQ_LO));
        let hi = rd32(lf_checker_rt::relocated(G_SEQ_HI));
        let prod = (lo as u64).wrapping_mul(STEP).wrapping_add(hi as u64);
        let new_lo = prod as u32;
        wr32(lf_checker_rt::relocated(G_SEQ_LO), new_lo);
        wr32(lf_checker_rt::relocated(G_SEQ_HI), (prod >> 32) as u32);
        wr8(this + 4, new_lo as u8);

        let w0 = rd16(lf_checker_rt::relocated(G_CNT_W0)) as u32;
        let d0 = rd32(lf_checker_rt::relocated(G_CNT_D0));
        let w1 = rd16(lf_checker_rt::relocated(G_CNT_W1));
        let d1 = rd32(lf_checker_rt::relocated(G_CNT_D1));

        let p = rd32(a1);
        put_be32(this + 6, p);
        put_be16(this + 0x0a, a2 as u16);
        put_be32(this + 0x0c, d0);
        put_be16(this + 0x10, w1);
        put_be32(this + 0x12, d1);
        wr8(this + 5, 2);
        put_be16(this + 0x16, w0 as u16);
        (w0 >> 8) as u32
    }
});
