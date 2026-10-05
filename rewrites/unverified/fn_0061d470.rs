// original: 0x0061D470 net_bitread_pair_00

/// Pull two words through the bit reader, reporting bits when asked.
///
/// Builds a six-word read frame and pulls two 32-bit words through the
/// bit reader: first the word at `this`, then the word at `this+4`.
/// `this` points at the two words, `buf` is a caller buffer recorded in
/// the frame, `unused` is popped but never loaded, `out` is an optional
/// out-pointer for the consumed bit count as bytes (`(BITS+7)>>3`,
/// arithmetic shift), or null to skip it. Returns 1 when the reader
/// accepts both pulls and at least one word is nonzero, 0 otherwise,
/// with the failing helper's accumulator in the upper 24 bits.
///
/// Frame layout (words): `BUF`, 0, 0x1C68, 0, `BITS`, 0. `BITS` is the
/// reader's out-word. The original also folds an uninitialized-stack
/// byte into a dead frame slot; the rewrite omits it (never read back).
/// Original: 0x0061D470 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061D470(this: u32, buf: u32, _unused: u32, out: u32) -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x019F089C;
        const READ_CALLEE: u32 = 1;
        const PULL_FIRST: u32 = 2;
        const PULL_SECOND: u32 = 3;
        const FRAME_SIZE: u32 = 0x1C68;
        const PULL_BITS: u32 = 0x20;
        const BITS_SLOT: usize = 4;
        let mut frame = [0u32; 6];
        frame[0] = buf;
        frame[2] = FRAME_SIZE;
        let base = frame.as_mut_ptr() as u32;
        let kind = lf_checker_rt::global::<u32>(GLOBAL).read_unaligned();
        let r1 = lf_checker_rt::callee_fastcall!(READ_CALLEE, u32, kind, base);
        // (last, ok): the accumulator to keep on failure, and whether all
        // stages passed. The zero-words check leaves 0 in the accumulator.
        let (mut last, mut ok) = (r1, r1 & 0xFF != 0);
        if ok {
            let first = (this as *const u32).read_unaligned();
            let second = ((this + 4) as *const u32).read_unaligned();
            let ra = lf_checker_rt::callee_thiscall!(PULL_FIRST, u32, base, first, PULL_BITS);
            last = ra;
            ok = ra & 0xFF != 0;
            if ok {
                let rb = lf_checker_rt::callee_thiscall!(PULL_SECOND, u32, base, second, PULL_BITS);
                last = rb;
                ok = rb & 0xFF != 0 && (first | second) != 0;
                if rb & 0xFF != 0 && (first | second) == 0 {
                    last = 0;
                }
            }
        }
        if !ok {
            if out != 0 {
                (out as *mut u32).write_unaligned(0);
                return 0;
            }
            return last & 0xFFFF_FF00;
        }
        if out == 0 {
            return (last & 0xFFFF_FF00) | 1;
        }
        let bytes = ((frame[BITS_SLOT].wrapping_add(7)) as i32 >> 3) as u32;
        (out as *mut u32).write_unaligned(bytes);
        (bytes & 0xFFFF_FF00) | 1
    }
});
