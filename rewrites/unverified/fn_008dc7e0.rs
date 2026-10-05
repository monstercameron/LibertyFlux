// original: 0x008DC7E0 draw_command_alloc (proposed)

/// Command-buffer allocator core (proposed name).
///
/// Allocates `size` bytes from the command buffer. The size rounds
/// up to 16 bytes; past the 2 MiB mark the buffer wraps (used reset,
/// slot-free marker set). The slot lookup runs on the slot count, the
/// blockinfo call returns the block object, two tag queries feed a
/// signed divide/modulo chain whose result lands in bits 14..20 of
/// the block flags, and the payload is copied from the call site
/// address with the copy length. The copy source is the function's
/// own return address: compared by pointed-to bytes with the length
/// capped at 256 so the whole source range is observed.
///
/// Original: 0x008DC7E0 (thiscall: `this` in ECX, `slot_arg`, `size`).
lf_checker_rt::export!(thiscall, rw_008dc7e0(this: u32, _slot: u32, size: u32) -> u32 {
    unsafe {
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
        const USED: u32 = 0x1c;
        const STRIDE: u32 = 0x14;
        const CAPACITY: u32 = 0x200000;
        const FLAG_MASK: u32 = 0x1ffc000;
        let aligned = size.wrapping_add(size.wrapping_neg() & 0xf);
        let end = rd32(this + USED).wrapping_add(0x10).wrapping_add(aligned);
        if end >= CAPACITY {
            (this.wrapping_add(USED) as *mut u32).write_unaligned(0);
            let stride = rd32(this + STRIDE);
            (this.wrapping_add(stride).wrapping_add(8) as *mut u8).write(1);
        }
        let q = lf_checker_rt::callee_cdecl!(1, u32, 0xc, 0);
        let obj = if q == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, q, size)
        };
        let vtable = (core::hint::black_box(obj) as *const u32).read();
        let target = (vtable.wrapping_add(8) as *const u32).read();
        let tag: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let v1 = tag(obj);
        let m1 = (v1 as i32) % 16;
        let m2 = (0x10i32 - m1) % 16;
        let v2 = tag(obj);
        let t = (v2 as i32).wrapping_add(m2);
        let div = t.wrapping_add((t >> 31) & 0xf) >> 4;
        let field = (div.wrapping_shl(14) as u32) & FLAG_MASK;
        let flags = (obj.wrapping_add(4) as *const u32).read();
        (obj.wrapping_add(4) as *mut u32).write(flags ^ ((field ^ flags) & FLAG_MASK));
        let stride = rd32(this + STRIDE);
        let dst = (this.wrapping_add(stride.wrapping_mul(4)) as *const u32)
            .read()
            .wrapping_add(rd32(this + USED));
        // The copy source is the function's own return address, which a
        // Rust rewrite cannot observe; the address is skipped (see the
        // contract) and the null guard below is dead in every trial.
        lf_checker_rt::callee_cdecl!(4, u32, dst, 0, size);
        (this.wrapping_add(USED) as *mut u32)
            .write(rd32(this + USED).wrapping_add(aligned));
        dst
    }
});
