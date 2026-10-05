// original: 0x005fc1a0 atany_holder_vector3_clone

/// Clone a holder's Vector3 payload into a freshly allocated holder.
///
/// `this` points at the source holder (vtable at `+0`, payload words listed
/// below). A new holder of 32 bytes is requested from the thread allocator
/// (TLS slot 0, virtual `alloc` at vtable `+0x8`, thiscall `(size, 0x10, 0)`);
/// on success its vtable is set, the payload copied from the same offsets of
/// the source, and the new holder returned; on failure null is returned. This
/// is virtual slot 2 (clone) of the holder class.
///
/// Original: 0x005FC1A0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_005fc1a0(this: u32) -> u32 {
    unsafe {
        const HOLDER_VTABLE: u32 = 0x00FE1784;
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
        let tls = lf_checker_rt::tls_slot(TLS_SLOT);
        let allocator = rd32(tls.wrapping_add(TLS_ALLOC_OFF));
        let vtable = rd32(allocator);
        let do_alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_ALLOC_OFF)) as usize);
        let fresh = do_alloc(allocator, ALLOC_SIZE, ALLOC_FLAGS, ALLOC_ZERO);
        if fresh != 0 {
            wr32(fresh, lf_checker_rt::relocated(HOLDER_VTABLE));
            wr32(fresh.wrapping_add(0x10), rd32(this.wrapping_add(0x10)));
            wr32(fresh.wrapping_add(0x14), rd32(this.wrapping_add(0x14)));
            wr32(fresh.wrapping_add(0x18), rd32(this.wrapping_add(0x18)));
        }
        fresh
    }

});
