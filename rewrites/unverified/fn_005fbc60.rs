// original: 0x005fbc60 rage::pgStreamableRef<rage::fragType>::~fragType>

/// Destroy a streamable reference to a fragment type.
///
/// `this` points at the reference (`+0x4`/`+0x8` link words, `+0xc` pointer to
/// the referenced item or null). `flags` selects the destroying form: bit 0
/// requests the storage be released after destruction.
///
/// The reference first takes the intermediate vtable, then detaches from its
/// item unless the item is null, already detached (both link words zero) or
/// owned elsewhere (the item's back-pointer at `+0xc` is not this reference).
/// It then takes the final vtable and, when bit 0 of `flags` is set, releases
/// its storage through the thread allocator's virtual `free` at vtable
/// `+0xc` (TLS slot 0, thiscall with `this`). Returns `this`.
///
/// Original: 0x005FBC60 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005fbc60(this: u32, flags: u32) -> u32 {
    unsafe {
        const LINK_A: u32 = 0x4;
        const LINK_B: u32 = 0x8;
        const ITEM_OFF: u32 = 0xc;
        const BACK_PTR: u32 = 0xc;
        const VTABLE_MID: u32 = 0x00FE17D4;
        const VTABLE_FINAL: u32 = 0x00FE1794;
        const VTABLE_FREE_OFF: u32 = 0xc;
        const TLS_SLOT: usize = 0;
        const TLS_ALLOC_OFF: u32 = 0x8;
        const DETACH_CALLEE: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let item = rd32(this.wrapping_add(ITEM_OFF));
        wr32(this, lf_checker_rt::relocated(VTABLE_MID));
        if item != 0
            && (rd32(this.wrapping_add(LINK_A)) != 0
                || rd32(this.wrapping_add(LINK_B)) != 0
                || rd32(item.wrapping_add(BACK_PTR)) == this)
        {
            lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, this);
        }
        wr32(this, lf_checker_rt::relocated(VTABLE_FINAL));
        if flags & 1 != 0 {
            let tls = lf_checker_rt::tls_slot(TLS_SLOT);
            let allocator = rd32(tls.wrapping_add(TLS_ALLOC_OFF));
            let vtable = rd32(allocator);
            let do_free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_FREE_OFF)) as usize);
            do_free(allocator, this);
        }
        this
    }

});
