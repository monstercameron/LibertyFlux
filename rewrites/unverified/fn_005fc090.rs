// original: 0x005fc090 atany_holder_assign_big_fe17f4

/// Assign a twelve-word structure through a holder box.
///
/// `this` points to a one-word box: the word at `+0` is null or points at the
/// live holder (payload at `+0x10`, twelve words with gaps at `+0x1c`,
/// `+0x2c` and `+0x3c`). `src` points at the source structure, read from the
/// same offsets it is written to (`+0x0` to `+0x38` skipping the gaps).
///
/// When the box already holds a holder, the twelve words are copied into its
/// payload and the box is returned. When the box is empty, the original
/// allocates a 0x50-byte holder from the thread allocator and asks its
/// construct helper to set the holder vtable and copy the twelve words; the
/// rewrite performs the same allocation and copy inline. Either way the box
/// is returned. The trailing release of the previous holder never runs: it
/// is always null on the allocate path.
///
/// Original: 0x005FC090 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005fc090(this: u32, src: u32) -> u32 {
    unsafe {
        const HELD_OFF: u32 = 0x0;
        const HOLDER_VTABLE: u32 = 0x00FE17F4;
        const ALLOC_SIZE: u32 = 0x50;
        const TLS_SLOT: usize = 0;
        const TLS_ALLOC_OFF: u32 = 0x8;
        const VTABLE_ALLOC_OFF: u32 = 0x8;
        const ALLOC_FLAGS: u32 = 0x10;
        const ALLOC_ZERO: u32 = 0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let held = rd32(this.wrapping_add(HELD_OFF));
        if held != 0 {
            wr32(held.wrapping_add(0x10), rd32(src.wrapping_add(0x0)));
            wr32(held.wrapping_add(0x14), rd32(src.wrapping_add(0x4)));
            wr32(held.wrapping_add(0x18), rd32(src.wrapping_add(0x8)));
            wr32(held.wrapping_add(0x20), rd32(src.wrapping_add(0x10)));
            wr32(held.wrapping_add(0x24), rd32(src.wrapping_add(0x14)));
            wr32(held.wrapping_add(0x28), rd32(src.wrapping_add(0x18)));
            wr32(held.wrapping_add(0x30), rd32(src.wrapping_add(0x20)));
            wr32(held.wrapping_add(0x34), rd32(src.wrapping_add(0x24)));
            wr32(held.wrapping_add(0x38), rd32(src.wrapping_add(0x28)));
            wr32(held.wrapping_add(0x40), rd32(src.wrapping_add(0x30)));
            wr32(held.wrapping_add(0x44), rd32(src.wrapping_add(0x34)));
            wr32(held.wrapping_add(0x48), rd32(src.wrapping_add(0x38)));
            return this;
        }
        let tls = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = rd32(tls.wrapping_add(TLS_ALLOC_OFF));
        let vtable = rd32(allocator);
        let do_alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_ALLOC_OFF)) as usize);
        let fresh = do_alloc(allocator, ALLOC_SIZE, ALLOC_FLAGS, ALLOC_ZERO);
        let stored = if fresh != 0 {
            wr32(fresh, lf_checker_rt::relocated(HOLDER_VTABLE));
            wr32(fresh.wrapping_add(0x10), rd32(src.wrapping_add(0x0)));
            wr32(fresh.wrapping_add(0x14), rd32(src.wrapping_add(0x4)));
            wr32(fresh.wrapping_add(0x18), rd32(src.wrapping_add(0x8)));
            wr32(fresh.wrapping_add(0x20), rd32(src.wrapping_add(0x10)));
            wr32(fresh.wrapping_add(0x24), rd32(src.wrapping_add(0x14)));
            wr32(fresh.wrapping_add(0x28), rd32(src.wrapping_add(0x18)));
            wr32(fresh.wrapping_add(0x30), rd32(src.wrapping_add(0x20)));
            wr32(fresh.wrapping_add(0x34), rd32(src.wrapping_add(0x24)));
            wr32(fresh.wrapping_add(0x38), rd32(src.wrapping_add(0x28)));
            wr32(fresh.wrapping_add(0x40), rd32(src.wrapping_add(0x30)));
            wr32(fresh.wrapping_add(0x44), rd32(src.wrapping_add(0x34)));
            wr32(fresh.wrapping_add(0x48), rd32(src.wrapping_add(0x38)));
            fresh
        } else {
            0
        };
        let old = rd32(this.wrapping_add(HELD_OFF));
        wr32(this.wrapping_add(HELD_OFF), stored);
        if old != 0 {
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(old) as usize);
            release(old, 1);
        }
        this
    }

});
