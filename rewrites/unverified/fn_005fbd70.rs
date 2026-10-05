// original: 0x005fbd70 fragref_attach_notify

/// Attach a streamable reference to an item and notify the registry.
///
/// `this` points at the reference (link words at `+0x4`/`+0x8`, back-pointer
/// at `+0xc`). `item` points at the item (`+0x8` flag word, `+0xc` head of
/// its reference list).
///
/// A previous head of the item's list, when present, has its back link
/// repointed at this reference; this reference takes the item's list head as
/// its own forward link, clears its back link, marks the item streaming
/// (sets bit 0x80000000 of its flag word) and becomes the new head, with its
/// back-pointer set to the item. The registry (global object) is then told
/// through its virtual slot at `+0x4`, thiscall with `(item, 0.5f, init-fn,
/// forward-fn, this, 0)`, where the two function values are the module's own
/// entry points. Returns the notify call's result.
///
/// Original: 0x005FBD70 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005fbd70(this: u32, item: u32) -> u32 {
    unsafe {
        const FWD_LINK: u32 = 0x4;
        const BACK_LINK: u32 = 0x8;
        const OWNER_OFF: u32 = 0xc;
        const FLAG_OFF: u32 = 0x8;
        const STREAMING_BIT: u32 = 0x80000000;
        const REGISTRY_GLOBAL: u32 = 0x018B7A60;
        const VTABLE_NOTIFY_OFF: u32 = 0x4;
        const FN_FORWARD: u32 = 0x005FC3C0;
        const FN_INIT: u32 = 0x005FBCB0;
        const HALF_BITS: u32 = 0x3F000000;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let head = rd32(item.wrapping_add(OWNER_OFF));
        if head != 0 {
            wr32(head.wrapping_add(BACK_LINK), this);
        }
        let next = rd32(item.wrapping_add(OWNER_OFF));
        wr32(this.wrapping_add(FWD_LINK), next);
        wr32(this.wrapping_add(BACK_LINK), 0);
        wr32(item.wrapping_add(FLAG_OFF),
             rd32(item.wrapping_add(FLAG_OFF)) | STREAMING_BIT);
        wr32(item.wrapping_add(OWNER_OFF), this);
        wr32(this.wrapping_add(OWNER_OFF), item);
        let registry = rd32(lf_checker_rt::relocated(REGISTRY_GLOBAL));
        let vtable = rd32(registry);
        let notify: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_NOTIFY_OFF)) as usize);
        notify(registry, item, HALF_BITS, lf_checker_rt::relocated(FN_INIT),
               lf_checker_rt::relocated(FN_FORWARD), this, 0)
    }

});
