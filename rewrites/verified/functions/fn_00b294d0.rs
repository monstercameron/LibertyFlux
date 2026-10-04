// original: 0x00b294d0 JF05_BA
/// Run the per-slot update pass over the slot table.
///
/// After three gate checks and a mode gate (an enable call plus a name
/// match against three known modes), visits all 24 slots: skips empty or
/// inhibited slots (reporting dead ones), advances each live slot's
/// progress by its timed rate subject to a cap log and a ceiling, clamps
/// the progress into range, seeks the slot cursor to the progress
/// position, stages the slot record, notifies the object, runs the
/// object update and the ratio update, notifies each item, and when the
/// staged point has drifted too far, re-anchors the object and follows
/// up through its state. Returns the final table pointer, or the gate or
/// match value on the early exits.
export!(cdecl, rw_00b294d0() -> u32 {
    unsafe {
        const G_GATE1: u32 = 0x011F7060;
        const G_GATE2A: u32 = 0x012088B4;
        const G_GATE2B: u32 = 0x00F1C040;
        const G_GATE3: u32 = 0x01037720;
        const G_NAME: u32 = 0x01295764;
        const G_DT_A: u32 = 0x011735B4;
        const G_DT_B: u32 = 0x011735B8;
        const LIT_JF05: u32 = 0x00EAC614;
        const LIT_Y1: u32 = 0x00EAC61C;
        const LIT_E2: u32 = 0x00EAC624;
        const LIT_STEP: u32 = 0x00EAC630;
        const T_IDS: u32 = 0x01657650;
        const T_BASES: u32 = 0x016576B0;
        const T_CURSORS: u32 = 0x01657710;
        const T_LIMITS: u32 = 0x01657770;
        const T_PROGRESS: u32 = 0x016577D0;
        const T_RATES: u32 = 0x01657830;
        const T_VALID: u32 = 0x01657890;
        const T_CLAMP: u32 = 0x016578A8;
        const T_NOTIMED: u32 = 0x016578C0;
        const T_NOREF: u32 = 0x016578D8;
        const T_TRIPLES: u32 = 0x0165FBA0;
        const K_CAP: u32 = 0x00FE8C2C;
        const K_FLOOR: u32 = 0x00E8E300;
        const K_DIST: u32 = 0x00FE8AD8;
        const ENTRY_EAX: u32 = 0xA1B2C3D4;
        let enabled: extern "cdecl" fn() -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let log_step: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let finish_slot: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let stage: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(4) as usize);
        let update: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(6) as usize);
        let ratio_upd: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let notify_item: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);
        let anchor: extern "cdecl" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(10) as usize);
        let follow: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(11) as usize);
        let settle: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(12) as usize);
        let report_dead: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(13) as usize);
        let dead_dt: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(99) as usize);

        let u2f = |v: u32| -> f32 { (v as f64) as f32 };
        // Bytewise strcmp returning -1/0/1, same read order as the original.
        let strcmp2 = |a: u32, b: u32| -> i32 {
            let mut i = 0u32;
            loop {
                let x = *((a.wrapping_add(i)) as *const u8);
                let y = *((b.wrapping_add(i)) as *const u8);
                if x != y {
                    return if x < y { -1 } else { 1 };
                }
                if x == 0 {
                    return 0;
                }
                i = i.wrapping_add(1);
            }
        };

        // Gates. The first exit preserves entry EAX, which the contract
        // pins to ENTRY_EAX (scripted-value transport, documented).
        if *(global::<u32>(G_GATE1)) == 1 {
            return ENTRY_EAX;
        }
        let g2a = *(global::<u32>(G_GATE2A));
        if g2a != *(global::<u32>(G_GATE2B)) {
            return g2a;
        }
        if *(global::<u32>(G_GATE3)) == 0x12 {
            return g2a;
        }
        let name = relocated(G_NAME);
        if (enabled() as u8) != 0 {
            if strcmp2(name, relocated(LIT_JF05)) == 0 {
                // fall through to the slot loop
            } else if strcmp2(name, relocated(LIT_Y1)) == 0 {
                // fall through to the slot loop
            } else {
                let r = strcmp2(name, relocated(LIT_E2));
                if r != 0 {
                    return r as u32;
                }
            }
        }
        let cap: f32 = *global::<f32>(K_CAP);
        let maxv_cap: f32 = *global::<f32>(K_FLOOR);
        let dist_lim: f32 = *global::<f32>(K_DIST);
        let step_lit = relocated(LIT_STEP);
        for slot in 0..24u32 {
            let triple = relocated(T_TRIPLES).wrapping_add(slot.wrapping_mul(16));
            if *((relocated(T_VALID).wrapping_add(slot)) as *const u8) == 0 {
                continue;
            }
            let id_cell =
                (relocated(T_IDS).wrapping_add(slot.wrapping_mul(4))) as *const u32;
            if (slot as i32) < 0 {
                // Dead: the slot index never goes negative. Mirrors the
                // original's negative-index early-out without its checks.
                if *id_cell != 0 {
                    continue;
                }
            } else {
                let obj = *id_cell;
                if obj == 0 || *((obj.wrapping_add(0x11A)) as *const u8) & 1 != 0
                {
                    report_dead(slot);
                    continue;
                }
            }
            if *((relocated(T_NOREF).wrapping_add(slot)) as *const u8) != 0 {
                continue;
            }
            let prog_cell =
                (relocated(T_PROGRESS).wrapping_add(slot.wrapping_mul(4))) as *mut f32;
            if *((relocated(T_NOTIMED).wrapping_add(slot)) as *const u8) == 0 {
                if (slot as i32) < 0 {
                    // Dead: negative slot timing. Overwrites progress.
                    let dt = dead_dt(slot, 0);
                    *prog_cell = u2f(dt);
                } else {
                    let a = *(global::<u32>(G_DT_A));
                    let b = *(global::<u32>(G_DT_B));
                    let dtf = u2f(a.wrapping_sub(b));
                    let rate = *((relocated(T_RATES)
                        .wrapping_add(slot.wrapping_mul(4)))
                        as *const f32);
                    let mut r = dtf * rate;
                    if r > cap {
                        log_step(step_lit);
                    }
                    // Clamp from above (the original keeps r only when the
                    // limit is strictly above it, else takes the limit).
                    if !(maxv_cap > r) {
                        r = maxv_cap;
                    }
                    *prog_cell = *prog_cell + r;
                }
            }
            let base = *((relocated(T_BASES).wrapping_add(slot.wrapping_mul(4)))
                as *const u32);
            let cursor_cell = (relocated(T_CURSORS)
                .wrapping_add(slot.wrapping_mul(4)))
                as *mut u32;
            let limit = *((relocated(T_LIMITS).wrapping_add(slot.wrapping_mul(4)))
                as *const u32);
            let tail = *((limit.wrapping_add(base).wrapping_sub(0x20)) as *const u32);
            let mut prog = *prog_cell;
            let clamp_flag =
                *((relocated(T_CLAMP).wrapping_add(slot)) as *const u8) != 0;
            if prog < 0.0 {
                if clamp_flag {
                    prog = u2f(tail);
                    *prog_cell = prog;
                } else {
                    prog = 0.0;
                    *prog_cell = prog;
                }
            }
            let maxv = u2f(tail);
            prog = *prog_cell;
            if prog >= maxv {
                if clamp_flag {
                    *prog_cell = 0.0;
                    *cursor_cell = 0;
                    prog = 0.0;
                } else {
                    *prog_cell = maxv;
                    prog = maxv;
                    finish_slot(*id_cell);
                }
            }
            let obj = *id_cell;
            if obj == 0 {
                // Dead in practice: a null object continued above, and no
                // store in between can null it. Mirrors the recheck.
                continue;
            }
            let mut cursor = *cursor_cell;
            let mut edi = base.wrapping_add(cursor);
            let first = u2f(*((edi.wrapping_add(0x20)) as *const u32));
            let mut ebx = edi.wrapping_add(0x20);
            if prog > first {
                loop {
                    if ebx >= base.wrapping_add(limit) {
                        break;
                    }
                    cursor = cursor.wrapping_add(0x20);
                    *cursor_cell = cursor;
                    let v = u2f(*((ebx.wrapping_add(0x20)) as *const u32));
                    ebx = ebx.wrapping_add(0x20);
                    edi = edi.wrapping_add(0x20);
                    if prog > v {
                        continue;
                    }
                    break;
                }
            }
            let back = u2f(*(edi as *const u32));
            if back > prog {
                loop {
                    if edi <= base {
                        break;
                    }
                    cursor = cursor.wrapping_sub(0x20);
                    *cursor_cell = cursor;
                    let v = u2f(*((edi.wrapping_sub(0x20)) as *const u32));
                    edi = edi.wrapping_sub(0x20);
                    ebx = ebx.wrapping_sub(0x20);
                    if v > prog {
                        continue;
                    }
                    break;
                }
            }
            let mut frame = [0u32; 8];
            let extra = *((obj.wrapping_add(0x20)) as *const u32);
            stage(frame.as_mut_ptr() as u32, extra);
            let vt = *(obj as *const u32);
            let prime: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                *((vt.wrapping_add(0x10C)) as *const u32) as usize,
            );
            let mut vbuf = [0u32; 3];
            prime(obj, vbuf.as_mut_ptr() as u32);
            update(obj, edi, 0, triple);
            let e0 = *(edi as *const u32);
            let e1 = *(ebx as *const u32);
            let ratio = (prog - u2f(e0)) / u2f(e1.wrapping_sub(e0));
            ratio_upd(obj, ebx, ratio.to_bits(), triple);
            let items_base = *((obj.wrapping_add(0xF80)) as *const u32);
            let mut i = 0u32;
            loop {
                let count = *((obj.wrapping_add(0xF84)) as *const i32);
                if !((i as i32) < count) {
                    break;
                }
                let this2 = if (i as i32) < count {
                    items_base.wrapping_add(i.wrapping_mul(0x170))
                } else {
                    0
                };
                notify_item(this2, frame.as_mut_ptr() as u32, extra);
                i = i.wrapping_add(1);
            }
            let dx = *((extra.wrapping_add(0x30)) as *const f32)
                - f32::from_bits(vbuf[0]);
            let dy = *((extra.wrapping_add(0x34)) as *const f32)
                - f32::from_bits(vbuf[1]);
            let dz = *((extra.wrapping_add(0x38)) as *const f32)
                - f32::from_bits(vbuf[2]);
            let dist = (dy * dy + dx * dx + dz * dz).sqrt();
            if dist > dist_lim {
                let reanchor: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(
                        *((vt.wrapping_add(0x190)) as *const u32) as usize
                    );
                reanchor(obj);
                if *((obj.wrapping_add(0x1304)) as *const u32) == 1 {
                    let sx = *((obj.wrapping_add(0x2E)) as *const i16) as i32;
                    anchor(
                        obj,
                        extra,
                        sx as u32,
                        *((obj.wrapping_add(0x38)) as *const u32),
                        0,
                        0,
                        0,
                        0x3F800000,
                        0x3F800000,
                        1,
                        0,
                    );
                }
            }
            if *((obj.wrapping_add(0x1304)) as *const u32) == 3 {
                follow(obj);
            }
            let anchor_pt = extra.wrapping_add(0x30);
            settle(anchor_pt, 0x40A00000, obj, 1);
        }
        relocated(T_TRIPLES).wrapping_add(24 * 16)
    }
});
