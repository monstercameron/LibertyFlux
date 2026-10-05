// original: 0x008AA0F0 audio_slot_sweep (proposed)

// Guard address skipped in the contract (each side builds the guard in its
// own frame); guard contents compared at destructor time.

/// Sweep all live voice slots, refreshing chains that match `arg`.
///
/// Under the voice-list lock (guard over `this+0x3210`, constructor
/// `0x403fa0`, destructor `0x403fd0`), visits slots 0..799 with set bits in
/// the bitset at `this+0x28a0` whose head word at `this+0xfa4+8*i` is not
/// `0xffff`, and walks each slot's word chain rooted in the object itself:
/// for every link, the voice handle is resolved from the two link bytes
/// (`byte3 * [0x115d964] + [[0x115d988] + byte2 * 0x6f40 + 0x6f10]`) and
/// passed to the voice refresh (`0x8907c0`, thiscall); when the refresh
/// answers `arg`, the voice commit (`0x890c80`, thiscall) runs too. The
/// chain base is `this`: the original spills it to a stack slot and re-reads
/// it per slot, which the rewrite does directly. No return value. Original
/// is thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008AA0F0(this: u32, arg: u32) -> u32 {
    const GUARD_CTOR: u32 = 1;
    const GUARD_DTOR: u32 = 2;
    const REFRESH: u32 = 3;
    const COMMIT: u32 = 4;
    const CS: u32 = 0x3210;
    const BITSET: u32 = 0x28a0;
    const HEADS: u32 = 0xfa4;
    const VOICES: u32 = 0x320;
    const NONE: u32 = 0xffff;
    const CHAIN_FILE_VA: u32 = 0x0115_d988;
    const SCALE_FILE_VA: u32 = 0x0115_d964;
    unsafe {
        let mut guard = [0u32; 2];
        lf_checker_rt::callee_thiscall!(GUARD_CTOR, u32, &mut guard as *mut _ as u32, this + CS);
        let bits = ((this + BITSET) as *const u32).read_unaligned();
        let chain = (lf_checker_rt::global::<u32>(CHAIN_FILE_VA) as *const u32).read_unaligned();
        let scale = (lf_checker_rt::global::<u32>(SCALE_FILE_VA) as *const u32).read_unaligned();
        let mut mask = 1u32;
        let mut head = this + HEADS;
        let mut i = 0u32;
        while i < VOICES {
            let w = ((bits + (i >> 5) * 4) as *const u32).read_unaligned();
            if w & mask != 0 {
                let mut link = (head as *const u16).read_unaligned() as u32;
                if link != NONE {
                    loop {
                        let b2 = ((this + link * 4 + 2) as *const u8).read() as u32;
                        let b3 = ((this + link * 4 + 3) as *const u8).read() as u32;
                        let slot = ((chain + b2.wrapping_mul(0x6f40) + 0x6f10) as *const u32)
                            .read_unaligned();
                        let h = b3.wrapping_mul(scale).wrapping_add(slot);
                        let r: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, h);
                        if r == arg {
                            lf_checker_rt::callee_thiscall!(COMMIT, u32, h);
                        }
                        link = ((this + link * 4) as *const u16).read_unaligned() as u32;
                        if link == NONE {
                            break;
                        }
                    }
                }
            }
            mask = mask.rotate_left(1);
            head += 8;
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(GUARD_DTOR, u32, &mut guard as *mut _ as u32);
    }
    0
});
