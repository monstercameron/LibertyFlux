// original: 0x00acf2d0 audio_voice_render
/// Audio voice render through the index chain (original 0xACF2D0).
///
/// `this` is the voice, `a0` the bank object. The bank's state object is
/// resolved through its virtual slot, falling back to the bank's spare
/// pointer; a null state ends the call. Live banks render one of two paths:
/// banks flagged solo resolve a voice index through the global voice table
/// (negative ends the call), fetch the voice record, rotate its matrix by
/// the voice angle via the cosine/sine helpers, and attach the indexed row
/// data; other banks forward the state, the table row, an optional pointer
/// and a count to the batch renderer. Returns the row tail on the solo
/// path, the batch answer on the forward path, else the spare or the
/// negative index.
export!(thiscall, rw_b45_f2d0(this: *mut u8, a0: *mut u8) -> u32 {
    unsafe {
        let vt = *(a0 as *const u32);
        let vslot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0xA0) as *const u32));
        let g = if vslot(a0 as u32) != 0 {
            let r2 = vslot(a0 as u32);
            let vt2 = *(r2 as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(*((vt2 + 0xE0) as *const u32));
            f(r2)
        } else {
            *(a0.add(0x100) as *const u32)
        };
        if g == 0 {
            return 0;
        }
        let table = relocated(0x1295CD8) as i32;
        let i = *(a0.add(0x2E) as *const i16) as i32;
        if *(a0.add(0x1300) as *const u32) == 1 {
            let p1 = *((table.wrapping_add(i.wrapping_mul(4))) as *const u32);
            let p2 = *((p1 + 0xCC) as *const u32);
            let sel = *(this as *const u32);
            let idx = *((p2 + sel.wrapping_mul(4)) as *const i32);
            if idx < 0 {
                return idx as u32;
            }
            let rec = callee_thiscall!(1, u32, a0 as u32, idx as u32) as *mut u8;
            let _: u32 = callee_thiscall!(2, u32, rec as u32);
            let ang = *(this.add(0x7C) as *const u32);
            let c = f32::from_bits(callee_cdecl!(3, u32, ang));
            let s = f32::from_bits(callee_cdecl!(4, u32, ang));
            let e10 = *(rec.add(0x10) as *const f32);
            let e14 = *(rec.add(0x14) as *const f32);
            let e18 = *(rec.add(0x18) as *const f32);
            let e20 = *(rec.add(0x20) as *const f32);
            let e24 = *(rec.add(0x24) as *const f32);
            let e28 = *(rec.add(0x28) as *const f32);
            let t0 = e20 * c;
            let t2 = e14 * c;
            let mut t4 = e24 * s;
            let t1 = e10 * c;
            t4 += t2;
            let mut t2b = s * e28;
            *(rec.add(0x20) as *mut f32) = t0;
            let t0b = e24 * c;
            let t3 = e18 * c;
            let c6 = c * e28;
            *(rec.add(0x24) as *mut f32) = t0b;
            let mut t5 = e20 * s;
            *(rec.add(0x28) as *mut f32) = c6;
            let mut r0 = t0;
            t5 += t1;
            let t1b = e10 * s;
            t2b += t3;
            r0 -= t1b;
            *(rec.add(0x20) as *mut f32) = r0;
            let t1c = e14 * s;
            let mut r0b = t0b;
            r0b -= t1c;
            *(rec.add(0x24) as *mut f32) = r0b;
            let t1d = e18 * s;
            let mut r0c = c6;
            r0c -= t1d;
            *(rec.add(0x28) as *mut f32) = r0c;
            *(rec.add(0x10) as *mut f32) = t5;
            *(rec.add(0x14) as *mut f32) = t4;
            *(rec.add(0x18) as *mut f32) = t2b;
            let r5 = callee_thiscall!(5, u32, a0 as u32);
            let base = *(r5 as *const u32);
            let off = (idx as u32).wrapping_mul(0xE0);
            *(rec.add(0x30) as *mut u32) = *((base + off + 0x20) as *const u32);
            *(rec.add(0x34) as *mut u32) = *((base + off + 0x24) as *const u32);
            let tail = *((base + off + 0x28) as *const u32);
            *(rec.add(0x38) as *mut u32) = tail;
            tail
        } else {
            let count = *(a0.add(0xF84) as *const u32);
            let ptr = if (count as i32) > 0 {
                *(a0.add(0xF80) as *const u32)
            } else {
                0
            };
            let p1 = *((table.wrapping_add(i.wrapping_mul(4))) as *const u32);
            let p2 = *((p1 + 0xCC) as *const u32);
            let arg0 = if vslot(a0 as u32) != 0 {
                let r2 = vslot(a0 as u32);
                let vt2 = *(r2 as *const u32);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(*((vt2 + 0xE0) as *const u32));
                f(r2)
            } else {
                *(a0.add(0x100) as *const u32)
            };
            callee_thiscall!(6, u32, this as u32, arg0, p2, ptr, count)
        }
    }
});
