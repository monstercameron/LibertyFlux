// original: 0x008A9FE0 audio_slot_alloc (proposed)
// Guard address skipped in the contract (each side builds the guard in its
// own frame); guard contents compared at destructor time.

/// Allocate the first free voice slot and initialise it from `arg`.
///
/// Under the voice-list lock (guard over `this+0x3210`), scans slots 1..799
/// for the first clear bit in the bitset at `this+0x28a0`. When every slot
/// is taken returns `0xffff`. Otherwise stores `arg` at `slot+0xfa0`,
/// `0xffff` at `slot+0xfa4`, sets the bit and returns the slot index.
/// Original is thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008A9FE0(this: u32, arg: u32) -> u32 {
    const GUARD_CTOR: u32 = 1;
    const GUARD_DTOR: u32 = 2;
    const CS: u32 = 0x3210;
    const BITSET: u32 = 0x28a0;
    const SLOTS: u32 = 0xfa0;
    const VOICES: u32 = 0x320;
    const NONE: u32 = 0xffff;
    unsafe {
        let mut guard = [0u32; 2];
        lf_checker_rt::callee_thiscall!(GUARD_CTOR, u32, &mut guard as *mut _ as u32, this + CS);
        let bits = ((this + BITSET) as *const u32).read_unaligned();
        let mut i = 1u32;
        while i < VOICES {
            let w = ((bits + (i >> 5) * 4) as *const u32).read_unaligned();
            if w & (1u32 << (i & 31)) == 0 {
                let slot = this + SLOTS + i * 8;
                (slot as *mut u32).write_unaligned(arg);
                ((slot + 4) as *mut u16).write_unaligned(NONE as u16);
                let wp = (bits + (i >> 5) * 4) as *mut u32;
                wp.write_unaligned(w | (1u32 << (i & 31)));
                lf_checker_rt::callee_thiscall!(GUARD_DTOR, u32, &mut guard as *mut _ as u32);
                return i;
            }
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(GUARD_DTOR, u32, &mut guard as *mut _ as u32);
        NONE
    }
});
