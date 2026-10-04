// original: 0x00a64130 refresh_pain_voice
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Refresh the pain-voice state of one attached entity.
///
/// Runs the sibling update over this object, resolves two staged selectors,
/// then walks a chain of gated voice stages: a distance gate against the
/// configured radius, a polled playback request, and two alignment stages
/// that set sticky flags on the entity. Each gate is driven by a scripted
/// callee answer or a heap/global field, and every floating-point
/// comparison treats an unordered (NaN) result as "gate closed", matching
/// the original's branch-on-below-or-equal structure. Returns the attached
/// entity pointer.
export!(thiscall, rw_00a64130(this: u32) -> u32 {
    const VOICE_POLL: u32 = 0x0167E2A0;
    unsafe {
        let obj = (this + 0x40) as *const u32;
        // Sibling update, then the staged selector pair.
        callee_thiscall!(1, u32, this);
        let picked = ((obj.read() + 0xA80) as *const u32).read();
        callee_thiscall!(2, u32, picked, 0);
        let scratch = this + 0x84;
        let first = callee_thiscall!(3, u32, this + 0x230, obj.read(), scratch);
        if (first as u8) == 0 {
            callee_thiscall!(4, u32, this + 0x220, obj.read(), scratch);
        }
        // Gated voice-request chain; any closed gate skips straight to the
        // tail, past the polled playback block below.
        let ent = ((this + 0x40) as *const u32).read();
        let mut chained = false;
        if ((ent + 0xAB0) as *const u32).read() != 0 {
            let gen = ((ent + 0x38) as *const u32).read();
            let stale = gen != 0 && gen == ((ent + 0x7B4) as *const u32).read();
            if !stale {
                let ready = callee_cdecl!(5, u32,);
                if (ready as u8) != 0 {
                    let ent2 = ((this + 0x40) as *const u32).read();
                    if ((ent2 + 0x7B8) as *const u32).read() != 0 {
                        let chan = ((ent2 + 0xAB0) as *const u32).read();
                        let ok = callee_cdecl!(6, u32, ent2, 0xC, chan, 0);
                        if (ok as u8) != 0 {
                            voice_request(this);
                            chained = true;
                        }
                    }
                }
            }
        }
        if chained {
            // Polled playback request; a null poll result skips the issue call.
            let poll = global::<u32>(VOICE_POLL).read();
        let handle = callee_thiscall!(10, u32, poll);
            let voice = if handle == 0 {
                0
            } else {
                let ent3 = ((this + 0x40) as *const u32).read();
                let chan3 = ((ent3 + 0xAB0) as *const u32).read();
                callee_thiscall!(11, u32, handle, 0x7D0, 0x7530, chan3, 4, 0, 0, 0, 0)
            };
            let ent4 = ((this + 0x40) as *const u32).read();
            callee_thiscall!(12, u32, voice, ent4);
            // Three frame-scratch consumers; their buffers are never read back.
            let mut stage = [0u32; 8];
            callee_thiscall!(13, u32, stage.as_mut_ptr() as u32, 0x7530, voice, 0);
            let mut mix = [0u32; 8];
            callee_thiscall!(14, u32, this + 0x84, mix.as_mut_ptr() as u32, 0, 1);
            let mut tail = [0u32; 8];
            callee_thiscall!(15, u32, tail.as_mut_ptr() as u32);
        }
        // Alignment stages set sticky flags, then the cooldown slot.
        let ent5 = ((this + 0x40) as *const u32).read();
        align_gate(ent5);
        aim_gate(this, ent5);
        let ent6 = ((this + 0x40) as *const u32).read();
        if ((ent6 + 0x26C) as *const u32).read() & 0x2000 == 0 {
            ((this + 0x288) as *mut u32).write(0x461C3C00);
        }
        ent6
    }
});

