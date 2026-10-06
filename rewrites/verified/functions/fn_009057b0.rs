// original: 0x009057b0 input_ready_check (proposed)
/// Report whether the input subsystem is ready to produce values.
///
/// Reads only static state. Returns 1 when both data words are non-zero and
/// either the mode word selects the ready mode (with the state word clear)
/// or the state word is already set; otherwise 0. Cdecl with no arguments;
/// only the low byte of the return is set.
export!(cdecl, rw_009057b0() -> u32 {
    unsafe {
        /// Mode selector word (file VA).
        const MODE: u32 = 0x011F7060;
        /// First data word (file VA).
        const DATA_A: u32 = 0x012088B4;
        /// Second data word (file VA).
        const DATA_B: u32 = 0x00F1C040;
        /// Ready-mode value the state word must hold (file VA of the word).
        const STATE: u32 = 0x01037720;
        /// Ready-mode value.
        const READY_MODE: u32 = 0x12;
        /// Latched-state word (file VA).
        const LATCH: u32 = 0x011F66A0;
        /// Output words that must both be non-zero (file VAs).
        const OUT_A: u32 = 0x012088B8;
        const OUT_B: u32 = 0x012088BC;
        const MODE_READY: u32 = 1;
        // Only the fall-through and first-two-branch paths consult the latch;
        // a non-ready state word jumps straight to the output checks.
        let direct = (global::<u32>(MODE)).read_unaligned() != MODE_READY
            && (global::<u32>(DATA_A)).read_unaligned()
                == (global::<u32>(DATA_B)).read_unaligned()
            && (global::<u32>(STATE)).read_unaligned() != READY_MODE;
        if !direct && (global::<u32>(LATCH)).read_unaligned() != 0 {
            return 0;
        }
        if (global::<u32>(OUT_A)).read_unaligned() == 0 {
            return 0;
        }
        if (global::<u32>(OUT_B)).read_unaligned() == 0 {
            return 0;
        }
        1
    }
});
