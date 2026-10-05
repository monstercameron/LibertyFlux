// original: 0x005fbdc0 fragref_detach_notify

/// Detach a streamable reference from its item and notify the registry.
///
/// `this` points at the reference (link words at `+0x4`/`+0x8`, back-pointer
/// at `+0xc`).
///
/// The reference is unlinked from its doubly-linked list: a set forward link
/// has its back link repointed at this reference's back link, and a set back
/// link has its forward link repointed at this reference's forward link. When
/// the owner still points back at this reference, the owner takes this
/// reference's forward link as its new head, and when that leaves no head the
/// owner's streaming bit (0x80000000 of its flag word at `+0x8`) is cleared.
/// Both link words are then cleared and the registry (global object) is told
/// through its virtual slot at `+0x8`, thiscall with
/// `(this, forward-fn, init-fn, owner)`. Returns the notify call's result.
///
/// Original: 0x005FBDC0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_005fbdc0(this: u32) -> u32 {
    unsafe {
        const FWD_LINK: u32 = 0x4;
        const BACK_LINK: u32 = 0x8;
        const OWNER_OFF: u32 = 0xc;
        const FLAG_OFF: u32 = 0x8;
        const STREAMING_BIT: u32 = 0x80000000;
        const REGISTRY_GLOBAL: u32 = 0x018B7A60;
        const VTABLE_NOTIFY_OFF: u32 = 0x8;
        const FN_FORWARD: u32 = 0x005FC3C0;
        const FN_INIT: u32 = 0x005FBCB0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let fwd = rd32(this.wrapping_add(FWD_LINK));
        if fwd != 0 {
            wr32(fwd.wrapping_add(BACK_LINK), rd32(this.wrapping_add(BACK_LINK)));
        }
        let back = rd32(this.wrapping_add(BACK_LINK));
        if back != 0 {
            wr32(back.wrapping_add(FWD_LINK), rd32(this.wrapping_add(FWD_LINK)));
        }
        let owner = rd32(this.wrapping_add(OWNER_OFF));
        if rd32(owner.wrapping_add(OWNER_OFF)) == this {
            let head = rd32(this.wrapping_add(FWD_LINK));
            wr32(owner.wrapping_add(OWNER_OFF), head);
            if head == 0 {
                wr32(owner.wrapping_add(FLAG_OFF),
                     rd32(owner.wrapping_add(FLAG_OFF)) & !STREAMING_BIT);
            }
        }
        wr32(this.wrapping_add(FWD_LINK), 0);
        wr32(this.wrapping_add(BACK_LINK), 0);
        let registry = rd32(lf_checker_rt::relocated(REGISTRY_GLOBAL));
        let vtable = rd32(registry);
        let notify: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VTABLE_NOTIFY_OFF)) as usize);
        notify(registry, this, lf_checker_rt::relocated(FN_FORWARD),
               lf_checker_rt::relocated(FN_INIT), owner)
    }

});
