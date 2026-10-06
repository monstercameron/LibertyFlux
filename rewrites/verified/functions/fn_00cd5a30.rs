// original: 0x00cd5a30 euphoria_blend_setup
/// Blend setup over an animation channel and its owner objects.
///
/// Validates the channel object, notifies two construction helpers, then
/// derives a direction triple: either from the channel's own rates dotted
/// against a basis (possibly negated), or, when the owner is live and the
/// blend selector matches, from a queried triple dotted the same way. After
/// a second gate byte it runs a fixed chain of member notifications, polls
/// two floats, scales a table rate by them, runs a seven-argument query
/// whose out-cells drive a long optional region (a nineteen-argument call
/// plus a second seven-argument call), and then walks a chain of mode and
/// state gates with small calls before finishing through a common tail that
/// notifies two effector slots and a teardown helper. Early exits return
/// the channel pointer, the selected object, or the value flowing in EAX.
///
/// Float operation order matches the original exactly (operands pinned). The
/// two dot-compare branches fire only on ordered greater (unordered keeps
/// the triple). Out-cells the original passes uninitialized are modelled as
/// zeroed buffers, matching the proof's defined stack fill.
export!(thiscall, rw_cd5a30(
    this: *mut u8,
    a8: *mut u8,
    aC: *mut u8,
    a10: *mut u8,
    a14: u32,
    _a18: u32,
    a1C: *mut u8,
) -> u32 {
    unsafe {
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let r32 = |p: *const u8, off: usize| *(p.add(off) as *const u32);
        let rf = |p: *const u8, off: usize| f32::from_bits(r32(p, off));
        const MODE_MASK: u32 = 0x3C0;
        const MODE_LIVE: u32 = 0xC0;
        const TABLE: u32 = 0x1295CD8;

        if r32(this, 0x14) == 0 {
            return this as u32;
        }
        if *this.add(0x21) == 0 {
            return this as u32;
        }
        let s5 = r32(this, 0x24);
        if s5 == 0 {
            return 0;
        }
        if *((s5 + 0x44) as *const u16) != 1 {
            return s5;
        }
        if r32(s5 as *const u8, 0x40) == 0 {
            return s5;
        }
        // Frame buffers. The original passes several stack addresses to
        // callees; only the values matter, compared through out-cells,
        // snapshots and downstream reads.
        let mut obj = [0u32; 8];
        let objp = obj.as_mut_ptr() as u32;
        let mut cell224 = [0u32; 8];
        // The queried cell is pre-filled with the channel head word below.
        cell224[0] = r32(a10, 0);
        let c224p = cell224.as_mut_ptr() as u32;
        callee_thiscall!(1, u32, objp);
        callee_thiscall!(2, u32, objp, a10 as u32);
        let mut x2 = rf(a10, 0x20);
        let mut x3 = rf(a10, 0x24);
        let mut x4 = rf(a10, 0x28);
        // EAX value flowing to the join-point exits below.
        let flow: u32;
        if r32(aC, 0x28) & MODE_MASK != MODE_LIVE {
            let b = r32(a8, 0x20) as *const u8;
            let t0 = mul(rf(b, 0x10), x2);
            let mut t1 = mul(rf(b, 0x14), x3);
            t1 = add(t1, t0);
            let t0 = mul(rf(b, 0x18), x4);
            let d = add(t1, t0);
            // Negate only on ordered greater (unordered keeps the triple).
            // The original flips the sign bit; a multiply would differ on
            // NaN payloads.
            if 0.0f32 > d {
                x2 = -core::hint::black_box(x2);
                x3 = -core::hint::black_box(x3);
                x4 = -core::hint::black_box(x4);
            }
            // The basis load above leaves the basis in EAX on this path.
            flow = b as u32;
        } else {
            if a1C.is_null() {
                flow = 0;
            } else if a1C as u32 != *global::<u32>(0x171C93C) {
                flow = a1C as u32;
            } else if *a1C.add(4) != 0x0C {
                flow = a1C as u32;
            } else {
                callee_thiscall!(3, u32, objp, r32(a1C, 0x84));
                callee_thiscall!(4, u32, objp, r32(a8, 0x20));
                x2 = f32::from_bits(obj[4]);
                x3 = f32::from_bits(obj[5]);
                x4 = f32::from_bits(obj[6]);
                let b = r32(a8, 0x20) as *const u8;
                let t0 = mul(rf(b, 0x10), x2);
                let mut t1 = mul(rf(b, 0x14), x3);
                t1 = add(t1, t0);
                let t0 = mul(rf(b, 0x18), x4);
                let d = add(t1, t0);
                // Reload only on ordered greater (unordered keeps the triple).
                if 0.0f32 > d {
                    x2 = rf(b, 0x10);
                    x3 = rf(b, 0x14);
                    x4 = rf(b, 0x18);
                }
                flow = b as u32;
            }
        }
        // Join point: exit if the spilled channel head word is zero.
        if cell224[0] == 0 {
            return flow;
        }
        if (a14 & 0xFF) as u8 != 0 {
            return flow;
        }
        let x = r32(a8, 0x2C4);
        let x2v = if x != 0 { r32(x as *const u8, 0x25C) } else { 0 };
        callee_thiscall!(5, u32, objp);
        if *this.add(0x5D) & 0x80 != 0 && x2v != 0 {
            let ans6 = callee_cdecl!(6, u32, r32(x2v as *const u8, 0x18));
            if r32(ans6 as *const u8, 0x0C) == 1 {
                callee_thiscall!(7, u32, objp, r32(x2v as *const u8, 0x18), 1, 0);
            } else {
                callee_thiscall!(7, u32, objp, 0, 1, 0);
            }
        } else {
            callee_thiscall!(7, u32, objp, 0, 1, 0);
        }
        callee_thiscall!(8, u32, objp, 1);
        // Past the reload, the base register holds the channel object.
        callee_thiscall!(9, u32, objp, r32(this, 0x2C));
        let ecx10 = r32(this, 0x14);
        let f10: f32 = callee_thiscall!(10, f32, ecx10);
        let idx = *(a8.add(0x2E) as *const i16) as i32;
        let tobj = *global::<u32>(TABLE).offset(idx as isize) as *const u8;
        let mut fc = mul(
            mul(rf(tobj, 0xF4), f10),
            f32::from_bits(*global::<u32>(0x105196C)),
        );
        let al11a = callee_thiscall!(11, u32, a8 as u32) as u8;
        if al11a != 0 {
            fc = 0.0;
        }
        // The second poll is stored and reloaded but its value is never used;
        // the query below takes the first polled float instead.
        let _f12: f32 = callee_thiscall!(12, f32, ecx10);
        let mut cell236 = [0x3F80_0000u32; 1];
        let c236p = cell236.as_mut_ptr() as u32;
        let _discard: f32 = callee_thiscall!(
            13, f32, objp, a8 as u32, r32(a8, 0x20), c224p, 1, fc.to_bits(), 0,
            c236p
        );
        let al14 = callee_thiscall!(14, u32, 0) as u8;
        if al14 == 0 {
            let al15 = callee_thiscall!(15, u32, a8 as u32) as u8;
            if al15 != 0
                && *global::<u32>(0x11D6FD4) == 2
                && *global::<u8>(0x12B61CE) != 0
            {
                // Curve region. The first difference below fills the call's
                // out-cell; the other two stores are never read back.
                let k = f32::from_bits(*global::<u32>(0xFE87E4));
                let t3 = mul(x2, k);
                let t1 = mul(x3, k);
                let t2 = mul(x4, k);
                let g0 = f32::from_bits(cell224[4]);
                let g1 = f32::from_bits(cell224[5]);
                let g2 = f32::from_bits(cell224[6]);
                let mut cell264 = [sub(g0, t3).to_bits()];
                let _u1 = sub(g1, t1);
                let _u2 = sub(g2, t2);
                callee_cdecl!(
                    16, u32, 0, a8 as u32, 0x16, 0x3F80_0000u32,
                    cell264.as_mut_ptr() as u32, 1, 0, 1, 0xBF80_0000u32, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0xFFFF_FFFFu32
                );
            }
        }
        let al11b = callee_thiscall!(11, u32, a8 as u32) as u8;
        // Out-word of the second seven-argument query; keeps its zero fill
        // when that query is skipped.
        let mut cell260 = [0u32; 1];
        if al11b == 0 {
            let g0 = f32::from_bits(cell224[4]);
            let g1 = f32::from_bits(cell224[5]);
            let g2 = f32::from_bits(cell224[6]);
            let mut cell132 = [g0.to_bits(), g1.to_bits(), g2.to_bits(), 0u32];
            // The first difference below fills the query's second out-cell;
            // the other two stores are never read back.
            cell260[0] = sub(g0, x2).to_bits();
            let _v1 = sub(g1, x3);
            let _v2 = sub(g2, x4);
            callee_thiscall!(
                17, u32, objp, a8 as u32, cell132.as_mut_ptr() as u32,
                cell260.as_mut_ptr() as u32, c224p, 1, 1, fc.to_bits()
            );
        }
        if cell224[0] == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        if r32(aC, 0x28) & MODE_MASK != MODE_LIVE {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        if callee_thiscall!(11, u32, aC as u32) as u8 == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        if r32(a8, 0x28) & MODE_MASK != MODE_LIVE {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        if callee_thiscall!(11, u32, aC as u32) as u8 == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let ecxA = r32(a8, 0x6C);
        let eaxA = r32(aC, 0x6C);
        if ecxA == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let ax = callee_thiscall!(18, u32, ecxA) & 0xFFFF;
        if ax == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let eaxB = r32(a8, 0x224).wrapping_add(0x2E0);
        if eaxB == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        if callee_thiscall!(19, u32, eaxB, 0x1B1, 0) as u8 == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let ans20 = callee_thiscall!(20, u32, eaxB, 0x1B1, 5);
        if ans20 == 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let ans21 = callee_thiscall!(21, u32, eaxA.wrapping_add(0x808), 0x1B1);
        if ans21 != 0 && *(ans21 as *const u8).add(0x1A) != 0 {
            return blend_tail(this, a8, aC, objp, c224p, fc.to_bits());
        }
        let ans22 = callee_thiscall!(22, u32, *global::<u32>(0x18B69B4));
        let eaxC = if ans22 != 0 {
            // First argument is the masked poll answer stashed above, not
            // the query out-cell (that cell is never read back).
            callee_thiscall!(23, u32, ans22, ax, ans20, 0, 1)
        } else {
            0
        };
        callee_thiscall!(24, u32, eaxA, eaxC, 1, 1);
        blend_tail(this, a8, aC, objp, c224p, fc.to_bits())
    }
});

/// Common tail of the blend setup: effector notifications and teardown.
///
/// Polls a final value, notifies two effector slots (the second only when
/// the owner is live), runs the teardown helper and returns its answer.
fn blend_tail(
    this: *mut u8, a8: *mut u8, aC: *mut u8, objp: u32, c224p: u32, fc: u32,
) -> u32 {
    unsafe {
        let r32 = |p: *const u8, off: usize| *(p.add(off) as *const u32);
        const MODE_MASK: u32 = 0x3C0;
        const MODE_LIVE: u32 = 0xC0;
        let edi2 = callee_thiscall!(25, u32, r32(this, 0x14));
        callee_thiscall!(26, u32, (a8 as u32).wrapping_add(0x3C0), edi2, c224p);
        if r32(aC, 0x28) & MODE_MASK == MODE_LIVE {
            callee_thiscall!(
                27, u32,
                (aC as u32).wrapping_add(0x3C0),
                edi2, c224p, fc
            );
        }
        callee_thiscall!(28, u32, objp)
    }
}
