// original: 0x0088F540 rage::audVoicePcAdpcm::vf11

/// Refresh this ADPCM voice's child mode, relay its level, then tail into
/// the restart entry.
///
/// `this` points to the voice. When synth bit 4 of the flags at `+0x8c`
/// is set, the child at `+0x140` is polled (callee 1); its answer,
/// reduced to 0/1 at the 0x10000 boundary, is compared with the word at
/// `+0x110`, and on a mismatch the mode is refreshed: directly through
/// the start entry (callee 2) when the byte at `+0x14c` is clear or the
/// counter at `+0x114` is still positive (which then decrements),
/// otherwise through the voice's own slot `+0x14` (callee 3). The level
/// word through `+0x4` is then relayed (callee 4) and control passes to
/// the restart entry with the voice as `this`; its answer is the answer
/// of this function. Same shape as the software voice's refresh entry at
/// shifted offsets.
///
/// Original: 0x0088F540 (thiscall, no stack arguments, tail call).
lf_checker_rt::export!(thiscall, rw_0088F540(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x8c;
        const SYNTH: u8 = 0x10;
        const CHILD: u32 = 0x140;
        const MODE_CMP: u32 = 0x110;
        const MODE_BYTE: u32 = 0x14c;
        const MODE_COUNT: u32 = 0x114;
        const LEVEL_PTR: u32 = 0x04;
        const OWN_SLOT: u32 = 0x14;
        const POLL_BOUND: u32 = 0x10000;
        const POLL_CHILD: u32 = 1;
        const START_CHILD: u32 = 2;
        const LEVEL_ENTRY: u32 = 4;
        const RESTART: u32 = 5;

        if ((this + FLAGS) as *const u8).read() & SYNTH != 0 {
            let child = ((this + CHILD) as *const u32).read_unaligned();
            let ans = lf_checker_rt::callee_thiscall!(POLL_CHILD, u32, child);
            let bit = (ans >= POLL_BOUND) as u32;
            let want = ((this + MODE_CMP) as *const u32).read_unaligned();
            if bit != want {
                let b = ((this + MODE_BYTE) as *const u8).read();
                let n = ((this + MODE_COUNT) as *const u32).read_unaligned();
                if b == 0 || n > 0 {
                    lf_checker_rt::callee_thiscall!(START_CHILD, u32, this, 0);
                    if b != 0 {
                        ((this + MODE_COUNT) as *mut u32)
                            .write_unaligned(n.wrapping_sub(1));
                    }
                } else {
                    let table = (this as *const u32).read_unaligned();
                    let slot: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(
                            ((table + OWN_SLOT) as *const u32).read_unaligned()
                                as usize,
                        );
                    slot(this);
                }
            }
        }
        let lp = ((this + LEVEL_PTR) as *const u32).read_unaligned();
        let w = (lp as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(LEVEL_ENTRY, u32, this, w);
        lf_checker_rt::callee_thiscall!(RESTART, u32, this)
    }
});
