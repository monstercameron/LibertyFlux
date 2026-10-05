// original: 0x00b76660 ped_task_entry_init (proposed)

/// Initialise a ped task entry from four argument words.
///
/// `this` points at the entry (fields up to `+0xbca`). Word 0's low byte
/// and word 1's low byte (zero-extended) select the entry's kind slots;
/// word 2 is kept as the source pointer and, when non-null, resolved
/// through two callees into the entry's handle slot. A scratch descriptor
/// with a fixed pattern is built for the second callee; it is never read
/// back. When word 3's low byte is set, the handle is converted to a
/// scaled integer (`(u32 as f32) * 0.8`, truncated toward zero, with the
/// x87/SSE out-of-range result `0x80000000`) and a fourth callee maps it
/// into the entry's index slot. The remaining slots get fixed defaults.
/// No defined return value.
///
/// Original: 0x00b76660 (thiscall, four stack words).
unsafe fn run_00b76660(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, skip_store_bb8: bool) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x0115dc18;
        const SCALE: u32 = 0x00fe8898;
        const ID_SETUP: u32 = 1;
        const ID_RESOLVE: u32 = 2;
        const ID_FETCH: u32 = 3;
        const ID_MAP: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// `cvttss2si` semantics: truncate toward zero; anything outside
        /// the i32 range, NaN included, gives `0x80000000` (Rust's `as`
        /// saturates instead, so it cannot be used directly).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x >= -2147483648.0 && x < 2147483648.0 {
                x as i32
            } else {
                0x80000000u32 as i32
            }
        }

        lf_checker_rt::callee_thiscall!(ID_SETUP, u32, this);
        ((this.wrapping_add(0xbc0)) as *mut u8).write((a0 & 0xff) as u8);
        wr32(this.wrapping_add(0xbc4), a1 & 0xff);
        wr32(this.wrapping_add(0x960), a2);
        wr32(this.wrapping_add(0x99c), 0);
        ((this.wrapping_add(0xbca)) as *mut u16).write_unaligned(1);
        wr32(this.wrapping_add(0xaa0), 0);
        wr32(this.wrapping_add(0xab0), 0xffffffff);
        wr32(this.wrapping_add(0xaac), 0xffffffff);
        wr32(this.wrapping_add(0xaa8), 0xffffffff);
        wr32(this.wrapping_add(0xaa4), 0xffffffff);
        if a2 != 0 {
            let key = rd32(a2.wrapping_add(4));
            let tok = lf_checker_rt::callee_thiscall!(ID_RESOLVE, u32, lf_checker_rt::relocated(REGISTRY), key);
            if tok != 0 {
                // Scratch descriptor for the fetch callee; write-only.
                let mut desc = [0u32; 24];
                let dp = desc.as_mut_ptr() as u32;
                let got = lf_checker_rt::callee_cdecl!(ID_FETCH, u32, tok, dp);
                wr32(this.wrapping_add(0x994), rd32(got));
            }
        }
        if ((a3 & 0xff) as u8) != 0 {
            let h = rd32(this.wrapping_add(0x994));
            let scaled = mul((h as f64) as f32, rdf(lf_checker_rt::relocated(SCALE)));
            let idx = cvtt(scaled);
            let mapped = lf_checker_rt::callee_cdecl!(ID_MAP, u32, 0, idx as u32);
            wr32(this.wrapping_add(0x990), mapped);
        }
        if !skip_store_bb8 {
            wr32(this.wrapping_add(0xbb8), 0);
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00b76660(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe { run_00b76660(this, a0, a1, a2, a3, false) }
});
