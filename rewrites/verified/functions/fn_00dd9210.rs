// original: 0x00dd9210 UIBasicClip::vf144

/// Tail-jump to a slot of the part object's function table.
///
/// `this` is the clip object. The part object at `this + PART (+0x1e4)` is
/// loaded, its table pointer is loaded from its first word, and control
/// jumps to the entry at `SLOT (+0x124)` with the part object still in ECX
/// (a computed tail jump: the stack is untouched). The rewrite performs the
/// same load-and-call through the same fabricated table; both sides land on
/// the checker's recorder stub, whose answer in EAX is the result.
///
/// Original: thiscall, no stack arguments, tail jump, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dd9210(this: u32) -> u32 {
    const PART: u32 = 0x1e4;
    const SLOT: u32 = 0x124;
    unsafe {
        let part = ((this + PART) as *const u32).read_unaligned();
        let table = (part as *const u32).read_unaligned();
        let target = ((table + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(part)
    }
});
