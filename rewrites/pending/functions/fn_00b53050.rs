// original: 0x00B53050 euphoria_entry_resolve
/// Scan the 32-entry flag table at `this+0x10`, starting from index
/// `this[0xC] mod 32` and walking down with wraparound, for up to 31 entries.
///
/// For the first entry whose own and predecessor flags are both set, ask
/// callee 1 to resolve the entry. On success copy the two resolved float
/// quads to `out0`/`out1`, derive a normalized direction from the object
/// rows, report it through callee 4 and return 1. Entries that fail to
/// resolve run an anchor-distance check (callees 2 and 3) instead.
/// Returns 0 when no entry resolves.
///
/// Cond: `this` points to a readable object, `out0`/`out1` to writable
/// float quads. Only the low result byte is significant.
export!(thiscall, rw_b01_f2(this: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        // Shared resolution triple (start-up-initialized floats, scripted
        // per trial) and the anchor-distance threshold (1.0, read-only).
        let g0 = *global::<f32>(0x1B4B320);
        let g1 = *global::<f32>(0x1B4B324);
        let g2 = *global::<f32>(0x1B4B328);
        let thresh = *global::<f32>(0xFE88E8);

        // Signed `mod 32` in the exact shape the original's bit trick
        // computes (Rust `%` is the same truncated remainder).
        let r = (*(this.wrapping_add(0xC) as *const i32)) % 32;
        let mut idx = r;
        let mut prev = if r - 1 < 0 { r - 1 + 32 } else { r - 1 };

        let mut iter = 0;
        loop {
            let flag_a = *(this.wrapping_add(idx as u32).wrapping_add(0x10) as *const u8);
            let flag_b = *(this.wrapping_add(prev as u32).wrapping_add(0x10) as *const u8);
            if flag_a != 0 && flag_b != 0 {
                let row = this
                    .wrapping_add(0x30)
                    .wrapping_add((idx as u32).wrapping_mul(16));
                let entry = this.wrapping_add((prev as u32).wrapping_add(3).wrapping_mul(16));
                // Resolution workspace. Words the original never stores stay
                // zero, matching the checker's defined stack fill.
                let mut st = [0u32; 21];
                st[4] = g0.to_bits();
                st[5] = g1.to_bits();
                st[6] = g2.to_bits();
                st[8] = g0.to_bits();
                st[9] = g1.to_bits();
                st[10] = g2.to_bits();
                st[12] = g0.to_bits();
                st[13] = g1.to_bits();
                st[14] = g2.to_bits();
                st[19] = 0xFFFF;
                let ok: u32 =
                    callee_cdecl!(1, u32, row, entry, 0, st.as_mut_ptr() as u32, 0xE, 1, 4);
                if ok != 0 {
                    // Callee 1 filled words 4..12 with the resolved quads.
                    let o0 = out0 as *mut u32;
                    let o1 = out1 as *mut u32;
                    *o0.offset(0) = st[4];
                    *o0.offset(1) = st[5];
                    *o0.offset(2) = st[6];
                    *o0.offset(3) = st[7];
                    *o1.offset(0) = st[8];
                    *o1.offset(1) = st[9];
                    *o1.offset(2) = st[10];
                    *o1.offset(3) = st[11];
                    // Row difference and its length. The add order mirrors
                    // the original's instruction order bit-for-bit.
                    let an8 = (((prev + 3) * 2) as u32).wrapping_mul(8);
                    let cn8 = (((idx + 3) * 2) as u32).wrapping_mul(8);
                    let ab = this.wrapping_add(an8);
                    let cb = this.wrapping_add(cn8);
                    let v2 = *(ab as *const f32) - *(cb as *const f32);
                    let v3 =
                        *(ab.wrapping_add(4) as *const f32) - *(cb.wrapping_add(4) as *const f32);
                    let v4 =
                        *(ab.wrapping_add(8) as *const f32) - *(cb.wrapping_add(8) as *const f32);
                    let len2 = v3 * v3 + v2 * v2 + v4 * v4;
                    // The original divides only when the length is not
                    // +0.0 (its flag test skips exactly then, NaN included
                    // on the divide side); `!=` reproduces that shape.
                    let mut inv = 0.0f32;
                    if len2 != 0.0 {
                        inv = thresh / len2.sqrt();
                    }
                    let dir = [v2 * inv, v3 * inv, v4 * inv];
                    let _: u32 = callee_thiscall!(
                        4,
                        u32,
                        relocated(0x13B0EB0),
                        this,
                        out0,
                        out1,
                        dir.as_ptr() as u32
                    );
                    return 1;
                }
                // Unresolved: squared distance from the row point to the
                // anchor triple; report a unit contact when within threshold.
                let anchor: u32 = callee_thiscall!(2, u32, relocated(0x103E498));
                let px = *(row as *const f32) - *(anchor.wrapping_add(0x40) as *const f32);
                let py =
                    *(row.wrapping_add(4) as *const f32) - *(anchor.wrapping_add(0x44) as *const f32);
                let pz =
                    *(row.wrapping_add(8) as *const f32) - *(anchor.wrapping_add(0x48) as *const f32);
                let d2 = py * py + px * px + pz * pz;
                if thresh >= d2 {
                    let _: u32 =
                        callee_thiscall!(3, u32, relocated(0x1723BB0), 0x3F800000);
                }
            }
            iter += 1;
            if iter >= 31 {
                break;
            }
            idx = prev;
            prev = if prev - 1 < 0 { prev - 1 + 32 } else { prev - 1 };
        }
        0
    }
});
