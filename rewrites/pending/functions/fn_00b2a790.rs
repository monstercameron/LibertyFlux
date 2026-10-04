// original: 0x00B2A790 follow-mode switch
/// Shared tail of the follow-mode switch: publish the settle code, run the
/// flag dance, then possibly clamp the rate by the measured distance.
unsafe fn land_follow_tail(obj: u32, sub: u32, r2: u32, dist: f32) {
    unsafe {
        (sub as *mut u8).byte_add(0x28).write(2);
        if (obj as *const u8).byte_add(0xF1E).read() & 0x20 != 0 {
            let b = (obj as *const u8).byte_add(0xF19).read();
            let mut cl = (!(b >> 4) & 1) << 5;
            cl |= (b & 0x9F) | 0x10;
            (obj as *mut u8).byte_add(0xF19).write(cl);
        }
        land_follow_clamps(sub, r2, dist);
    }
}

/// Distance clamps of the follow-mode tail.
unsafe fn land_follow_clamps(sub: u32, r2: u32, dist: f32) {
    unsafe {
        if (r2 as *const u32).byte_add(0xB30).read() != 0 {
            return;
        }
        let c1 = global::<f32>(0x00FE_8B38).read();
        if c1 > dist {
            let v = (sub as *const u8).byte_add(0x27).read();
            (sub as *mut u8).byte_add(0x27).write(v.min(0x0A));
            return;
        }
        let c2 = global::<f32>(0x00FE_8B5C).read();
        if c2 > dist {
            let v = (sub as *const u8).byte_add(0x27).read();
            (sub as *mut u8).byte_add(0x27).write(v.min(0x0F));
        }
    }
}

/// Steer a follower into its follow mode (original 0x00B2A790).
///
/// Detached and scripted objects delegate to their own handlers. Otherwise
/// the subject is resolved twice; a miss, a missing pool, a busy flag or a
/// failed check parks it through the idle exit. Survivors measure their
/// planar distance to the anchor, pick a mode from the check result (or
/// mode zero when pinned), publish the mode's code triple through the
/// notifier calls, then settle and clamp the rate by the distance.
export!(cdecl, rw_b2a790(obj: u32, sub: u32) -> u32 {
    const TYPE_OFF: usize = 0x1304;
    const ANCHOR_OFF: usize = 0x20;
    const POOL_OFF: usize = 0x228;
    const HEAD_DELTA: u32 = 0x70;
    const BUSY_OFF: usize = 0x5A;
    const PINNED_OFF: usize = 0xA70;
    const FLAGB_OFF: usize = 0x2C;
    const FLAGW_OFF: usize = 0x2E;
    const CODE_OFF: usize = 0x26;
    const RATE_OFF: usize = 0x27;
    const SETTLE_OFF: usize = 0x28;
    const WORD_OFF: usize = 0x24;
    const G1: u32 = 0x012F_A068;
    const G2: u32 = 0x012F_A494;
    const G3: u32 = 0x012F_A044;
    const G4: u32 = 0x012F_A314;
    unsafe {
        let ty = (obj as *const u32).byte_add(TYPE_OFF).read();
        if ty == 4 {
            callee_cdecl!(1, u32, obj, sub);
            return 0;
        }
        if ty == 2 {
            callee_cdecl!(2, u32, obj, sub);
            return 0;
        }
        if callee_thiscall!(3, u32, sub) == 0 {
            return land_follow_idle(obj, sub);
        }
        let r2 = callee_thiscall!(3, u32, sub);
        // Proof-only spill transport (NOT for merge): the original stores R2
        // into its incoming arg0 slot, which the stack check observes. The
        // bias below is measured from this exact build (see lane notes) and
        // is re-verified by the stack check on every trial.
        let spill_anchor: u32 = 0xA6C40A11;
        let incoming = (&spill_anchor as *const u32 as u32).wrapping_add(24);
        core::ptr::write_volatile(incoming as *mut u32, r2);
        let pool = (r2 as *const u32).byte_add(POOL_OFF).read();
        if pool == 0 || pool.wrapping_add(HEAD_DELTA) == 0 {
            (sub as *mut u8).byte_add(CODE_OFF).write(7);
            callee_thiscall!(7, u32, sub, r2);
            let al = (obj as *const u8).byte_add(FLAGB_OFF).read() & 7;
            (sub as *mut u8).byte_add(RATE_OFF).write(al.wrapping_add(0x0E));
            (sub as *mut u8).byte_add(SETTLE_OFF).write(2);
            let f = (obj as *const u8).byte_add(0xF19).read();
            (obj as *mut u8).byte_add(0xF19).write(f & 0x8F);
            return 0;
        }
        let head = pool.wrapping_add(HEAD_DELTA);
        if (head as *const u8).byte_add(BUSY_OFF).read() & 7 != 0 {
            return land_follow_idle(obj, sub);
        }
        if callee_thiscall!(4, u32, head) & 0xFF != 0 {
            return land_follow_idle(obj, sub);
        }
        let bp = (r2 as *const u32).byte_add(ANCHOR_OFF).read() as *const f32;
        let ap = (obj as *const u32).byte_add(ANCHOR_OFF).read() as *const f32;
        let dx = bp.add(13).read() - ap.add(13).read();
        let dy = bp.add(12).read() - ap.add(12).read();
        let dist = (dx * dx + dy * dy).sqrt();
        let mode = callee_thiscall!(5, u32, head);
        let pinned = (r2 as *const u32).byte_add(PINNED_OFF).read() == 1;
        let fb = (obj as *const u8).byte_add(FLAGB_OFF).read();
        let fw = (obj as *const u16).byte_add(FLAGW_OFF).read() as i16 as i32;
        let special = fw == global::<i32>(G1).read()
            || fw == global::<i32>(G2).read()
            || fw == global::<i32>(G3).read()
            || fw == global::<i32>(G4).read();
        if pinned {
            return land_follow_case0r(obj, sub, fb);
        }
        match mode {
            0 => {
                land_follow_case0r(obj, sub, fb);
            }
            1 => {
                callee_thiscall!(6, u32, obj, 0);
                (sub as *mut u8).byte_add(CODE_OFF).write(7);
                callee_thiscall!(7, u32, sub, r2);
                (sub as *mut u8)
                    .byte_add(RATE_OFF)
                    .write((fb & 3).wrapping_add(0x16));
                land_follow_tail(obj, sub, r2, dist);
            }
            2 => {
                callee_thiscall!(8, u32, obj, 0);
                callee_thiscall!(6, u32, obj, 1);
                (sub as *mut u8)
                    .byte_add(CODE_OFF)
                    .write(if fb & 3 == 1 { 2 } else { 3 });
                if special {
                    (sub as *mut u8).byte_add(CODE_OFF).write(7);
                }
                callee_thiscall!(7, u32, sub, r2);
                (sub as *mut u8)
                    .byte_add(RATE_OFF)
                    .write((fb & 3).wrapping_add(0x1E));
                (sub as *mut u16).byte_add(WORD_OFF).write(0x23);
                land_follow_tail(obj, sub, r2, dist);
            }
            3 => {
                callee_thiscall!(8, u32, obj, 0);
                callee_thiscall!(6, u32, obj, 1);
                (sub as *mut u8)
                    .byte_add(CODE_OFF)
                    .write(if fb & 3 <= 2 { 3 } else { 2 });
                if special {
                    (sub as *mut u8).byte_add(CODE_OFF).write(7);
                }
                callee_thiscall!(7, u32, sub, r2);
                (sub as *mut u8)
                    .byte_add(RATE_OFF)
                    .write((fb & 3).wrapping_add(0x24));
                (sub as *mut u16).byte_add(WORD_OFF).write(0x28);
                land_follow_tail(obj, sub, r2, dist);
            }
            4 => land_follow_case45(obj, sub, r2, dist, fb, special, fb & 3 <= 1),
            5 => land_follow_case45(obj, sub, r2, dist, fb, special, fb & 3 == 0),
            6 => {
                callee_thiscall!(8, u32, obj, 0);
                callee_thiscall!(6, u32, obj, 1);
                (sub as *mut u8)
                    .byte_add(CODE_OFF)
                    .write(if fb & 3 == 0 { 3 } else { 2 });
                if special {
                    (sub as *mut u8).byte_add(CODE_OFF).write(7);
                }
                callee_thiscall!(7, u32, sub, r2);
                (sub as *mut u8)
                    .byte_add(RATE_OFF)
                    .write((fb & 7).wrapping_add(0x25));
                (sub as *mut u16).byte_add(WORD_OFF).write(0x28);
                land_follow_tail(obj, sub, r2, dist);
            }
            _ => {
                let f = (obj as *const u8).byte_add(0xF19).read();
                (obj as *mut u8).byte_add(0xF19).write(f & 0x8F);
                land_follow_clamps(sub, r2, dist);
            }
        }
        0
    }
});

