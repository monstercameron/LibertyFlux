// original: 0x0097d390 audio_positional_oneshot_dispatch (proposed)

/// Dispatch a positional one-shot voice, or resolve it quietly.
///
/// `this` is the emitter state (byte at `+0x10` marked active, dword at
/// `+0x120` is the slot bank); `index` selects the voice; `pos_obj` points
/// at an object whose dword at `+0x00` is the position block and whose three
/// dwords at `+0x10`/`+0x14`/`+0x18` are the position.
///
/// Behaviour: three global gates must pass (init flag not 1, two epoch
/// words equal, mode not 0x12) or the function returns the epoch at once.
/// Otherwise the position block is tested for the flag masked by `0x3C0`
/// (set when the masked bits equal `0xC0`), the voice id is read from the
/// resolved bank at `+0x12` (flag set) or `+0x0E` (flag clear), and the
/// slot chain is followed: a null bank or null allocation joins the quiet
/// tail, a zero or sentinel id skips the submit test, a refused submit
/// re-resolves the id from the twin slot. Any path that reaches the tail
/// with a zero id exits with the last observed value stamped with the flag
/// byte in its low 8 bits; the rest build a
/// descriptor block in frame scratch (block at frame `+0x38`, position
/// copied to frame `+0x20`, tag `0xFFFFFFFF` and tail `0x46BAB800`) and
/// issue it: the bank sample always plays, and the extra voice plays only
/// when the flag was set. On a clear flag the descriptor root comes back
/// through two float conversions; on a set flag the emitter itself is it.
///
/// Floats are only copied or carried as bits, never computed, so no
/// arithmetic order applies. The gate-1 exit returns the caller's entry
/// EAX, which a rewrite cannot observe; the proof keeps gate 1 passing.
///
/// Original: 0x0097D390 (thiscall, two stack words).
mod d390 {
    pub const GATE_INIT: u32 = 0x11F7060;
    pub const GATE_EPOCH_A: u32 = 0x12088B4;
    pub const GATE_EPOCH_B: u32 = 0xF1C040;
    pub const GATE_MODE: u32 = 0x1037720;
    pub const GATE_MODE_EXIT: u32 = 0x12;
    pub const BANK_OBJ: u32 = 0x115D9A0;
    pub const FLOAT_OBJ: u32 = 0x12202E0;
    pub const VOICE_SENTINEL: u32 = 0x1231310;
    pub const VOICE_TABLE: u32 = 0x1295CD8;
    pub const FLAG_MASK: u32 = 0x3C0;
    pub const FLAG_BITS: u32 = 0x0C0;
    pub const ACTIVE_BYTE: u8 = 1;
    pub const STRUCT_HEAD: u32 = 0x258;
    pub const STRUCT_TAG: u32 = 0xFFFFFFFF;
    pub const STRUCT_TAIL: u32 = 0x46BAB800;
    pub const RING_BITS: u32 = 0x3F000000;
    pub const F_POS: usize = 0x20 / 4;
    pub const F_BLOCK: usize = 0x38 / 4;
    pub const C_OUTER: u32 = 1;
    pub const C_RESOLVE: u32 = 2;
    pub const C_SLOT_ALLOC: u32 = 3;
    pub const C_SUBMIT: u32 = 4;
    pub const C_FLOAT_CVT: u32 = 5;
    pub const C_FLOAT_BITS: u32 = 6;
    pub const C_BLOCK_INIT: u32 = 7;
    pub const C_ISSUE: u32 = 8;
    pub const C_PLAY_BANK: u32 = 9;
    pub const C_PLAY_EXTRA: u32 = 10;

