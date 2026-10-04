// original: 0x00981ff0 audio_voice_slot_alloc
use lf_k2_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global};

/// Audio voice-slot allocator.
///
/// Unless the game window is minimized, the audio engine is muted, or the
/// engine reports busy, this writes one voice description into the banked
/// slot table owned by `this` and appends the slot to the owner's work list.
///
/// Layout (all offsets from `this`): `+0x5bf8` holds the current bank index;
/// `+0x5bf0` holds one live-voice count per bank. Bank `i`, voice `c` lives
/// at `(i * 245 + c) * 48` bytes from `this`. Each 48-byte slot carries the
/// 16-byte descriptor at `+0x10`, the owner handle at `+0x20`, two id words
/// at `+0x24`/`+0x28` and a gain value at `+0x2c`. The tag word for the
/// *next* slot is staged at offset `+0x00` of the following slot.
///
/// outright exit (no writes) when: window minimized, either mute flag set,
/// the busy probe fails, or the bank already holds 245 voices.
export!(thiscall, rw_00981ff0(
    this: u32,
    voice_id: u32,
    voice_flags: u32,
    desc: u32,
    owner: u32,
    gain: u32,
    next_tag: u32
) -> () {
    const SLOTS_PER_BANK: u32 = 245;
    const SLOT_SIZE: u32 = 48;
    const BANK_INDEX_OFF: u32 = 0x5bf8;
    const BANK_COUNTS_OFF: u32 = 0x5bf0;
    unsafe {
        // Gate 1: minimized window, or both config bytes set.
        let hwnd = *global::<u32>(0x17accd8);
        let iconic = callee_stdcall!(1, u32, hwnd);
        let blocked = if iconic == 0 {
            *global::<u8>(0x105b48f) != 0 && *global::<u8>(0x17ed8d1) != 0
        } else {
            true
        };
        // Either mute byte set also exits.
        let muted = *global::<u8>(0x1173590) | *global::<u8>(0x1173591);
        if blocked || muted != 0 {
            return;
        }
        // Gate 2: engine busy probe (low byte only). The callee ignores its
        // incoming registers (it loads everything from globals), so the ECX
        // value at this site is call-clobbered leftover, not an argument.
        if (callee_cdecl!(2, u32,) & 0xFF) != 0 {
            return;
        }
        let bank = *(this.wrapping_add(BANK_INDEX_OFF) as *const u32);
        let count_ptr =
            this.wrapping_add(BANK_COUNTS_OFF).wrapping_add(bank.wrapping_mul(4)) as *mut u32;
        let count = *count_ptr;
        if count >= SLOTS_PER_BANK {
            return;
        }
        let slot = this.wrapping_add(
            bank.wrapping_mul(SLOTS_PER_BANK).wrapping_add(count).wrapping_mul(SLOT_SIZE),
        ) as *mut u32;
        let d = desc as *const u32;
        *slot.byte_add(0x24) = voice_id;
        *slot.byte_add(0x28) = voice_flags;
        *slot.byte_add(0x10) = *d;
        *slot.byte_add(0x14) = *d.byte_add(4);
        *slot.byte_add(0x18) = *d.byte_add(8);
        *slot.byte_add(0x1c) = *d.byte_add(0xc);
        *slot.byte_add(0x2c) = gain;
        // Stage the next slot's tag word, then publish this slot's owner.
        let next_slot = (slot as u32).wrapping_add(SLOT_SIZE) as *mut u32;
        *next_slot = next_tag;
        *slot.byte_add(0x20) = owner;
        if owner != 0 {
            callee_thiscall!(3, u32, owner, (slot as u32).wrapping_add(0x20));
        }
        *count_ptr = count.wrapping_add(1);
    }
});
