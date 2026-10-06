// original: 0x009A6D10 audio_forward_positional (proposed)

/// Forwards a positional emit, resolving the position from a handle object.
///
/// thiscall, four stack words (`a0`, `a1`, `a2`, `a3`). Builds an
/// 18-word frame through the frame constructor (callee 1), resolves three
/// position words from `a2`: when the dword at `LINK` (+0x20) of `a2` is
/// null the words come from `a2 + DIRECT` (+0x10), otherwise from
/// `link + LINKED` (+0x30). The words are copied verbatim (plain 32-bit
/// moves, no floating-point arithmetic, so bit-exact by construction).
/// Then takes a tag from callee 2 (cdecl, no arguments) into frame word
/// `TAG_SLOT` (+0x0C), stores the address of the position words at
/// `POS_SLOT` (+0x14) and `a3` at `A3_SLOT` (+0x20), and calls the emitter
/// (callee 3, thiscall on `this`) with (`a0`, `a1`, frame address).
/// Returns the emitter's answer. The frame address differs between the
/// two sides, so the contract skips that call argument and snapshots
/// frame words 2-17 (minus the interior pointer word, whose pointed-to
/// position words are snapshotted separately below the frame).
lf_checker_rt::export!(thiscall, rw_009a6d10(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const CTOR: u32 = 1;
        const TAGGER: u32 = 2;
        const EMIT: u32 = 3;
        const LINK: u32 = 0x20;
        const DIRECT: u32 = 0x10;
        const LINKED: u32 = 0x30;
        const TAG_SLOT: usize = 3;
        const POS_SLOT: usize = 5;
        const A3_SLOT: usize = 8;
        // Words 0-2 hold the position triple (frame - 0x18, as in the
        // original's aligned buffer); words 6-23 are the 18-word frame.
        let mut buf = [0u32; 24];
        let frame = buf.as_mut_ptr().add(6) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, frame);
        let link = (a2.wrapping_add(LINK) as *const u32).read_unaligned();
        let src = if link == 0 {
            a2.wrapping_add(DIRECT)
        } else {
            link.wrapping_add(LINKED)
        };
        buf[0] = (src as *const u32).read_unaligned();
        buf[1] = (src.wrapping_add(4) as *const u32).read_unaligned();
        buf[2] = (src.wrapping_add(8) as *const u32).read_unaligned();
        let tag: u32 = lf_checker_rt::callee_cdecl!(TAGGER, u32,);
        (frame as *mut u32).add(TAG_SLOT).write_unaligned(tag);
        (frame as *mut u32).add(POS_SLOT).write_unaligned(buf.as_mut_ptr() as u32);
        (frame as *mut u32).add(A3_SLOT).write_unaligned(a3);
        lf_checker_rt::callee_thiscall!(EMIT, u32, this, a0, a1, frame)
    }
});
