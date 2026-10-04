// original: 0x00a64c20 NativeImpl_SET_COMBAT_DECISION_MAKER
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Apply one combat voice level from the shared voice table.
///
/// Records the incoming handle, then either takes the hot path (when the
/// voice mode global selects it and both the readiness poll and the keyed
/// lookup agree, stamping the fixed level) or the table path, which reads
/// the handle's level byte, clamps it into range, and stamps that. Both
/// paths end by converting the handle's gain byte to float and pushing it
/// into the entity's mixer. The shared table pointer is created on first
/// use. Returns the mixer's answer.
export!(thiscall, rw_00a64c20(this: u32, handle: u32) -> u32 {
    unsafe {
        let esi = this;
        ((esi + 0xD8) as *mut u32).write(handle);
        if global::<u32>(0x011D6FD4).read() == 3 {
            let ready = callee_cdecl!(1, u32,);
            if (ready as u8) != 0 {
                let ent = ((esi + 0x40) as *const u32).read();
                let vtable = (ent as *const u32).read();
                let target = ((vtable + 0xD0) as *const u32).read();
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                let row = slot(ent);
                let key = ((row + 0x12C) as *const u32).read();
                let hit = callee_cdecl!(3, u32, key);
                if (hit as u8) != 0 {
                    ((ent + 0x371) as *mut u8).write(0x14);
                    return voice_gain_tail(esi);
                }
            }
        }
        let table = ensure_voice_table();
        let held = ((esi + 0xD8) as *const u32).read();
        let row = callee_thiscall!(6, u32, table, held);
        // The original's negative clamp is dead (movzx never sets the sign
        // flag, so the jump-not-sign is always taken); only the top clamp
        // at 100 can fire.
        let level = ((row + 0x8FB) as *const u8).read().min(100);
        let ent = ((esi + 0x40) as *const u32).read();
        ((ent + 0x371) as *mut u8).write(level);
        voice_gain_tail(esi)
    }
});

/// Shared voice table pointer, created on first use.
#[inline(always)]
fn ensure_voice_table() -> u32 {
    unsafe {
        let slot = global::<u32>(0x0167E3B4);
        let cur = slot.read();
        if cur != 0 {
            return cur;
        }
        let fresh = callee_cdecl!(4, u32, 0x20020);
        // The original picks EAX up after the init call, so the stored
        // pointer is the init call's answer, not the fresh pointer.
        let stored = if fresh != 0 {
            callee_thiscall!(5, u32, fresh)
        } else {
            0
        };
        slot.write(stored);
        stored
    }
}

/// Convert the handle's gain byte and push it into the entity's mixer.
#[inline(always)]
fn voice_gain_tail(esi: u32) -> u32 {
    unsafe {
        let table = ensure_voice_table();
        let held = ((esi + 0xD8) as *const u32).read();
        let row = callee_thiscall!(6, u32, table, held);
        let gain = ((row + 0x8FA) as *const u8).read();
        let ent = ((esi + 0x40) as *const u32).read();
        let mixer = ((ent + 0x224) as *const u32).read();
        callee_thiscall!(7, u32, mixer, (gain as f32).to_bits())
    }
}
