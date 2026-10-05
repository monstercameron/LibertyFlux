// original: 0x009441d0 streaming_slot_advance (proposed)

/// Advance the streaming slot cursor and publish the current value.
///
/// The cursor byte at `this + 0x1919` selects the slot
/// `(cursor + 4) % 5` of the table at `this + 0x17d4`; when that slot
/// already holds the global current value the call ends. Otherwise the
/// global value is written to the slot indexed by the cursor itself, the
/// cursor is advanced to `(cursor + 1) % 5`, and, when the global selector
/// byte reads below 5, the selector-indexed global base plus the stack
/// argument is published to the matching slot of the table at `+0x17e8`.
/// Returns the published sum, the probed slot number, or the selector
/// depending on the path taken.
///
/// Original: 0x009441d0 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_009441d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CURSOR: u32 = 0x1919;
        const SLOTS: u32 = 0x17D4;
        const SHADOW: u32 = 0x17E8;
        const CURRENT: u32 = 0x011D7620;
        const SELECTOR: u32 = 0x01037605;
        const BASES: u32 = 0x00E889A0;
        const WAYS: u32 = 5;
        let cursor = ((this + CURSOR) as *const u8).read() as u32;
        let probe = (cursor + 4) % WAYS;
        let current = lf_checker_rt::global::<u32>(CURRENT).read();
        if ((this + probe * 4 + SLOTS) as *const u32).read_unaligned() == current {
            return probe;
        }
        ((this + cursor * 4 + SLOTS) as *mut u32).write_unaligned(current);
        ((this + CURSOR) as *mut u8).write(((cursor + 1) % WAYS) as u8);
        let sel = lf_checker_rt::global::<u8>(SELECTOR).read() as u32;
        if sel >= WAYS {
            // The original overwrites the low byte of the small quotient
            // with the selector, which is the whole value back.
            return sel;
        }
        let bases = lf_checker_rt::relocated(BASES);
        let sum = ((bases + sel * 4) as *const u32)
            .read_unaligned()
            .wrapping_add(arg);
        ((this + sel * 4 + SHADOW) as *mut u32).write_unaligned(sum);
        sum
    }
});
