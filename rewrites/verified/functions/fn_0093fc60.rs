// original: 0x0093fc60 streaming_slot_state_check (proposed)

/// Decide whether the streaming slot is ready to advance.
///
/// Dispatches on the 4-bit tag at bits 6..9 of the word at `obj + 0x28`:
/// tag 6 is ready when the global enable flag is set and the slot word at
/// `+0x48` reads -1; tag 3 refreshes a countdown word at `+0x79c` (to
/// 0x2710 when the slot word is live, else decayed by the timer helper's
/// answer, saturating at zero) and is ready when the enable flag is set,
/// the state bytes disagree with the stalled markers and the countdown hit
/// zero; tag 2 walks a linked slot and is ready when the chain ends or its
/// marker byte is clear. Any other tag, and any failed gate, returns 0.
/// Only the low byte of the return value is set.
///
/// Original: 0x0093fc60 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_0093fc60(obj: u32) -> u32 {
    unsafe {
        const TAG_WORD: u32 = 0x28;
        const SLOT: u32 = 0x48;
        const COUNTDOWN: u32 = 0x79C;
        const STATE: u32 = 0xA60;
        const INNER: u32 = 0x21C;
        const INNER_STATE: u32 = 0x12C;
        const MARKER: u32 = 0x10B8;
        const FLAGS: u32 = 0xF1F;
        const NEXT: u32 = 0xF50;
        const NEXT_MARKER: u32 = 0x219;
        const ENABLE: u32 = 0x011A890A;
        const ENABLE_B: u32 = 0x011A4FB7;
        const STALLED: u8 = 2;
        const RELOAD: u16 = 0x2710;
        const CALLEE: u32 = 1;
        let tag = ((((obj + TAG_WORD) as *const u32).read_unaligned() >> 6) & 0xF) as u8;
        if tag == 6 {
            if lf_checker_rt::global::<u8>(ENABLE).read() == 0 {
                return 0;
            }
            return u32::from(
                ((obj + SLOT) as *const u32).read_unaligned() == 0xFFFF_FFFF,
            );
        }
        if tag == 3 {
            if ((obj + SLOT) as *const u32).read_unaligned() != 0xFFFF_FFFF {
                ((obj + COUNTDOWN) as *mut u16).write_unaligned(RELOAD);
                return 0;
            }
            let left = ((obj + COUNTDOWN) as *const u16).read_unaligned() as u32;
            if left != 0 {
                let step: u32 = lf_checker_rt::callee_cdecl!(CALLEE, u32,);
                let rest = if left > step { left - step } else { 0 };
                ((obj + COUNTDOWN) as *mut u16).write_unaligned(rest as u16);
            }
            if lf_checker_rt::global::<u8>(ENABLE).read() == 0 {
                return 0;
            }
            if ((obj + STATE) as *const u8).read() == STALLED {
                return 0;
            }
            let inner = ((obj + INNER) as *const u32).read_unaligned();
            if ((inner + INNER_STATE) as *const u32).read_unaligned() == STALLED as u32 {
                return 0;
            }
            return u32::from(
                ((obj + COUNTDOWN) as *const u16).read_unaligned() == 0,
            );
        }
        if tag == 2 {
            if lf_checker_rt::global::<u8>(ENABLE_B).read() == 0 {
                return 0;
            }
            if ((obj + SLOT) as *const u32).read_unaligned() != 0xFFFF_FFFF {
                return 0;
            }
            if ((obj + MARKER) as *const u8).read() == tag {
                return 0;
            }
            if ((obj + FLAGS) as *const u8).read() & 0x20 != 0 {
                return 0;
            }
            let next = ((obj + NEXT) as *const u32).read_unaligned();
            if next == 0 {
                return 1;
            }
            if ((next + STATE) as *const u8).read() == STALLED {
                return 0;
            }
            return u32::from(((next + NEXT_MARKER) as *const u8).read() == 0);
        }
        0
    }
});