/// Distance-gated voice request through the entity's position vtable slot.
///
/// Reads the returned position triple, and only when its squared length
/// strictly exceeds the configured radius squared issues the two staged
/// request calls.
#[inline(always)]
fn voice_request(this: u32) {
    const RADIUS_SQ_SRC: u32 = 0x0103CD64;
    unsafe {
        let ent = ((this + 0x40) as *const u32).read();
        let vtable = (ent as *const u32).read();
        let target = ((vtable + 0xEC) as *const u32).read();
        let slot_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let mut pos_slot = [0u32; 4];
        let pos = slot_fn(ent, pos_slot.as_mut_ptr() as u32);
        let x = f32::from_bits((pos as *const u32).read());
        let y = f32::from_bits(((pos + 4) as *const u32).read());
        let z = f32::from_bits(((pos + 8) as *const u32).read());
        let dist2 = x * x + y * y + z * z;
        let radius = f32::from_bits(global::<u32>(RADIUS_SQ_SRC).read());
        if dist2 > radius * radius {
            let r8 = callee_thiscall!(8, u32, 0, relocated(0x00E9D7A4), 0, 0xFFFF_FFFF, 0, 0, 0x3F80_0000);
            let ent2 = ((this + 0x40) as *const u32).read();
            callee_thiscall!(9, u32, ent2 + 0x570, relocated(0x00E9D7B0), 1, 1, r8);
        }
    }
}

/// First alignment stage: sets the entity's sticky bit when the blended
/// direction score strictly exceeds the negative threshold.
#[inline(always)]
fn align_gate(ent: u32) {
    unsafe {
        if !(f32::from_bits(((ent + 0x170) as *const u32).read()) > 0.0) {
            return;
        }
        let link = ((ent + 0x16C) as *const u32).read();
        if link == 0 || ((link + 0x28) as *const u32).read() & 0x3C0 == 0xC0 {
            return;
        }
        let basis = ((ent + 0x20) as *const u32).read();
        let c184 = f32::from_bits(((ent + 0x184) as *const u32).read());
        let m10 = f32::from_bits(((basis + 0x10) as *const u32).read());
        let c180 = f32::from_bits(((ent + 0x180) as *const u32).read());
        let m14 = f32::from_bits(((basis + 0x14) as *const u32).read());
        let c188 = f32::from_bits(((ent + 0x188) as *const u32).read());
        let m18 = f32::from_bits(((basis + 0x18) as *const u32).read());
        let score = (c184 * m14 + m10 * c180) + c188 * m18;
        if -0.5f32 > score {
            let flags = (ent + 0x29C) as *mut u32;
            flags.write(flags.read() | 0x40);
        }
    }
}

/// Second alignment stage: resolves the aim delta through two direction
/// samples and latches the entity state when either sample wins strictly.
#[inline(always)]
fn aim_gate(this: u32, ent: u32) {
    unsafe {
        if !(f32::from_bits(((ent + 0x170) as *const u32).read()) > 0.0) {
            return;
        }
        let link = ((ent + 0x16C) as *const u32).read();
        if link == 0
            || ((link + 0x28) as *const u32).read() & 0x3C0 != 0x100
            || ((link + 0x22B) as *const u8).read() == 0
            || ((link + 0x249) as *const u8).read() == 0
        {
            return;
        }
        let alt = ((link + 0x20) as *const u32).read();
        let base = if alt != 0 { alt + 0x30 } else { link + 0x10 };
        let basis = ((ent + 0x20) as *const u32).read();
        let d0 = f32::from_bits(((basis + 0x30) as *const u32).read())
            - f32::from_bits((base as *const u32).read());
        let d1 = f32::from_bits(((basis + 0x34) as *const u32).read())
            - f32::from_bits(((base + 4) as *const u32).read());
        let d2 = f32::from_bits(((basis + 0x38) as *const u32).read())
            - f32::from_bits(((base + 8) as *const u32).read());
        let dir1 = callee_cdecl!(16, u32,);
        let before = sample_score(dir1, d0, d1, d2);
        let mood = (link + 0x249) as *const i8;
        if !(before > 0.0 && mood.read() < 0) {
            let dir2 = callee_cdecl!(16, u32,);
            let after = sample_score(dir2, d0, d1, d2);
            if 0.0f32 > after && mood.read() > 0 {
                latch(this);
            }
        } else {
            latch(this);
        }
    }
}

/// Dot product of one direction sample against the aim delta.
#[inline(always)]
fn sample_score(dir: u32, d0: f32, d1: f32, d2: f32) -> f32 {
    unsafe {
        let f10 = f32::from_bits(((dir + 0x10) as *const u32).read());
        let f14 = f32::from_bits(((dir + 0x14) as *const u32).read());
        let f18 = f32::from_bits(((dir + 0x18) as *const u32).read());
        (f14 * d1 + f10 * d0) + f18 * d2
    }
}

/// Latch the entity's aimed state byte.
#[inline(always)]
fn latch(this: u32) {
    unsafe {
        let ent = ((this + 0x40) as *const u32).read();
        ((ent + 0x299) as *mut u8).write(0xA);
    }
}
