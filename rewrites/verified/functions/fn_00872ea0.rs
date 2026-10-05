// original: 0x00872EA0 crmt_attach_or_link

/// Attach `arg0` to `this` or link through the existing owner: bump `arg0`'s word refcount; when `arg0` is owned by `this`, notify through 0x8726d0, drop the refcount and on zero tear down through 0x874890 or the slot-+0 hook; when `this` has an owner link at +0xc run the 0x872db0 link on it, drop `arg0`'s refcount and on zero tear down the same way, returning the teardown or link answer; otherwise splice `arg0` into the +0x8 chain head, clear its owner, run `this`'s slot-+4 hook, drop `this`'s refcount and on zero tear `this` down the same way.
///
/// Original: 0x00872EA0 (thiscall, two stack words (the second ignored)).
lf_checker_rt::export!(thiscall, rw_00872ea0(this: u32, arg0: u32, _a1: u32) -> u32 {
    const NOTIFY: u32 = 1;
    const TEARDOWN: u32 = 2;
    const LINK: u32 = 3;
    const V0_SLOT: usize = 0;
    const V4_SLOT: usize = 4;
    unsafe {
        let r0 = ((arg0 + 4) as *const u16).read_unaligned();
        ((arg0 + 4) as *mut u16).write_unaligned(r0.wrapping_add(1));
        let owner = ((arg0 + 0xc) as *const u32).read_unaligned();
        if owner == this {
            let r1 = ((arg0 + 4) as *const u16).read_unaligned();
            ((arg0 + 4) as *mut u16).write_unaligned(r1.wrapping_add(1));
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, arg0);
            let r2 = ((arg0 + 4) as *const u16).read_unaligned().wrapping_add(0xffff);
            ((arg0 + 4) as *mut u16).write_unaligned(r2);
            if r2 == 0 {
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
        let gate = ((this + 0xc) as *const u32).read_unaligned();
        if gate != 0 {
            let r3: u32 = lf_checker_rt::callee_thiscall!(LINK, u32, gate, this, arg0, 1);
            let r4 = ((arg0 + 4) as *const u16).read_unaligned().wrapping_add(0xffff);
            ((arg0 + 4) as *mut u16).write_unaligned(r4);
            if r4 == 0 {
                let t = ((arg0 + 0x18) as *const u32).read_unaligned();
                if t != 0 {
                    lf_checker_rt::callee_thiscall!(TEARDOWN, u32, t, arg0)
                } else {
                    let vt = (arg0 as *const u32).read_unaligned();
                    let tgt = ((vt as *const u8).add(V0_SLOT) as *const u32).read_unaligned();
                    let releaser: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    releaser(arg0, 1)
                }
            } else {
                r3
            }
        } else {
            let linktgt = ((this + 8) as *const u32).read_unaligned();
            ((linktgt + 8) as *mut u32).write_unaligned(arg0);
            ((arg0 + 0xc) as *mut u32).write_unaligned(0);
            let vt = (this as *const u32).read_unaligned();
            let tgt = ((vt as *const u8).add(V4_SLOT) as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            let r5: u32 = hook(this);
            let r6 = ((this + 4) as *const u16).read_unaligned().wrapping_add(0xffff);
            ((this + 4) as *mut u16).write_unaligned(r6);
            if r6 == 0 {
                let t = ((this + 0x18) as *const u32).read_unaligned();
                if t != 0 {
                    lf_checker_rt::callee_thiscall!(TEARDOWN, u32, t, this)
                } else {
                    let vt = (this as *const u32).read_unaligned();
                    let tgt = ((vt as *const u8).add(V0_SLOT) as *const u32).read_unaligned();
                    let releaser: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    releaser(this, 1)
                }
            } else {
                r5
            }
        }
    }
});
