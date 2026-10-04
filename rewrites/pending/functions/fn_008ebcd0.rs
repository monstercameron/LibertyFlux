// original: 0x008ebcd0 radial_record_query
/// Radius query over grouped packed records with a cached result table.
///
/// Scans 64 record groups (each a count plus a base pointer reached through
/// `obj`), keeps the records that pass the flag filters and lie within
/// `radius` of the query point `q` (2D distance over the packed x/y words),
/// and records their ids in a global 64-entry table. Past 64 hits a
/// random-replacement draw keeps the table bounded; afterwards the table is
/// shuffled with 32 random swaps. A repeat call with unchanged inputs skips
/// the scan and reuses the table. Finally the table is walked from the
/// cursor and the first live entry's position and id are written to
/// `out_pos`/`out_id` (1 returned), or -1/0 when no entry is live.
///
/// Flag words `f3`/`f5`/`f6`/`f7` (low bytes significant) select record
/// variants; `a4` is unused. Only the low result byte is significant.
export!(thiscall, rw_008ebcd0(obj: u32, q: u32, radius: u32, f3: u32, _a4: u32,
        f5: u32, f6: u32, f7: u32, out_pos: u32, out_id: u32) -> u8 {
    #[inline(always)]
    fn cvttss2si(x: f32) -> i32 {
        // Exact cvttss2si: truncate toward zero, out-of-range and NaN give
        // 0x80000000 (Rust `as` would saturate instead).
        if x.is_nan() || x >= 2147483648.0 || x <= -2147483648.0 {
            i32::MIN
        } else {
            x as i32
        }
    }
    unsafe {
        const G_FILL: u32 = 0x01176E4C;
        const G_CURSOR: u32 = 0x01176E50;
        const G_TOTAL: u32 = 0x01176E54;
        const G_RADIUS: u32 = 0x01176E58;
        const G_VALID: u32 = 0x01176E5C;
        const G_QX: u32 = 0x0117E6B0;
        const G_QY: u32 = 0x0117E6B4;
        const G_QZ: u32 = 0x0117E6B8;
        const G_QW: u32 = 0x0117E6BC;
        const TABLE: u32 = 0x01179690;
        const XY_SCALE: f32 = 0.125;
        const Z_SCALE: f32 = 0.015625;
        const INV_32768: f32 = f32::from_bits(0x3800_0000);
        const RAND_SCALE: f32 = f32::from_bits(0x3800_0100);

        let table = lf_checker_rt::relocated(TABLE);
        let rad = f32::from_bits(radius);

        // Cache probe: reuse the table when the query is unchanged.
        let cached = {
            let fill = *global::<i32>(G_FILL);
            let cursor = *global::<i32>(G_CURSOR);
            if fill <= cursor {
                false
            } else if *(q as *const f32) != *global::<f32>(G_QX) {
                false
            } else if *((q as *const f32).add(1)) != *global::<f32>(G_QY) {
                false
            } else if *((q as *const f32).add(2)) != *global::<f32>(G_QZ) {
                false
            } else if rad != *global::<f32>(G_RADIUS) {
                false
            } else {
                *global::<u32>(G_VALID) == 1
            }
        };

        if !cached {
            // Rescan: publish the query, reset the table state.
            let qx = *(q as *const f32);
            let qy = *((q as *const f32).add(1));
            let qz = *((q as *const f32).add(2));
            let qw = *((q as *const f32).add(3));
            *global::<f32>(G_QX) = qx;
            *global::<f32>(G_QY) = qy;
            *global::<f32>(G_QZ) = qz;
            *global::<f32>(G_QW) = qw;
            *global::<f32>(G_RADIUS) = rad;
            *global::<u32>(G_TOTAL) = 0;
            *global::<u32>(G_CURSOR) = 0;
            *global::<u32>(G_FILL) = 0;
            *global::<u32>(G_VALID) = 1;

            let mut total: u32 = 0;
            let mut fill: i32 = 0;
            let mut slot = obj.wrapping_add(0xB04);
            for _ in 0..0x40u32 {
                let base = *((slot.wrapping_sub(0x300)) as *const u32);
                if base != 0 {
                    let count = *(slot as *const i32);
                    let mut i: i32 = 0;
                    while i < count {
                        let rec = base.wrapping_add((i as u32).wrapping_mul(0x20));
                        i += 1;
                        if *((rec.wrapping_add(0x1C)) as *const u8) & 0xF0 != 0 {
                            continue;
                        }
                        let b1e = *((rec.wrapping_add(0x1E)) as *const u8);
                        if b1e & 0x0F != 2 {
                            continue;
                        }
                        let cl = *((rec.wrapping_add(0x1F)) as *const u8);
                        let al = (cl >> 1) & 1;
                        if al != u8::from((f3 as u8) != 0) {
                            continue;
                        }
                        if (f5 as u8) != 0
                            && *((rec.wrapping_add(0x1B)) as *const u8) & 0xE0 != 0
                        {
                            continue;
                        }
                        if (f6 as u8) != 0 && (cl & 1) != 0 && al == 0 {
                            continue;
                        }
                        if (f7 as u8) != 0 && (b1e as i8) < 0 {
                            continue;
                        }
                        let dx = *((rec.wrapping_add(0x14)) as *const i16) as f32
                            * XY_SCALE
                            - qx;
                        let dy = *((rec.wrapping_add(0x16)) as *const i16) as f32
                            * XY_SCALE
                            - qy;
                        let dist = (dy * dy + dx * dx).sqrt();
                        if dist >= rad {
                            continue;
                        }
                        total = total.wrapping_add(1);
                        *global::<u32>(G_TOTAL) = total;
                        let id = *((rec.wrapping_add(8)) as *const u32);
                        if fill < 0x40 {
                            *(table.wrapping_add((fill as u32).wrapping_mul(8))
                                as *mut u32) = id;
                            fill += 1;
                            *global::<u32>(G_FILL) = fill as u32;
                        } else {
                            // Random replacement once the table is full.
                            let r1: u32 = callee_cdecl!(1, u32,);
                            let seen = *global::<u32>(G_TOTAL);
                            let keep = 64.0f32 / (seen as f32);
                            let draw = (r1 as i32) as f32 * RAND_SCALE;
                            if keep > draw {
                                let r2: u32 = callee_cdecl!(1, u32,);
                                let idx = cvttss2si(
                                    ((r2 & 0xFFFF) as f32) * INV_32768 * -64.0,
                                );
                                let cell = table.wrapping_sub(
                                    (idx as u32).wrapping_mul(8),
                                );
                                *(cell as *mut u32) = id;
                            }
                        }
                    }
                }
                slot = slot.wrapping_add(4);
            }

            // Shuffle the table with 32 random swaps.
            let mut live = *global::<i32>(G_FILL);
            if live > 0 {
                for _ in 0..0x200u32 {
                    let bound = *global::<i32>(G_FILL);
                    let r1: u32 = callee_cdecl!(1, u32,);
                    let i1 = cvttss2si(
                        ((r1 & 0xFFFF) as f32) * INV_32768 * (live as f32),
                    );
                    let r2: u32 = callee_cdecl!(1, u32,);
                    let i2 = cvttss2si(
                        ((r2 & 0xFFFF) as f32) * INV_32768 * (bound as f32),
                    );
                    let a = table.wrapping_add((i1 as u32).wrapping_mul(8));
                    let b = table.wrapping_add((i2 as u32).wrapping_mul(8));
                    let t0 = *(a as *const u32);
                    let t1 = *((a.wrapping_add(4)) as *const u32);
                    *(a as *mut u32) = *(b as *const u32);
                    *(a.wrapping_add(4) as *mut u32) =
                        *((b.wrapping_add(4)) as *const u32);
                    *(b as *mut u32) = t0;
                    *(b.wrapping_add(4) as *mut u32) = t1;
                    live = *global::<i32>(G_FILL);
                }
            }
        }

        // Walk the table from the cursor; first live entry wins.
        let bound = *global::<i32>(G_FILL);
        let mut cur = *global::<i32>(G_CURSOR);
        let mut id: u32 = 0xFFFF_FFFF;
        while cur < bound {
            let e = *(table.wrapping_add((cur as u32).wrapping_mul(8))
                as *const u32);
            cur += 1;
            *global::<u32>(G_CURSOR) = cur as u32;
            let lo = e & 0xFFFF;
            let cell = obj.wrapping_add(lo.wrapping_mul(4)).wrapping_add(0x804);
            if *(cell as *const u32) != 0 {
                id = e;
                break;
            }
        }
        if id & 0xFFFF == 0xFFFF {
            *(out_id as *mut u32) = 0xFFFF_FFFF;
            0u8
        } else {
            let lo = id & 0xFFFF;
            let hi = id >> 16;
            let cell = obj.wrapping_add(lo.wrapping_mul(4)).wrapping_add(0x804);
            let base = *(cell as *const u32);
            let rec = base.wrapping_add(hi.wrapping_mul(0x20));
            let px =
                *((rec.wrapping_add(0x14)) as *const i16) as f32 * XY_SCALE;
            *(out_pos as *mut f32) = px;
            let py =
                *((rec.wrapping_add(0x16)) as *const i16) as f32 * XY_SCALE;
            *((out_pos.wrapping_add(4)) as *mut f32) = py;
            let pz = *((rec.wrapping_add(0x18)) as *const i16) as f32 * Z_SCALE;
            *((out_pos.wrapping_add(8)) as *mut f32) = pz;
            *(out_id as *mut u32) = id;
            1u8
        }
    }
});
