// original: 0x009e6080 ped_link_rebind (proposed)

/// Rebind or attach this ped's link slot, then clear it.
///
/// Sets bit 0x400 of `this + 0x24`. When bit 2 of `this + 0x26c` is set
/// and the link at `this + 0xb30` is nonzero, calls rebind (`thiscall`
/// on the link with the argument) if the link's back-pointer at +0xf50
/// is this object, else attach (`thiscall` on the link with this);
/// then clears bit 2, unlinks through the (possibly rewritten) link
/// unless it is null, and clears the slot. Returns the last call's
/// answer. The two early exits return entry `eax`, which a rewrite
/// cannot observe: the contract always sets the mode bit and the link
/// (noted in `narrowed`). `thiscall`, one stack word.
lf_checker_rt::export!(thiscall, rw_009e6080(this: u32, a1: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x24;
        const FLAG_BIT: u32 = 0x400;
        const MODE: u32 = 0x26c;
        const LINK: u32 = 0xb30;
        const BACK: u32 = 0xf50;
        const REBIND: u32 = 1;
        const ATTACH: u32 = 2;
        const UNLINK: u32 = 3;
        let f = ((this + FLAG) as *const u32).read_unaligned();
        ((this + FLAG) as *mut u32).write_unaligned(f | FLAG_BIT);
        // Unreachable by contract (see doc comment): return a placeholder.
        if (((this + MODE) as *const u8).read() & 4) == 0 {
            return 0;
        }
        let link = ((this + LINK) as *const u32).read_unaligned();
        if link == 0 {
            return 0;
        }
        let back = ((link + BACK) as *const u32).read_unaligned();
        let mut r = if back == this {
            lf_checker_rt::callee_thiscall!(REBIND, u32, link, a1)
        } else {
            lf_checker_rt::callee_thiscall!(ATTACH, u32, link, this)
        };
        let cur = ((this + LINK) as *const u32).read_unaligned();
        let m = ((this + MODE) as *const u32).read_unaligned();
        ((this + MODE) as *mut u32).write_unaligned(m & 0xfffffffb);
        if cur != 0 {
            r = lf_checker_rt::callee_thiscall!(UNLINK, u32, cur, this.wrapping_add(LINK));
        }
        ((this + LINK) as *mut u32).write_unaligned(0);
        r
    }
});
