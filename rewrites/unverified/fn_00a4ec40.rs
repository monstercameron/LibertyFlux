// original: 0x00a4ec40 vehicle_maybe_flag_f1d (proposed)

/// Set the done bit at +0xf1d once every gate passes, else do nothing.
///
/// Ten gates in order: the bit not already set; the word at +0x2c plus the
/// global tick landing on a 32-boundary; a non-null link at +0x12b0 whose
/// adjusted value does not exceed the global limit (unsigned); the word at
/// +0x1304 zero; the byte at +0x10b8 not 2; the global float not strictly
/// greater than the float at +0x28 in the object at +0x20 (NaN falls
/// through); the byte at +0x11a even; the state callee answering zero; the
/// probe callee on the object at +0x30 answering zero. Only then the bit is
/// set. Thiscall, no stack words, two callees, no result.
lf_checker_rt::export!(thiscall, rw_00a4ec40(this: u32) -> u32 {
    unsafe {
        const G_TICK: u32 = 0x01173604;
        const G_LIMIT: u32 = 0x011735b4;
        const G_FLOAT: u32 = 0x00fe8874;
        const STATE: u32 = 1;
        const PROBE: u32 = 2;
        const LINK_ADJ: u32 = 0x36b0;
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        if rd8(this.wrapping_add(0xf1d)) & 2 != 0 {
            return 0;
        }
        let w = rd16(this.wrapping_add(0x2c));
        let tick = rd32(lf_checker_rt::relocated(G_TICK));
        if (w.wrapping_add(tick)) & 0x1f != 0 {
            return 0;
        }
        let e = rd32(this.wrapping_add(0x12b0));
        if e == 0 {
            return 0;
        }
        let limit = rd32(lf_checker_rt::relocated(G_LIMIT));
        if e.wrapping_add(LINK_ADJ) > limit {
            return 0;
        }
        if rd32(this.wrapping_add(0x1304)) != 0 {
            return 0;
        }
        if rd8(this.wrapping_add(0x10b8)) == 2 {
            return 0;
        }
        let g = rdf(lf_checker_rt::relocated(G_FLOAT));
        let o = rd32(this.wrapping_add(0x20));
        if g > rdf(o.wrapping_add(0x28)) {
            return 0;
        }
        if rd8(this.wrapping_add(0x11a)) & 1 != 0 {
            return 0;
        }
        let r1: u32 = lf_checker_rt::callee_thiscall!(STATE, u32, this);
        if (r1 & 0xff) != 0 {
            return 0;
        }
        let r2: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, o.wrapping_add(0x30));
        if (r2 & 0xff) != 0 {
            return 0;
        }
        (this.wrapping_add(0xf1d) as *mut u8).write(rd8(this.wrapping_add(0xf1d)) | 2);
        0
    }
});
