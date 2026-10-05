// original: 0x00d54870 CCamDebug::vf1

/// Fetch two debug targets, stamp their constant blocks, and notify the first.
///
/// `this` points to the object. The peer at `PEER` is fetched twice with tags
/// `TAG_A` and `TAG_B` (each taking (`tag`, 0, `this`)); the first answer
/// receives three constant words, the incoming stack residue word (zero under
/// the contract's stack fill) and a flag-bit update, the second answer gets
/// the same flag-bit update, and slot `VSLOT` of the first answer's table is
/// then invoked with the first answer in ECX. Returns 1 in AL (upper EAX
/// passes through, so only AL is compared).
///
/// Original: 0x00d54870 (thiscall, no stack arguments, three calls).
lf_checker_rt::export!(thiscall, rw_00d54870(this: u32) -> u32 {
    unsafe {
        /// Slot holding the peer pointer.
        const PEER: u32 = 0x114;
        /// First fetch tag.
        const TAG_A: u32 = 0x26;
        /// Second fetch tag.
        const TAG_B: u32 = 0x28;
        /// Flag byte updated on both answers.
        const FLAG: u32 = 0x13c;
        /// Virtual slot invoked on the first answer.
        const VSLOT: u32 = 4;
        /// Peer fetch, first call (intercepted; thiscall, three stack words).
        const FETCH_A: u32 = 1;
        /// Peer fetch, second call (intercepted; thiscall, three stack words).
        const FETCH_B: u32 = 2;
        let peer = ((this + PEER) as *const u32).read_unaligned();
        let first = lf_checker_rt::callee_thiscall!(FETCH_A, u32, peer, TAG_A, 0, this);
        ((first + 0x40) as *mut u32).write_unaligned(0x4518a000);
        ((first + 0x44) as *mut u32).write_unaligned(0xc4cf8000);
        ((first + 0x48) as *mut u32).write_unaligned(0x41680000);
        ((first + 0x4c) as *mut u32).write_unaligned(0);
        let f = ((first + FLAG) as *mut u8).read();
        ((first + FLAG) as *mut u8).write((f & 0xfb) | 8);
        let second = lf_checker_rt::callee_thiscall!(FETCH_B, u32, peer, TAG_B, 0, this);
        let g = ((second + FLAG) as *mut u8).read();
        ((second + FLAG) as *mut u8).write((g & 0xfb) | 8);
        let vt = (first as *const u32).read_unaligned();
        let tgt = (vt.wrapping_add(VSLOT) as *const u32).read_unaligned();
        let notify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt as usize);
        notify(first);
        1
    }
});
