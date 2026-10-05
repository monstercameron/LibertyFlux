// original: 0x005fc040 atany_holder_assign_w128_fe1764

/// Assign 16 bytes through a holder box, constructing on first use.
///
/// `this` points to a one-word box: the word at `+0` is null or points at the
/// live holder (payload at `+0x4`, 16 bytes). `src` points at 16 bytes. The
/// original moves both halves through vector registers; that is a plain bit
/// copy with no arithmetic.
///
/// When the box already holds a holder, the 16 bytes are copied into its
/// payload and the box is returned. When the box is empty, the original asks
/// its copy-construct helper to build a replacement holder in the caller's
/// own argument slot, swaps that slot with the box, and releases whatever
/// ends up in the slot (always null: the live holder was null on this path).
/// The helper itself allocates 0x14 bytes from the thread allocator, sets
/// the holder vtable and copies the 16 bytes. The rewrite performs the same
/// allocation and copy inline and the same swap-then-release against a local.
///
/// Original: 0x005FC040 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005fc040(this: u32, src: u32) -> u32 {
    unsafe {
        const HELD_OFF: u32 = 0x0;
        const HOLDER_VTABLE: u32 = 0x00FE1764;
        const HOLDER_PAYLOAD: u32 = 0x4;
        const ALLOC_SIZE: u32 = 0x14;
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
            wr32(held.wrapping_add(0x4), rd32(src));
            wr32(held.wrapping_add(0x8), rd32(src.wrapping_add(4)));
            wr32(held.wrapping_add(0xc), rd32(src.wrapping_add(8)));
            wr32(held.wrapping_add(0x10), rd32(src.wrapping_add(12)));
            return this;
        }
        let tls = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = rd32(tls.wrapping_add(TLS_ALLOC_OFF));
        let vtable = rd32(allocator);
        let do_alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_ALLOC_OFF)) as usize);
        let fresh = do_alloc(allocator, ALLOC_SIZE, ALLOC_FLAGS, ALLOC_ZERO);
        let mut slot = if fresh != 0 {
            wr32(fresh, lf_checker_rt::relocated(HOLDER_VTABLE));
            wr32(fresh.wrapping_add(0x4), rd32(src));
            wr32(fresh.wrapping_add(0x8), rd32(src.wrapping_add(4)));
            wr32(fresh.wrapping_add(0xc), rd32(src.wrapping_add(8)));
            wr32(fresh.wrapping_add(0x10), rd32(src.wrapping_add(12)));
            fresh
        } else {
            0
        };
        let old = rd32(this.wrapping_add(HELD_OFF));
        let incoming = slot;
        slot = old;
        wr32(this.wrapping_add(HELD_OFF), incoming);
        if slot != 0 {
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(slot) as usize);
            release(slot, 1);
        }
        this
    }

});
