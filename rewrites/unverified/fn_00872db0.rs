// original: 0x00872DB0 crmt_link_or_notify

/// Attach `arg0` to `this`: walk the +0x10 chain from `[this+0x1c]` for `arg0` (position 0 when missing or empty); when `arg0` is owned by `this`, bump its word refcount at +4, run the 0x8726d0 notification, run its slot-+4 hook unless `arg2`'s low byte is clear, drop the refcount and when it hits zero tear down through 0x874890 or the slot-+0 hook with argument 1; always finish through the 0x872e40 link with `(pos, arg2)`. Returns the link's answer.
///
/// Original: 0x00872DB0 (thiscall, three stack words (the middle one ignored)).
lf_checker_rt::export!(thiscall, rw_00872db0(this: u32, arg0: u32, _a1: u32, arg2: u32) -> u32 {
    const NOTIFY: u32 = 1;
    const TEARDOWN: u32 = 2;
    const LINK: u32 = 3;
    const V0_SLOT: usize = 0;
    const V4_SLOT: usize = 4;
    unsafe {
        let mut pos = 0u32;
        let mut p = ((this + 0x1c) as *const u32).read_unaligned();
        if p != 0 {
            loop {
                if p == arg0 { break; }
                p = ((p + 0x10) as *const u32).read_unaligned();
                pos += 1;
                if p == 0 { pos = 0; break; }
            }
        }
        let owner = ((arg0 + 0xc) as *const u32).read_unaligned();
        if owner == this {
            let r = ((arg0 + 4) as *const u16).read_unaligned();
            ((arg0 + 4) as *mut u16).write_unaligned(r.wrapping_add(1));
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, arg0);
            if arg2 & 0xff != 0 {
                let vt = (arg0 as *const u32).read_unaligned();
                let tgt = ((vt as *const u8).add(V4_SLOT) as *const u32).read_unaligned();
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let _: u32 = hook(arg0);
            }
            let r2 = ((arg0 + 4) as *const u16).read_unaligned();
            let r3 = r2.wrapping_add(0xffff);
            ((arg0 + 4) as *mut u16).write_unaligned(r3);
            if r3 == 0 {
                let t = ((arg0 + 0x18) as *const u32).read_unaligned();
                if t != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, t, arg0);
                } else {
                    let vt = (arg0 as *const u32).read_unaligned();
                    let tgt = ((vt as *const u8).add(V0_SLOT) as *const u32).read_unaligned();
                    let releaser: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    let _: u32 = releaser(arg0, 1);
                }
            }
        }
        lf_checker_rt::callee_thiscall!(LINK, u32, this, pos, arg2)
    }
});
