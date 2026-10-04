// original: 0x00967650 timing_object_init (proposed)

/// Initialise a large timing/state object by constructing its sub-objects in
/// place and then writing eight identical constant blocks plus a zero tail.
///
/// `this` points to an object of at least 0x31d6 bytes. Three intercepted
/// callees construct the parts (all thiscall, no stack arguments, return
/// value ignored): element initialiser (id 1) over five strided runs and
/// seven single elements, header initialiser (id 2) over sixteen single
/// headers and one strided run of three, and a neighbouring initialiser
/// (id 3) called once. 83 calls in total, in a fixed order.
///
/// After the calls, eight blocks of eight words at stride 0x30 starting at
/// `+0x3060` are set to the same pattern (zero with 1.0 at word 3 and word
/// 5), then a zero word pair at `+0x31d0`/`+0x31d4`. Returns `this`.
///
/// Original: thiscall, no stack arguments, returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00967650(this: u32) -> u32 {
    unsafe {
        const ELEMENT_INIT: u32 = 1;
        const HEADER_INIT: u32 = 2;
        const NEIGHBOUR_INIT: u32 = 3;
        const ELEMENT_STRIDE: u32 = 0x1c;
        const HEADER_STRIDE: u32 = 0x28;
        const BLOCK_BASE: u32 = 0x3060;
        const BLOCK_STRIDE: u32 = 0x30;
        const BLOCK_COUNT: u32 = 8;
        const ONE_BITS: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn init_element(addr: u32) {
            unsafe {
                let _ = lf_checker_rt::callee_thiscall!(ELEMENT_INIT, u32, addr);
            }
        }
        #[inline(always)]
        unsafe fn init_header(addr: u32) {
            unsafe {
                let _ = lf_checker_rt::callee_thiscall!(HEADER_INIT, u32, addr);
            }
        }
        #[inline(always)]
        unsafe fn wr32(addr: u32, v: u32) {
            unsafe { (addr as *mut u32).write_unaligned(v) }
        }

        let mut i = 0u32;
        while i < 32 {
            init_element(this.wrapping_add(0x0a30).wrapping_add(i.wrapping_mul(ELEMENT_STRIDE)));
            i += 1;
        }
        let _ = lf_checker_rt::callee_thiscall!(NEIGHBOUR_INIT, u32, this.wrapping_add(0x0db0));
        for off in [0x1240u32, 0x1284, 0x12ac, 0x12d4, 0x12fc, 0x1324] {
            init_header(this.wrapping_add(off));
        }
        init_header(this.wrapping_add(0x16f0));
        let mut i = 0u32;
        while i < 4 {
            init_element(this.wrapping_add(0x1728).wrapping_add(i.wrapping_mul(ELEMENT_STRIDE)));
            i += 1;
        }
        init_header(this.wrapping_add(0x1798));
        for off in [0x1ca0u32, 0x1dd0, 0x1df8, 0x1e20, 0x1e48] {
            init_header(this.wrapping_add(off));
        }
        let mut i = 0u32;
        while i < 3 {
            init_header(this.wrapping_add(0x2114).wrapping_add(i.wrapping_mul(HEADER_STRIDE)));
            i += 1;
        }
        init_header(this.wrapping_add(0x218c));
        init_header(this.wrapping_add(0x21b4));
        let mut i = 0u32;
        while i < 12 {
            init_element(this.wrapping_add(0x21dc).wrapping_add(i.wrapping_mul(ELEMENT_STRIDE)));
            i += 1;
        }
        init_element(this.wrapping_add(0x2744));
        init_header(this.wrapping_add(0x2760));
        let mut i = 0u32;
        while i < 4 {
            init_element(this.wrapping_add(0x28cc).wrapping_add(i.wrapping_mul(ELEMENT_STRIDE)));
            i += 1;
        }
        let mut i = 0u32;
        while i < 4 {
            init_element(this.wrapping_add(0x294c).wrapping_add(i.wrapping_mul(ELEMENT_STRIDE)));
            i += 1;
        }
        for off in [0x29bcu32, 0x2a10, 0x2fa8, 0x2fc8, 0x3008, 0x302c] {
            init_element(this.wrapping_add(off));
        }

        let mut b = 0u32;
        while b < BLOCK_COUNT {
            let base = this.wrapping_add(BLOCK_BASE).wrapping_add(b.wrapping_mul(BLOCK_STRIDE));
            wr32(base, 0);
            wr32(base.wrapping_add(4), 0);
            wr32(base.wrapping_add(8), 0);
            wr32(base.wrapping_add(12), ONE_BITS);
            wr32(base.wrapping_add(16), 0);
            wr32(base.wrapping_add(20), ONE_BITS);
            wr32(base.wrapping_add(24), 0);
            wr32(base.wrapping_add(28), 0);
            b += 1;
        }
        wr32(this.wrapping_add(0x31d0), 0);
        unsafe { (this.wrapping_add(0x31d4) as *mut u16).write_unaligned(0) }
        this
    }
});
