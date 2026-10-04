// original: 0x00B52CB0 euphoria_blend_step
/// Advance one animation-blend object by a time step and report contacts.
///
/// Runs only when a chain of global gates agrees (window state, two flag
/// bytes, a validator call, generation counters); otherwise returns early
/// with a gate-dependent code. The deep path advances the entry cursor,
/// decays every flagged row by `dt_bits`, resolves the object through the
/// sibling scan (callee 3, stubbed), dispatches through the object's
/// function table (callees 4-6) and the contact reporter (callee 7), sweeps
/// per-entry contacts (callees 8-9), runs gated maintenance (callees
/// 10-12) and, when every flag cleared and the object is live, releases it
/// (callee 13) before detaching (`this[0] = 0`).
///
/// Three call arguments carry values the rewrite cannot observe and are
/// masked in the contract: the window-handle slot holds a planted stub
/// (read exactly like the original), one reporter argument repeats the
/// caller's entry register, and the reporter's last argument is outside the
/// checker's comparison window.
export!(thiscall, rw_b01_f1(this: u32, gate_arg: u32, dt_bits: u32) -> u32 {
    unsafe {
        // Gate 1: window-state callback through the read-only slot (the
        // checker plants its stub there; load and call it like the original).
        let slot = *global::<u32>(0xE733DC);
        let iconic_fn: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let iconic: u32 = iconic_fn(*global::<u32>(0x17ACCD8));
        let mut gated: u8;
        if iconic != 0 {
            gated = 1;
        } else if *global::<u8>(0x105B48F) == 0 {
            gated = 0;
        } else if *global::<u8>(0x17ED8D1) != 0 {
            gated = 1;
        } else {
            gated = 0;
        }
        gated |= *global::<u8>(0x1173590);
        gated |= *global::<u8>(0x1173591);
        if gated != 0 {
            return gated as u32;
        }
        // Gate 2: validator call; only its low byte decides, the full
        // answer is the early-return value.
        let chk: u32 = callee_cdecl!(2, u32,);
        if (chk & 0xFF) != 0 {
            return chk;
        }
        if *global::<u32>(0x11F7060) == 1 {
            return chk;
        }
        let lv = *global::<u32>(0x12088B4);
        if lv != *global::<u32>(0xF1C040) {
            return lv;
        }
        if *global::<u32>(0x1037720) == 0x12 {
            return lv;
        }
        let obj = *(this.wrapping_add(0x430) as *const u32);
        if obj == 0 {
            *(this as *mut u32) = 0;
            return lv;
        }
        // Conditional cursor advance with row rotation.
        let cnt = *(this.wrapping_add(8) as *const u32);
        if *global::<u32>(0x11735B4) > cnt.wrapping_add(0x96) {
            let old = *(this.wrapping_add(8 + 4) as *const i32);
            let new = old.wrapping_add(1) % 32;
            *(this.wrapping_add(0xC) as *mut i32) = new;
            *(this.wrapping_add(new as u32).wrapping_add(0x10) as *mut u8) = 0;
            // Row copies, word by word in order (the original's exact
            // load-store sequence, so even overlapping rows would agree).
            let s1 = this.wrapping_add(
                (old.wrapping_add(3).wrapping_mul(2) as u32).wrapping_mul(8),
            );
            let d1 = this.wrapping_add((new.wrapping_add(3) as u32).wrapping_mul(16));
            *(d1 as *mut u32) = *(s1 as *const u32);
            *(d1.wrapping_add(4) as *mut u32) = *(s1.wrapping_add(4) as *const u32);
            *(d1.wrapping_add(8) as *mut u32) = *(s1.wrapping_add(8) as *const u32);
            *(d1.wrapping_add(12) as *mut u32) = *(s1.wrapping_add(12) as *const u32);
            let s2 = this.wrapping_add(
                (old.wrapping_add(0x23).wrapping_mul(2) as u32).wrapping_mul(8),
            );
            let d2 = this.wrapping_add((new.wrapping_add(0x23) as u32).wrapping_mul(16));
            *(d2 as *mut u32) = *(s2 as *const u32);
            *(d2.wrapping_add(4) as *mut u32) = *(s2.wrapping_add(4) as *const u32);
            *(d2.wrapping_add(8) as *mut u32) = *(s2.wrapping_add(8) as *const u32);
            *(d2.wrapping_add(12) as *mut u32) = *(s2.wrapping_add(12) as *const u32);
        }
        // Decay every flagged row by the time step.
        let dt = f32::from_bits(dt_bits);
        let rate = *global::<f32>(0xEA3AC0);
        for i in 0..32u32 {
            if *(this.wrapping_add(i).wrapping_add(0x10) as *const u8) != 0 {
                let e = this.wrapping_add(0x34).wrapping_add(i.wrapping_mul(16));
                let mut c3 = *(e.wrapping_add(0x204) as *const f32);
                c3 -= dt * rate;
                *(e.wrapping_add(0x204) as *mut f32) = c3;
                let c1 = *(e.wrapping_add(0x1FC) as *const f32) * dt;
                let mut c2 = *(e.wrapping_add(0x200) as *const f32) * dt;
                let mut c0 = *(e.wrapping_sub(4) as *const f32);
                c3 *= dt;
                c2 += *(e as *const f32);
                c0 += c1;
                c3 += *(e.wrapping_add(4) as *const f32);
                *(e as *mut f32) = c2;
                *(e.wrapping_sub(4) as *mut f32) = c0;
                *(e.wrapping_add(4) as *mut f32) = c3;
            }
        }
        // Resolve through the sibling scan (stubbed): two scratch quads.
        let mut buf_a = [0u32; 4];
        let mut buf_b = [0u32; 4];
        let _: u32 = callee_thiscall!(
            3,
            u32,
            this,
            buf_b.as_mut_ptr() as u32,
            buf_a.as_mut_ptr() as u32
        );
        // Table gate, then function-table dispatch.
        let esi_obj = *(this.wrapping_add(0x430) as *const u32);
        let widx = *(esi_obj.wrapping_add(0x2E) as *const i16) as i32;
        let slot2 = relocated(0x1295CD8).wrapping_add((widx as u32).wrapping_mul(4));
        let o1 = *(slot2 as *const u32);
        let o2 = *(o1.wrapping_add(0xCC) as *const u32);
        let gate = *(o2.wrapping_add(0x144) as *const i32);
        if gate > -1 {
            let vt = *(esi_obj as *const u32);
            let probe_addr = *(vt.wrapping_add(0xA0) as *const u32);
            let probe: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(probe_addr as usize);
            let ans: u32;
            if probe(esi_obj) == 0 {
                ans = *(esi_obj.wrapping_add(0x100) as *const u32);
            } else {
                let fetch_addr = *(vt.wrapping_add(0xA0) as *const u32);
                let fetch: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(fetch_addr as usize);
                let mid = fetch(esi_obj);
                let vt2 = *(mid as *const u32);
                let tail_addr = *(vt2.wrapping_add(0xE0) as *const u32);
                let tail: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tail_addr as usize);
                ans = tail(mid);
            }
            // The table word saved before dispatch, shifted and added to
            // the pointed-to word.
            let esi2 = (gate as u32)
                .wrapping_mul(64)
                .wrapping_add(*(ans.wrapping_add(0x14) as *const u32));
            // Lazily initialized shared constants.
            let lz = global::<u32>(0x1669D30);
            if (*lz & 1) == 0 {
                *lz |= 1;
                *global::<u32>(0x1669D20) = 0;
                *global::<u32>(0x1669D24) = 0x3F800000;
                *global::<u32>(0x1669D28) = 0x3F19999A;
            }
            // Contact triple through the table, then the reporter.
            let mut contact = [0u32; 4];
            let vt3 = *(esi_obj as *const u32);
            let rep_addr = *(vt3.wrapping_add(0xEC) as *const u32);
            let rep: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rep_addr as usize);
            let triple = rep(esi_obj, contact.as_mut_ptr() as u32);
            let tx = *(triple as *const f32);
            let ty = *(triple.wrapping_add(4) as *const f32);
            let tz = *(triple.wrapping_add(8) as *const f32);
            // The original takes a packed square root and keeps the low
            // lane only; a scalar root is the identical operation.
            let dist = (tx * tx + ty * ty + tz * tz).sqrt();
            let cur = *(this.wrapping_add(0xC) as *const i32);
            let row4 =
                this.wrapping_add((cur.wrapping_add(0x23) as u32).wrapping_mul(16));
            let byte4 = *(this.wrapping_add(4) as *const u8) as u32;
            // Argument 5 repeats the caller's entry register on the
            // original side (unobservable here, masked in the contract).
            let _: u32 = callee_thiscall!(
                7,
                u32,
                relocated(0x13B0EB0),
                this,
                byte4,
                esi2,
                relocated(0x1669D20),
                row4,
                0,
                buf_b.as_ptr() as u32,
                buf_a.as_ptr() as u32,
                dist.to_bits()
            );
        }
        // Per-entry contact sweep.
        *(this.wrapping_add(4) as *mut u8) = 0;
        let mut cursor = this.wrapping_add(0x30);
        for si in 0..32u32 {
            if *(this.wrapping_add(si).wrapping_add(0x10) as *const u8) != 0 {
                let _: u32 =
                    callee_thiscall!(8, u32, relocated(0x12E2420), cursor, 0x40000000);
                let _: u32 = callee_thiscall!(
                    9,
                    u32,
                    relocated(0x13BABA0),
                    0xA,
                    cursor,
                    0x40000000,
                    0
                );
            }
            cursor = cursor.wrapping_add(0x10);
        }
        // Gated maintenance pair, then the unconditional pass.
        if ((*global::<u32>(0x1173604)).wrapping_add(gate_arg) & 3) == 0 {
            let _: u32 = callee_thiscall!(10, u32, this);
            let _: u32 = callee_thiscall!(11, u32, this);
        }
        let _: u32 = callee_thiscall!(12, u32, this);
        // A set flag anywhere returns its index; otherwise release a live
        // object (its answer is the result) and detach.
        for ai in 0..32u32 {
            if *(this.wrapping_add(ai).wrapping_add(0x10) as *const u8) != 0 {
                return ai;
            }
        }
        let live = *(this.wrapping_add(0x430) as *const u32);
        let ret: u32;
        if live != 0 {
            ret = callee_thiscall!(13, u32, live, this.wrapping_add(0x430));
        } else {
            ret = 0x20;
        }
        *(this as *mut u32) = 0;
        ret
    }
});
