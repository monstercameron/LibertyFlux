// original: 0x005d5f00 html_text_node_init (proposed)
//
// Initialise a text node over a caller string, copying the string into a
// freshly allocated buffer.
//
// `this` is the node and the two stack arguments are a byte string (`s`)
// and a length (`n`). A base constructor runs first with `this + 0x14`,
// then the node header is written (vtable, zeroed words, kind 1 at `+4`)
// and a probe callee classifies `s`. When the probe answers zero, `n` is
// used directly: `(n + 1) * 2` bytes are allocated through the game's
// allocator (TLS slot 0, `+8` object, vtable slot `+8`, constant arguments
// `0x10, 0`; the multiply is unsigned with overflow saturating to
// 0xffffffff) and a store callee records the buffer, the string and -1.
// Otherwise, when `n` is negative it is replaced by the NUL-terminated
// length of `s` plus one, the same allocation runs, and a copy callee
// duplicates the string into the buffer when the allocation succeeded.
// The vtable and header words are file addresses, relocated here.
//
// Original: 0x005d5f00 (thiscall, two stack words; returns `this`).
lf_checker_rt::export!(thiscall, rw_005d5f00(this: u32, s: u32, n: u32) -> u32 {
    unsafe {
        const ID_CTOR: u32 = 1;
        const ID_PROBE: u32 = 2;
        const ID_STORE: u32 = 4;
        const ID_COPY: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Unsigned `(n + 1) * 2`, saturating to all-ones on overflow,
        /// as the original's `lea; mul; seto; neg; or` sequence.
        #[inline(always)]
        fn buf_size(n: u32) -> u32 {
            let (v, overflow) = n.wrapping_add(1).overflowing_mul(2);
            if overflow {
                u32::MAX
            } else {
                v
            }
        }

        wr32(this, lf_checker_rt::relocated(0x00fe0b94));
        wr32(this.wrapping_add(8), 0);
        wr32(this.wrapping_add(0xc), 0);
        wr32(this.wrapping_add(0x10), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_CTOR, u32, this.wrapping_add(0x14));
        wr32(this, lf_checker_rt::relocated(0x00fe0b60));
        wr32(this.wrapping_add(0xd8), 0);
        wr32(this.wrapping_add(0xdc), 0);
        wr32(this.wrapping_add(4), 1);
        let probe: u32 = lf_checker_rt::callee_thiscall!(ID_PROBE, u32, s);
        let tls0 = lf_checker_rt::tls_slot(0);
        let alloc = rd32(tls0.wrapping_add(8));
        let malloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(alloc).wrapping_add(8)) as usize);
        if (probe & 0xff) == 0 {
            let buf = malloc(alloc, buf_size(n), 0x10, 0);
            wr32(this.wrapping_add(0xdc), buf);
            let _: u32 = lf_checker_rt::callee_cdecl!(ID_STORE, u32, buf, s, 0xffff_ffff);
        } else {
            let mut m = n as i32;
            if m < 0 {
                let mut len = 0u32;
                while ((s.wrapping_add(len)) as *const u8).read() != 0 {
                    len = len.wrapping_add(1);
                }
                m = len.wrapping_add(1) as i32;
            }
            let buf = malloc(rd32(tls0.wrapping_add(8)), buf_size(m as u32), 0x10, 0);
            wr32(this.wrapping_add(0xdc), buf);
            if buf != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(ID_COPY, u32, s, buf);
            }
        }
        this
    }
});
