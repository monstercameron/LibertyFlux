// original: 0x006340D0 tls_pool_register (proposed)

/// Register one record in a thread-local pool and append its handle.
///
/// `this` keeps an array base at `+0xE8` and a 16-bit used-count at `+0xEC`.
/// The pool allocator is reached through TLS slot 0 (`[slot0+8]`), whose
/// vtable slot 2 allocates: first a small header (0x18), then a 0x30-byte
/// descriptor that is stamped with a vtable pointer, a version word, a
/// constant and zeros. Two helpers run over the pair (one takes three
/// unread padding words past its real argument), the descriptor is stored
/// into the header, and a third allocation of `count*4` bytes, saturated to
/// all-bits on overflow, receives a copy of `count` dwords from `src` when
/// `count` is positive (SIGNED test). The low byte of `a4` lands at header
/// `+0x15`. The used-count is bumped (wrapping 16-bit store) and the handle
/// appended at `base[old]`; the return is the new count as a full 32-bit
/// value, so a wrapping store still returns 0x10000. thiscall, four stack
/// words (tag, signed count, source array, flags byte).
lf_checker_rt::export!(thiscall, rw_006340D0(this: u32, a1: u32, count: u32, src: u32, a4: u32) -> u32 {
    unsafe {
        const VTABLE_ALLOC_SLOT: u32 = 8;
        const DESC_VTABLE: u32 = 0x00FE_3854;
        const DESC_MAGIC: u32 = 0x007F_0000;
        const HDR_DESC: u32 = 0x00;
        const HDR_BUF: u32 = 0x04;
        const HDR_COUNT: u32 = 0x08;
        const HDR_FLAG: u32 = 0x15;
        const THIS_BASE: u32 = 0xE8;
        const THIS_USED: u32 = 0xEC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn valloc(alloc: u32, n: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(alloc);
                let slot: u32 = rd32(vt.wrapping_add(VTABLE_ALLOC_SLOT));
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(alloc, n, 0x10, 0)
            }
        }

        let edi: u32 = lf_checker_rt::tls_slot(0);
        let alloc: u32 = rd32(edi.wrapping_add(8));
        let s1: u32 = valloc(alloc, 0x18);
        let ebx: u32 = if s1 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, s1)
        } else {
            0
        };
        let s2: u32 = valloc(alloc, 0x30);
        let eax: u32 = if s2 != 0 {
            wr32(s2, lf_checker_rt::relocated(DESC_VTABLE));
            wr32(s2.wrapping_add(4), 1);
            wr32(s2.wrapping_add(8), DESC_MAGIC);
            wr32(s2.wrapping_add(0x0C), 0);
            wr32(s2.wrapping_add(0x10), 0);
            wr32(s2.wrapping_add(0x14), 0);
            wr32(s2.wrapping_add(0x18), 0);
            wr32(s2.wrapping_add(0x1C), 0);
            wr32(s2.wrapping_add(0x20), 0);
            wr32(s2.wrapping_add(0x24), 0);
            (s2.wrapping_add(0x28) as *mut u16).write_unaligned(0);
            s2
        } else {
            0
        };
        wr32(ebx.wrapping_add(HDR_DESC), eax);
        // Helper A takes four stack words; past a1 the original passes the
        // leftover words of its own earlier pushes (0, 0x10, 0x30), still on
        // the stack under the reserved slots: deterministic, so passed as is.
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, eax, a1, 0x30, 0x10, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, ebx.wrapping_add(0x0C), a1);
        let ebp: i32 = count as i32;
        // `mul 4; seto; neg; or`: saturate to all-bits on overflow.
        let (prod, ov) = (ebp as u32).overflowing_mul(4);
        let size: u32 = if ov { 0xFFFF_FFFF } else { prod };
        let s3: u32 = valloc(alloc, size);
        wr32(ebx.wrapping_add(HDR_COUNT), ebp as u32);
        wr32(ebx.wrapping_add(HDR_BUF), s3);
        // Signed: `(an instruction of the original); jle skip`.
        if ebp > 0 {
            let n: u32 = ebp as u32;
            let mut i: u32 = 0;
            while i < n {
                let v: u32 = rd32(src.wrapping_add(i.wrapping_mul(4)));
                wr32(s3.wrapping_add(i.wrapping_mul(4)), v);
                i = i.wrapping_add(1);
            }
        }
        (ebx.wrapping_add(HDR_FLAG) as *mut u8).write(a4 as u8);
        let old: u32 = (this.wrapping_add(THIS_USED) as *const u16).read_unaligned() as u32;
        (this.wrapping_add(THIS_USED) as *mut u16)
            .write_unaligned(old.wrapping_add(1) as u16);
        let base: u32 = rd32(this.wrapping_add(THIS_BASE));
        wr32(base.wrapping_add(old.wrapping_mul(4)), ebx);
        old.wrapping_add(1)
    }
});
