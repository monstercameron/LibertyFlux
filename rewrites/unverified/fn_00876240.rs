// original: 0x00876240 rage::crmtRequestAddSubtract::vf2

/// Rebuild a combiner request's target: when both words at +0x14/+0x18 are
/// zero return the second argument unchanged; otherwise resolve the target
/// object through the 0x87b1e0 lookup on the first argument, feed this
/// object's f32 value at +0x1c to the target through vtable slot +0x38 (the
/// float travels in XMM0; the original also spills it over its own saved
/// register slot, which the callee consumes as its stack word, so the
/// function returns with EDI holding the float bits), release the
/// child at +0x20 through its slot-+4 hook when non-null, release the old
/// occupant of the target's +0x28 slot through its slot-+8 hook when
/// non-null, install the child there, and run the 0x8762b0 link on
/// `(this, arg0, arg1, target)`. Returns the target.
///
/// Original: 0x00876240 (thiscall, two stack words; the callee pops 8 bytes).
/// The feed call uses the checker's XMM0-from-stack transport; only XMM0
/// is compared for that call.
lf_checker_rt::export!(thiscall, rw_00876240(this: u32, a0: u32, a1: u32) -> u32 {
    const LOOKUP: u32 = 1;
    const FEED_SLOT: usize = 0x38;
    const REL4_SLOT: usize = 4;
    const REL8_SLOT: usize = 8;
    const LINK: u32 = 5;
    unsafe {
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        if w14 == 0 && w18 == 0 {
            return a1;
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, a0);
        let fbits = ((this + 0x1c) as *const u32).read_unaligned();
        let child = ((this + 0x20) as *const u32).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let tgt = ((vt as *const u8).add(FEED_SLOT) as *const u32).read_unaligned();
        let feed: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let _: u32 = feed(obj, fbits);
        if child != 0 {
            let cvt = (child as *const u32).read_unaligned();
            let ctgt = ((cvt as *const u8).add(REL4_SLOT) as *const u32).read_unaligned();
            let rel: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ctgt as usize);
            let _: u32 = rel(child);
        }
        let slot = ((obj + 0x28) as *const u32).read_unaligned();
        if slot != 0 {
            let svt = (slot as *const u32).read_unaligned();
            let stgt = ((svt as *const u8).add(REL8_SLOT) as *const u32).read_unaligned();
            let rel2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(stgt as usize);
            let _: u32 = rel2(slot);
        }
        ((obj + 0x28) as *mut u32).write_unaligned(child);
        let _: u32 = lf_checker_rt::callee_thiscall!(LINK, u32, this, a0, a1, obj);
        obj
    }
});
