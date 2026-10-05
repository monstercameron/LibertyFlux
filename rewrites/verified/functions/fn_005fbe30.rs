// original: 0x005fbe30 atany_holder_vector3_assign

/// Assign a four-word vector through a holder box, allocating on first use.
///
/// `this` points to a one-word box: the word at `+0` is null or points at the
/// live holder (payload at `+0x10`). `src` points at 16 bytes to store. The
/// payload layout matches the `Holder<Vector3>` clone in this batch (same
/// vtable, same 0x20-byte holder).
///
/// When the box already holds a holder, all four words are copied into its
/// payload (`+0x10` to `+0x1c`; the original moves the middle two through
/// vector registers, which is a plain bit copy) and the box is returned.
/// When the box is empty, a fresh 0x20-byte holder is requested from the
/// thread allocator (TLS slot 0, virtual `alloc` at vtable `+0x8`, thiscall
/// `(size, 0x10, 0)`); on success its vtable is set and only the first three
/// words are copied (`+0x10` to `+0x18`; `+0x1c` is left as allocated), on
/// failure the box is left null. Either way the box is returned. The trailing
/// release of the previous holder never runs: it is always null here.
///
/// Original: 0x005FBE30 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005fbe30(this: u32, src: u32) -> u32 {
    unsafe {
        const HELD_OFF: u32 = 0x0;
        const HOLDER_VTABLE: u32 = 0x00FE1784;
        const HOLDER_PAYLOAD: u32 = 0x10;
        const ALLOC_SIZE: u32 = 0x20;
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
            wr32(held.wrapping_add(0x10), rd32(src));
            wr32(held.wrapping_add(0x14), rd32(src.wrapping_add(4)));
            wr32(held.wrapping_add(0x18), rd32(src.wrapping_add(8)));
            wr32(held.wrapping_add(0x1c), rd32(src.wrapping_add(12)));
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
            wr32(fresh.wrapping_add(0x10), rd32(src));
            wr32(fresh.wrapping_add(0x14), rd32(src.wrapping_add(4)));
            wr32(fresh.wrapping_add(0x18), rd32(src.wrapping_add(8)));
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