    #[inline(always)]
    pub unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }

    #[inline(always)]
    pub unsafe fn rd16(a: u32) -> u16 {
        unsafe { (a as *const u16).read_unaligned() }
    }

    /// Entry half: gates, flag test, id resolution, slot chain.
    pub unsafe fn entry(
        this: u32,
        index: u32,
        pos_obj: u32,
        mark_active: bool,
    ) -> u32 {
        unsafe {
            if lf_checker_rt::global::<u32>(GATE_INIT).read() == 1 {
                // Unreachable under the proof (gate pinned passing): the
                // original returns entry EAX here, which no rewrite can read.
                return 0;
            }
            let epoch = lf_checker_rt::global::<u32>(GATE_EPOCH_A).read();
            if epoch != lf_checker_rt::global::<u32>(GATE_EPOCH_B).read() {
                return epoch;
            }
            if lf_checker_rt::global::<u32>(GATE_MODE).read() == GATE_MODE_EXIT {
                return epoch;
            }
            if mark_active {
                (this.wrapping_add(0x10) as *mut u8).write(ACTIVE_BYTE);
            }
            let pos_block = rd32(pos_obj);
            let flag = if pos_block == 0 {
                false
            } else {
                let outer = lf_checker_rt::callee_cdecl!(C_OUTER, u32, pos_block);
                if outer == 0 {
                    false
                } else {
                    rd32(outer.wrapping_add(0x28)) & FLAG_MASK == FLAG_BITS
                }
            };
            let mut voice = 0u32;
            let bank = lf_checker_rt::callee_thiscall!(
                C_RESOLVE,
                u32,
                lf_checker_rt::relocated(BANK_OBJ),
                index
            );
            if bank != 0 {
                voice = rd32(bank.wrapping_add(if flag { 0x12 } else { 0x0E }));
            }
            let slot = rd32(this.wrapping_add(0x120));
            // The shared tail reloads AL with the flag byte, so every quiet
            // exit answers the last value with its low byte replaced.
            let stamp = |v: u32| (v & 0xFF_FF_FF_00) | (flag as u32);
            if slot == 0 {
                if voice == 0 {
                    return stamp(this);
                }
                return main_body(this, index, pos_obj, pos_block, flag, voice, 0);
            }
            let alloc = lf_checker_rt::callee_thiscall!(C_SLOT_ALLOC, u32, slot.wrapping_add(0x2B0));
            if alloc == 0 {
                if voice == 0 {
                    return stamp(0);
                }
                return main_body(this, index, pos_obj, pos_block, flag, voice, 0);
            }
            if voice != 0
                && voice != lf_checker_rt::global::<u32>(VOICE_SENTINEL).read()
            {
                if rd32(alloc.wrapping_add(0x18)) != 0 {
                    let al = lf_checker_rt::callee_thiscall!(C_SUBMIT, u32, alloc);
                    if al != 0 {
                        return main_body(this, index, pos_obj, pos_block, flag, voice, 0);
                    }
                    return after_slot2(alloc, this, index, pos_obj, pos_block, flag, al, voice);
                }
                return main_body(this, index, pos_obj, pos_block, flag, voice, 0);
            }
            after_slot2(alloc, this, index, pos_obj, pos_block, flag, alloc, voice)
        }
    }

    /// Slot-chain tail: optionally re-resolves the id from the twin slot.
    unsafe fn after_slot2(
        alloc: u32,
        this: u32,
        index: u32,
        pos_obj: u32,
        pos_block: u32,
        flag: bool,
        entry_eax: u32,
        voice: u32,
    ) -> u32 {
        unsafe {
            let next = rd32(alloc.wrapping_add(0x10));
            let stamp = |v: u32| (v & 0xFF_FF_FF_00) | (flag as u32);
            if next == 0 {
                if voice == 0 {
                    return stamp(entry_eax);
                }
                return main_body(this, index, pos_obj, pos_block, flag, voice, 0);
            }
            let e14 = rd32(alloc.wrapping_add(0x18));
            let voice = rd32(next.wrapping_add(if flag { 0x1E } else { 0x1A }));
            if voice == 0 {
                return stamp(entry_eax);
            }
            main_body(this, index, pos_obj, pos_block, flag, voice, e14)
        }
    }

    /// Main body: descriptor block, issue, bank sample, extra voice.
    #[allow(clippy::too_many_arguments)]
    unsafe fn main_body(
        this: u32,
        index: u32,
        pos_obj: u32,
        pos_block: u32,
        flag: bool,
        voice: u32,
        e14: u32,
    ) -> u32 {
        unsafe {
            let mut frame = [0u32; 0x80];
            frame[F_POS] = rd32(pos_obj.wrapping_add(0x10));
            frame[F_POS + 1] = rd32(pos_obj.wrapping_add(0x14));
            frame[F_POS + 2] = rd32(pos_obj.wrapping_add(0x18));
            let base = frame.as_mut_ptr() as u32;
            let block = base.wrapping_add((F_BLOCK * 4) as u32);
            let root: u32 = if flag {
                this
            } else {
                let f: f32 = lf_checker_rt::callee_thiscall!(
                    C_FLOAT_CVT,
                    f32,
                    lf_checker_rt::relocated(FLOAT_OBJ),
                    pos_obj
                );
                let g: f32 = lf_checker_rt::callee_cdecl!(C_FLOAT_BITS, f32, f.to_bits());
                g.to_bits()
            };
            lf_checker_rt::callee_thiscall!(C_BLOCK_INIT, u32, block);
            let s = frame.as_mut_ptr().add(F_BLOCK);
            let idx_bank = rd32(root.wrapping_add(0x120));
            let idx = rd16(idx_bank.wrapping_add(0x2E)) as i16 as i32 as u32;
            let entry = lf_checker_rt::global::<u32>(VOICE_TABLE.wrapping_add(idx.wrapping_mul(4)))
                .read();
            let dl = rd32(entry.wrapping_add(0x120)) & 2 != 0;
            s.add(0).write(if dl { 0 } else { STRUCT_HEAD });
            s.add(1).write(0);
            s.add(2).write(0);
            s.add(3).write(0);
            s.add(4).write(0);
            s.add(5).write(base.wrapping_add((F_POS * 4) as u32));
            s.add(6).write(0);
            s.add(7).write(0);
            s.add(8).write(rd32(root.wrapping_add(8)));
            s.add(9).write(0);
            s.add(10).write(0);
            s.add(11).write(0);
            s.add(12).write(0);
            s.add(13).write(0);
            s.add(14).write(STRUCT_TAG);
            s.add(15).write(STRUCT_TAIL);
            let _issue = lf_checker_rt::callee_thiscall!(
                C_ISSUE,
                u32,
                root,
                voice,
                block,
                0xFFFFFFFF,
                0,
                0
            );
            let root120 = rd32(root.wrapping_add(0x120));
            let play = lf_checker_rt::callee_cdecl!(
                C_PLAY_BANK,
                u32,
                root120,
                index,
                flag as u32,
                base.wrapping_add((F_POS * 4) as u32),
                root,
                e14
            );
            if !flag {
                return play;
            }
            let q2 = lf_checker_rt::callee_cdecl!(C_OUTER, u32, pos_block);
            let si = rd16(pos_obj.wrapping_add(0x52)) as u32;
            lf_checker_rt::callee_thiscall!(
                C_PLAY_EXTRA,
                u32,
                q2.wrapping_add(0x3C0),
                RING_BITS,
                si
            )
        }
    }
}

lf_checker_rt::export!(thiscall, rw_0097d390(this: u32, index: u32, pos_obj: u32) -> u32 {
    unsafe { d390::entry(this, index, pos_obj, true) }
});
