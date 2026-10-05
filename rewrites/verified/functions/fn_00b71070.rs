// original: 0x00b71070 CTaskSimpleCarShuffle::vf5

/// Task event handler: on event 2, release the shuffle target at full speed.
///
/// Ignores any event but 2 (returns 0, keeping the caller's upper return
/// bytes). With no target stored at `this+0x18` reports done at once.
/// Otherwise calls the release callee (thiscall on the target, one stack
/// word: -1000.0f; its answer is ignored), flags the target's second word
/// with 0x4000 and reports done. The low return byte is the decision; the
/// upper bytes repeat the target pointer with its low byte cleared on the
/// taken path (the contract pins entry eax so the early paths are
/// deterministic too).
///
/// Original: 0x00b71070 (thiscall, three stack words; only the second, the
/// event id, is read).
lf_checker_rt::export!(thiscall, rw_00b71070(this: u32, _a1: u32, event: u32, _a3: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x18;
        const FLAG_WORD_OFF: u32 = 4;
        const RELEASED_FLAG: u32 = 0x4000;
        const SHUFFLE_EVENT: u32 = 2;
        const RELEASE_SPEED: u32 = 0xc47a0000; // -1000.0f
        const RELEASE: u32 = 1;
        if event != SHUFFLE_EVENT {
            return 0;
        }
        let target = ((this + TARGET_OFF) as *const u32).read_unaligned();
        if target == 0 {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, target, RELEASE_SPEED);
        let slot = (target + FLAG_WORD_OFF) as *mut u32;
        slot.write_unaligned(slot.read_unaligned() | RELEASED_FLAG);
        // The original reloads the target into the return register before
        // setting al, so the upper return bytes are the target's, not the
        // release answer's (which is ignored).
        (target & 0xffff_ff00) | 1
    }
});