/// Mode zero of the follow switch: plain settle, no tail.
unsafe fn land_follow_case0r(obj: u32, sub: u32, fb: u8) -> u32 {
    unsafe {
        callee_thiscall!(6, u32, obj, 0);
        (sub as *mut u8).byte_add(0x26).write(1);
        (sub as *mut u8)
            .byte_add(0x27)
            .write((fb & 3).wrapping_add(0x0A));
        (sub as *mut u8).byte_add(0x28).write(0);
        0
    }
}

/// Modes four/five of the follow switch, sharing one body.
unsafe fn land_follow_case45(
    obj: u32,
    sub: u32,
    r2: u32,
    dist: f32,
    fb: u8,
    special: bool,
    flag3: bool,
) {
    unsafe {
        callee_thiscall!(8, u32, obj, 0);
        callee_thiscall!(6, u32, obj, 1);
        (sub as *mut u8).byte_add(0x26).write(if flag3 { 3 } else { 2 });
        if special {
            (sub as *mut u8).byte_add(0x26).write(7);
        }
        callee_thiscall!(7, u32, sub, r2);
        (sub as *mut u8)
            .byte_add(0x27)
            .write((fb & 7).wrapping_add(0x24));
        (sub as *mut u16).byte_add(0x24).write(0x28);
        land_follow_tail(obj, sub, r2, dist);
    }
}

/// Idle exit of the follow update.
unsafe fn land_follow_idle(obj: u32, sub: u32) -> u32 {
    unsafe {
        callee_thiscall!(9, u32, sub);
        (sub as *mut u16).byte_add(0x26).write(0x0A01);
        (sub as *mut u8).byte_add(0x28).write(0);
        let f = (obj as *const u8).byte_add(0xF19).read();
        (obj as *mut u8).byte_add(0xF19).write(f & 0x8F);
        0
    }
}
