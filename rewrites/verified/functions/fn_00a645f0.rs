// original: 0x00a645f0 track_aim_point
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

/// Track one entity's aim point against its moving target.
///
/// Polls the staged selector for a target handle, then either bumps the
/// settle counter or walks the candidate chain: each candidate whose range
/// weight clears the floor is bit-tested, and a surviving candidate is
/// confirmed through the resolver before the chain refetches. Past the
/// chain, the settle counter gates a snapshot of the target frame, and a
/// distance gate against one of two thresholds either ages the lock or
/// re-snaps the aim point. Returns nothing meaningful.
export!(thiscall, rw_00a645f0(this: u32) -> u32 {
    unsafe {
        let esi = this;
        let ent = ((esi + 0x40) as *const u32).read();
        let group = ((ent + 0x224) as *const u32).read();
        let mode = ((group + 0x2E8) as *const u32).read() & 7;
        let mut slot_a = 0xFFFF_FFFFu32;
        let mut slot_b = 0u8;
        let poll = callee_thiscall!(
            1,
            u32,
            esi,
            &mut slot_a as *mut u32 as u32,
            &mut slot_b as *mut u8 as u32
        );
        let mut flag = false;
        if (poll as u8) != 0 {
            flag = slot_a == 0xF || slot_a == 0xE || slot_a == 2;
        }
        if mode != 2 && mode != 3 && mode != 4 {
            if flag {
                settle_state(esi);
                settle_join(esi);
            } else {
                settle_bump(esi);
                settle_exit(esi);
            }
            return 0;
        }
        if !flag {
            let ent2 = ((esi + 0x40) as *const u32).read();
            let hub = ((ent2 + 0x78) as *const u32).read();
            let mut cur = callee_thiscall!(2, u32, hub, 0, 4);
            if cur != 0 {
                loop {
                    if candidate_ok(cur) {
                        let bits = ((cur + 4) as *const u32).read();
                        if bits >> 0x15 & 1 == 0
                            && confirm_candidate(esi, cur, bits)
                        {
                            settle_exit(esi);
                            return 0;
                        }
                        if !candidate_bits_clear(cur) {
                            settle_exit(esi);
                            return 0;
                        }
                    }
                    let ent3 = ((esi + 0x40) as *const u32).read();
                    let hub3 = ((ent3 + 0x78) as *const u32).read();
                    cur = callee_thiscall!(4, u32, hub3, 0, 4);
                    if cur == 0 {
                        break;
                    }
                }
            }
        }
        settle_state(esi);
        settle_join(esi);
        0
    }
});

/// Range-floor test for one candidate: below the floor or unordered refetches.
#[inline(always)]
fn candidate_ok(cur: u32) -> bool {
    unsafe {
        let w = f32::from_bits(((cur + 0x58) as *const u32).read());
        w >= f32::from_bits(0x3F666666)
    }
}

/// Confirm one candidate through the resolver; true means the lane exits.
#[inline(always)]
fn confirm_candidate(esi: u32, cur: u32, bits: u32) -> bool {
    unsafe {
        let tag = ((cur + 0x44) as *const u16).read();
        let tgt = if tag == 1 {
            ((cur + 0x40) as *const u32).read()
        } else {
            0
        };
        if ((tgt + 6) as *const u8).read() & 0x10 == 0 {
            return false;
        }
        if bits & 0xF == 0 {
            return true;
        }
        let verdict = callee_cdecl!(3, u32, bits & 0xF, 0);
        (verdict as u8) != 0
    }
}

/// Trailing bit screen shared by every surviving candidate.
#[inline(always)]
fn candidate_bits_clear(cur: u32) -> bool {
    unsafe {
        let bits = ((cur + 4) as *const u32).read();
        bits >> 9 & 1 == 0 && bits >> 10 & 1 == 0 && bits >> 11 & 1 == 0 && bits >> 12 & 1 == 0
    }
}

/// Settle-counter bump with the wrap sentinel remapped to the retry value.
#[inline(always)]
fn settle_bump(esi: u32) {
    unsafe {
        let slot = (esi + 0x268) as *mut u32;
        let n = slot.read().wrapping_add(1);
        slot.write(if n == 0xFFFF_FFFF { 4 } else { n });
    }
}

/// Lane exit: clear the age slot.
#[inline(always)]
fn settle_exit(esi: u32) {
    unsafe {
        ((esi + 0x264) as *mut u32).write(0);
    }
}

/// Post-chain state: snapshot the target frame once the counter runs past
/// its window, otherwise reset the counter (or bump it when unlinked).
#[inline(always)]
fn settle_state(esi: u32) {
    unsafe {
        let ent = ((esi + 0x40) as *const u32).read();
        if ((ent + 0x16C) as *const u32).read() == 0 {
            settle_bump(esi);
            return;
        }
        if ((esi + 0x268) as *const u32).read() > 4 {
            let frame = ((ent + 0x20) as *const u32).read();
            snap_frame(esi, frame);
        }
        ((esi + 0x268) as *mut u32).write(0);
    }
}

/// Join: an overrun counter exits, otherwise the distance gate runs.
#[inline(always)]
fn settle_join(esi: u32) {
    unsafe {
        if ((esi + 0x268) as *const u32).read() > 4 {
            settle_exit(esi);
            return;
        }
        let ent = ((esi + 0x40) as *const u32).read();
        let wide = ((ent + 0x29C) as *const u8).read() & 4 != 0;
        let frame = ((ent + 0x20) as *const u32).read();
        let dx = f32::from_bits(((frame + 0x30) as *const u32).read())
            - f32::from_bits(((esi + 0x270) as *const u32).read());
        let dy = f32::from_bits(((frame + 0x34) as *const u32).read())
            - f32::from_bits(((esi + 0x274) as *const u32).read());
        let dist2 = dy * dy + dx * dx;
        let limit = if wide { 0.125f32 } else { 0.0625f32 };
        if limit > dist2 {
            let age = (esi + 0x264) as *mut u32;
            age.write(age.read().wrapping_add(1));
        } else {
            settle_exit(esi);
            snap_frame(esi, frame);
        }
    }
}

/// Copy one target frame (four words) into the aim slots.
#[inline(always)]
fn snap_frame(esi: u32, frame: u32) {
    unsafe {
        ((esi + 0x270) as *mut u32).write(((frame + 0x30) as *const u32).read());
        ((esi + 0x274) as *mut u32).write(((frame + 0x34) as *const u32).read());
        ((esi + 0x278) as *mut u32).write(((frame + 0x38) as *const u32).read());
        ((esi + 0x27C) as *mut u32).write(((frame + 0x3C) as *const u32).read());
    }
}
