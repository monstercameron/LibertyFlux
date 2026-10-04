// original: 0x00E3E7F0 emit_state_setup_calls (proposed)

/// Run a fixed sequence of state-setup calls with small constant arguments.
///
/// Takes no arguments and reads three global bytes/words that gate the
/// sequence: a two-byte mode at `0x116c250`/`0x116c253`, an enable byte at
/// `0x1161548`, and a selector dword at `0x11d6fd4`.
///
/// Behaviour: after two priming calls (the first fills a scratch word that
/// is never read again), the mode bytes pick a constant (`7` unless the
/// first byte is `0x6a` or the second is nonzero, else `2`) for a third
/// call. Two more callees share one two-word out-pointer (the first one's
/// word is overwritten before any read); those two floats are handed
/// (twice) to a pair-taking callee, then the constant pair
/// `(-5.0, 5.0)` goes to another. A flag byte starts at `0xff` and, when
/// the enable byte is set, becomes the low byte of a registry callee's
/// answer; it is patched into the low byte of a scratch word the function
/// never writes (uninitialized stack, pinned to 0 by the contract's stack
/// fill). A probe callee's nonzero answer combined with the selector dword
/// not being `2` picks a trailing constant (`0x41`, else `0x3b`); a lookup
/// callee takes that word's address, the constant and the patched word,
/// and returns a pointer whose pointed-to word feeds the next call. The
/// flag byte shifted to the top byte goes to one more call, a zero to the
/// next, and a final status call's nonzero answer selects `1.0`, else
/// `0.0`, for the last call, whose answer is returned. All fifteen callees
/// use caller-cleanup conventions. Returns the last callee's answer.
///
/// Original: 0x00E3E7F0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00E3E7F0() -> u32 {
    unsafe {
        const MODE_LO: u32 = 0x116c250;
        const ENABLE: u32 = 0x1161548;
        const SELECTOR: u32 = 0x11d6fd4;
        const REGISTRY: u32 = 0x1161548;
        const NEG_FIVE: u32 = 0xc0a00000;
        const POS_FIVE: u32 = 0x40a00000;
        const ONE: u32 = 0x3f800000;
        const CAL_PRIME0: u32 = 1;
        const CAL_PRIME1: u32 = 2;
        const CAL_MODE: u32 = 3;
        const CAL_FLOAT_A: u32 = 4;
        const CAL_FLOAT_B: u32 = 5;
        const CAL_PAIR: u32 = 6;
        const CAL_FIVE: u32 = 7;
        const CAL_REGISTRY: u32 = 8;
        const CAL_PROBE: u32 = 9;
        const CAL_LOOKUP: u32 = 10;
        const CAL_CELL: u32 = 11;
        const CAL_FLAG: u32 = 12;
        const CAL_ZERO: u32 = 13;
        const CAL_STATUS: u32 = 14;
        const CAL_LAST: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        // Scratch words filled by the priming and float callees. The first
        // is never read back. Both float callees share one out-pointer: the
        // first fills word 0 (immediately overwritten), the second fills
        // both words; word 0 feeds the pair calls as the first float and
        // word 1 as the second.
        let mut _unused: u32 = 0;
        let mut pair: [u32; 2] = [0, 0];
        lf_checker_rt::callee_cdecl!(
            CAL_PRIME0,
            u32,
            &mut _unused as *mut u32 as u32
        );
        lf_checker_rt::callee_cdecl!(CAL_PRIME1, u32, 1);
        let mode = rd32(lf_checker_rt::relocated(MODE_LO));
        let mode_arg =
            if mode & 0xff != 0x6a && mode >> 24 == 0 { 7 } else { 2 };
        lf_checker_rt::callee_cdecl!(CAL_MODE, u32, mode_arg);
        lf_checker_rt::callee_cdecl!(
            CAL_FLOAT_A,
            u32,
            pair.as_mut_ptr() as u32,
            0x2c
        );
        lf_checker_rt::callee_cdecl!(
            CAL_FLOAT_B,
            u32,
            7,
            0,
            pair.as_mut_ptr() as u32,
            0
        );
        let first = pair[0];
        let float2 = pair[1];
        // A third scratch word the original never writes: it reads
        // uninitialized stack here, which the contract pins to 0 with a
        // defined stack fill, so this starts at 0 on both sides.
        let mut uninit: u32 = 0;
        lf_checker_rt::callee_cdecl!(CAL_PAIR, u32, first, float2);
        lf_checker_rt::callee_cdecl!(CAL_PRIME1, u32, 1);
        lf_checker_rt::callee_cdecl!(CAL_PAIR, u32, first, float2);
        lf_checker_rt::callee_cdecl!(CAL_FIVE, u32, NEG_FIVE, POS_FIVE);

        // Incoming EBX is forced to 0xff by an OR; its entry value is dead.
        let mut flag: u8 = 0xff;
        if rd8(lf_checker_rt::relocated(ENABLE)) != 0 {
            let answer: u32 = lf_checker_rt::callee_thiscall!(
                CAL_REGISTRY,
                u32,
                lf_checker_rt::relocated(REGISTRY)
            );
            flag = (answer & 0xff) as u8;
        }
        uninit = (uninit & 0xffffff00) | flag as u32;
        let probe: u32 = lf_checker_rt::callee_cdecl!(CAL_PROBE, u32, 0);
        let tail = if probe & 0xff != 0
            && rd32(lf_checker_rt::relocated(SELECTOR)) != 2
        {
            0x41
        } else {
            0x3b
        };
        let cell_ptr: u32 = lf_checker_rt::callee_cdecl!(
            CAL_LOOKUP,
            u32,
            &mut uninit as *mut u32 as u32,
            tail,
            uninit
        );
        lf_checker_rt::callee_cdecl!(CAL_CELL, u32, rd32(cell_ptr));
        lf_checker_rt::callee_cdecl!(CAL_FLAG, u32, (flag as u32) << 24);
        lf_checker_rt::callee_cdecl!(CAL_ZERO, u32, 0);
        let status: u32 = lf_checker_rt::callee_cdecl!(CAL_STATUS, u32,);
        let last_arg = if status & 0xff != 0 { ONE } else { 0 };
        lf_checker_rt::callee_cdecl!(CAL_LAST, u32, last_arg)
    }
});
