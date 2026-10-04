// original: 0x00acef70 audio_voice_setup
// 0x00ACEF70: audio voice setup (proposed name: audio_voice_setup).
// Runs only while the voice timer at this+0x160 is positive. Looks up a
// voice id through the runtime table at 0x1295CD8 (indexed by a word at
// arg0+0x2E, then +0xCC, then head*4), resolves it to a voice struct with
// callee 1, and, when bit 3 of this+0x164 is set, runs a positional
// check (callee 2, a 10-word cdecl call whose out-word lands in this
// function's own incoming argument slot) followed by a dispatch on the
// global audio engine (callee 3). Then it steps the accumulator at
// this+0x16c, gates on the accumulator/energy pair and the shared RNG,
// and fires the start/notify/release tail calls (callees 4-7).
export!(thiscall, rw_00acef70(this: u32, arg0: u32) -> u32 {
    unsafe {
        // Gate 1: continue while the timer is positive (`comiss` 0,timer
        // + `jae`-exit exits exactly when 0 >= timer ordered, so NaN
        // continues too, which `timer <= 0.0` reproduces).
        // Gate 2: flag 0x10 clear.
        let timer = *((this + 0x160) as *const f32);
        if timer <= 0.0 {
            return 0;
        }
        if *((arg0 + 0xF17) as *const u8) & 0x10 != 0 {
            return 0;
        }
        // Voice-id lookup through the runtime table.
        let head = *(this as *const u32);
        let idx = *((arg0 + 0x2E) as *const i16) as i32 as u32;
        let tab = relocated(0x01295CD8).wrapping_add(idx.wrapping_mul(4));
        let tab_entry = *(tab as *const u32);
        let inner = *(tab_entry.wrapping_add(0xCC) as *const u32);
        let voice = *(inner.wrapping_add(head.wrapping_mul(4)) as *const u32);
        let posed: u32 = callee_thiscall!(1, u32, arg0, voice);
        let flags = *((this + 0x164) as *const u32);
        if ((flags >> 3) & 1) != 0 {
            if voice != 0xFFFFFFFF {
                let mut do_dispatch = true;
                if *((arg0 + 0x118) as *const u8) & 1 != 0 {
                    let b30 = *((posed + 0x30) as *const u32);
                    let b34 = *((posed + 0x34) as *const u32);
                    let b38 = *((posed + 0x38) as *const u32);
                    // The out-word lands in this function's own incoming
                    // argument slot; the rewrite models it with a local
                    // seeded with the same value (see the contract: the
                    // slot clobber itself is not compared, q-13 precedent).
                    // NOTE: the stub logs only the first 8 of the 10
                    // words; words 8-9 are the constant pushes 20.0 and 0
                    // and are passed faithfully but not compared.
                    let mut slot = arg0;
                    let ok: u32 = callee_cdecl!(
                        2, u32, b30, b34, b38, &mut slot as *mut u32 as u32, 1, 0, 0,
                        0x40C00000, 0x41A00000, 0
                    );
                    if (ok as u8) != 0 {
                        let got = f32::from_bits(slot);
                        let want = f32::from_bits(b38);
                        if !(got < want) {
                            *((this + 0x164) as *mut u32) = flags & 0xFFFFFFF7;
                            do_dispatch = false;
                        }
                    }
                }
                if do_dispatch {
                    let mix = *((this + 0x16C) as *const u32);
                    let engine = relocated(0x016D9F58);
                    callee_thiscall!(3, u32, engine, this, arg0, posed, mix, head);
                }
            }
            // Step the accumulator by the frame delta.
            let stepped = *global::<f32>(0x011735BC) + *((this + 0x16C) as *const f32);
            *((this + 0x16C) as *mut f32) = stepped;
        }
        // Gate: accumulator >= 10, else energy must reach 70. The
        // energy test is a bare `jb`-exit, taken also when unordered,
        // so NaN exits: `!(x >= 70.0)`, not `x < 70.0`.
        if !(*((this + 0x16C) as *const f32) >= 10.0) {
            if !(*((this + 0x8C) as *const f32) >= 70.0) {
                return 0;
            }
        }
        // Shared 64-bit RNG step (same generator as 0xACF190).
        let m1 = *global::<u32>(0x011101A0);
        let m2 = *global::<u32>(0x011101A4);
        let stepped64 = (m1 as u64).wrapping_mul(0x5CDCFAA7).wrapping_add(m2 as u64);
        *global::<u32>(0x011101A0) = stepped64 as u32;
        *global::<u32>(0x011101A4) = (stepped64 >> 32) as u32;
        let scale = *global::<f32>(0x00FE864C);
        let roll = ((stepped64 as u32) & 0x7FFFFF) as f32 * scale;
        if !(*global::<f32>(0x00FE870C) > roll) {
            return 0;
        }
        // Gate: the sub-object is absent or its flag byte is clear.
        let sub = *((arg0 + 0x6C) as *const u32);
        if sub != 0 && *((sub + 0xE) as *const u8) != 0 {
            return 0;
        }
        *((this + 0x160) as *mut u32) = 0;
        if voice != 0xFFFFFFFF {
            let c16c = (*((this + 0x16C) as *const f32) >= 10.0) as u32;
            let c8c = (*((this + 0x8C) as *const f32) >= 70.0) as u32;
            let engine = relocated(0x016D9F58);
            callee_thiscall!(
                4, u32, engine, this, arg0, posed.wrapping_add(0x30), head, c8c, c16c
            );
        }
        *((this + 0x164) as *mut u32) &= 0xFFFFFFF7;
        *((this + 0x16C) as *mut u32) = 0;
        let holder = *((arg0 + 0x34) as *const u32);
        let dep = *((holder.wrapping_add(4)) as *const u32);
        *((dep.wrapping_add(0xCC)) as *mut u8) = 1;
        callee_thiscall!(5, u32, arg0.wrapping_add(0x210), head, 0);
        let again: u32 = callee_thiscall!(6, u32, arg0);
        if (again as u8) == 0 {
            return 0;
        }
        callee_thiscall!(7, u32, arg0)
    }
});
