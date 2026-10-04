// original: 0x00c8b0c0 audio_mix_grid_build
/// Audio mix-grid builder (original 0x00C8B0C0, thiscall/3).
///
/// Gated on the source flags (`[src+0x28] & 0x3C0 == 0x80`) and the selector
/// argument (`a == 0x19`); either mismatch clears `[this+8]` and returns 0.
/// Otherwise it samples two direction vectors through the source's virtual
/// table (slots `0x60`/`0x64`), notifies a hook callee, then fills a 5x5 grid
/// of 32-byte rows at `[this+4]` by calling a per-cell solver 25 times and
/// accumulating the row magnitudes. The tail normalizes six summary fields on
/// `this` by the accumulated magnitude and returns the child vector pointer.
///
/// Argument `b` is accepted but never read by the original. The float
/// operation order below matches the original instruction by instruction, so
/// results agree bit-exactly (SSE scalar semantics, no FMA).
export!(thiscall, rw_rb28_f1(this_ptr: *mut u8, a: u32, _b: u32, src: *mut u8) -> u32 {
    unsafe {
        // Gate 1 (checked before the child chain is touched): source flags.
        if (*(src.add(0x28) as *const u32) & 0x3C0) != 0x80 {
            *(this_ptr.add(8) as *mut u32) = 0;
            return 0;
        }
        let child = *(src.add(0xDC8) as *const u32);
        let vec = *((child as *const u8).add(0x100) as *const u32);
        // Gate 2: selector. The chain above is read even when this fails.
        if a != 0x19 {
            *(this_ptr.add(8) as *mut u32) = 0;
            return 0;
        }
        // Two virtual direction samples from the source object.
        let vtable = *(src as *const u32);
        let sample0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtable as *const u8).add(0x60) as *const u32) as usize);
        let p0 = sample0(src as u32);
        let v0x = f32::from_bits(*(p0 as *const u32));
        let v0y = f32::from_bits(*((p0 as *const u8).add(4) as *const u32));
        let v0z = f32::from_bits(*((p0 as *const u8).add(8) as *const u32));
        let sample1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtable as *const u8).add(0x64) as *const u32) as usize);
        let p1 = sample1(src as u32);
        let v1x = f32::from_bits(*(p1 as *const u32));
        let v1y = f32::from_bits(*((p1 as *const u8).add(4) as *const u32));
        // Mixed products of the child vector with the two samples.
        let vx = f32::from_bits(*((vec as *const u8).add(0x00) as *const u32));
        let vy = f32::from_bits(*((vec as *const u8).add(0x04) as *const u32));
        let vz = f32::from_bits(*((vec as *const u8).add(0x08) as *const u32));
        let c1 = *global::<f32>(0x00FE8D7C);
        let c2 = *global::<f32>(0x00FE87E4);
        let t4 = vx * v1y - vy * v0y;
        let t3 = vz * v1x - vz * v0x;
        let s0 = t3 * c1;
        let s3 = t3 * c2;
        let s2 = t4 * c1;
        let s4 = t4 * c2;
        let mut acc = 0.0f32;
        // Notification hook over scratch; its answer is ignored.
        let mut probe = [0u32; 8];
        callee_thiscall!(2, u32, probe.as_mut_ptr() as u32);
        // 5x5 solver grid over the output rows.
        let c3 = *global::<f32>(0x00FE8830);
        let vz2 = f32::from_bits(*((vec as *const u8).add(0x0C) as *const u32));
        let vec_10 = f32::from_bits(*((vec as *const u8).add(0x10) as *const u32));
        let arr = *(this_ptr.add(4) as *const u32) as *mut u8;
        let mut k = 0u32;
        let mut row = 0usize;
        'outer: for outer in 0..5u32 {
            let cur2 = outer as f32 * s4 + s2;
            for inner in 0..5u32 {
                let cur1 = inner as f32 * s3 + s0;
                let mut buf_a = [cur1, cur2, v0z];
                let mut buf_b = [cur1, cur2, vz2];
                let mut buf_c = [0u32; 7];
                let ok: u32 = callee_cdecl!(
                    3, u32, src as u32,
                    buf_a.as_mut_ptr() as u32,
                    buf_b.as_mut_ptr() as u32,
                    buf_c.as_mut_ptr() as u32,
                    1, 0, 0
                );
                if (ok & 0xFF) != 0 {
                    let o0 = f32::from_bits(buf_c[4]);
                    let o1 = f32::from_bits(buf_c[5]);
                    let o2 = f32::from_bits(buf_c[6]);
                    let d = o2 - vec_10;
                    *(arr.add(row) as *mut f32) = (o0 + cur1) * c3;
                    *(arr.add(row + 4) as *mut f32) = (o1 + cur2) * c3;
                    *(arr.add(row + 8) as *mut f32) = (vz2 + d) * c3;
                    // fabs via bit mask, exactly the original's ANDPS.
                    let m = f32::from_bits((vz2 - d).to_bits() & 0x7FFF_FFFF) * c3;
                    *(arr.add(row + 0x10) as *mut f32) = m;
                    acc += m;
                } else {
                    *(arr.add(row + 0x10) as *mut f32) = 0.0;
                }
                k += 1;
                row += 0x20;
                // Never fires (row caps at 0x320 after 25 cells); kept for
                // fidelity with the original's guard.
                if row > 0x320 {
                    break 'outer;
                }
            }
        }
        // Normalize the summary fields by the accumulated magnitude.
        let c4 = *global::<f32>(0x00FE88E8);
        let g = *global::<f32>(0x00E99B0C);
        let scale = c4 / acc;
        let n = if k < 0x19 { k & 0xFFFF } else { 0x19 };
        *(this_ptr.add(8) as *mut u16) = n as u16;
        *(this_ptr.add(0xA) as *mut u16) = n as u16;
        let s120 = f32::from_bits(*(src.add(0x120) as *const u32));
        let vec_14 = f32::from_bits(*((vec as *const u8).add(0x14) as *const u32));
        let ch_9c = f32::from_bits(*((child as *const u8).add(0x9C) as *const u32));
        let vec_30 = f32::from_bits(*((vec as *const u8).add(0x30) as *const u32));
        let vec_34 = f32::from_bits(*((vec as *const u8).add(0x34) as *const u32));
        let vec_38 = f32::from_bits(*((vec as *const u8).add(0x38) as *const u32));
        *(this_ptr as *mut f32) = s120 * g * scale;
        *(this_ptr.add(0x0C) as *mut f32) = vec_14 * g * scale;
        *(this_ptr.add(0x10) as *mut f32) = ch_9c * g * scale;
        *(this_ptr.add(0x14) as *mut f32) = vec_30 * g * scale;
        *(this_ptr.add(0x18) as *mut f32) = vec_34 * g * scale;
        *(this_ptr.add(0x1C) as *mut f32) = vec_38 * g * scale;
        // The original falls through with the child vector pointer in EAX.
        vec
    }
});
