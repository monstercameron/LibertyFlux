// original: 0x005fbec0 atany_holder_float_assign

/// Assign a float value through a holder box, allocating on first use.
///
/// `this` points to a one-word box: the word at `+0` is null or points at the
/// live holder (vtable at `+0`, payload at `+0x4`). `src` points at the
/// value to store.
///
/// When the box already holds a holder, the value is copied into its payload
/// slot and the box is returned. When the box is empty, a fresh 8-byte holder
/// is requested from the thread allocator (reached through TLS slot 0:
/// allocator object at `[tls + 0x8]`, its virtual `alloc` at vtable `+0x8`,
/// called thiscall with `(size, 0x10, 0)`); on success its vtable is set and
/// the value copied in, on failure the box is left null. Either way the box
/// is returned. The trailing release of the previous holder never runs: the
/// previous holder is always null on the allocate path.
///
/// Original: 0x005FBEC0 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005fbec0(this: u32, src: u32) -> u32 {
    unsafe {
        const HELD_OFF: u32 = 0x0;
        const HOLDER_VTABLE: u32 = 0x00FE17E4;
        const HOLDER_PAYLOAD: u32 = 0x4;
        const ALLOC_SIZE: u32 = 8;
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
            wr32(held.wrapping_add(HOLDER_PAYLOAD), rd32(src));
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
            wr32(fresh.wrapping_add(HOLDER_PAYLOAD), rd32(src));
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
