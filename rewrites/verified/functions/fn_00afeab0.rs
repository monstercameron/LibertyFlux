// original: 0x00afeab0 input-ui handle resolver
/// Resolve two handles through the game's word tables and emit a placement
/// record through one of two intercepted helpers.
///
/// `a2` points at a descriptor whose dword at +8 selects a row of the handle
/// table; `a3` picks an entry in that row. The low word of the entry selects a
/// struct base and the high word an offset into it. Depending on flag bytes in
/// both structs, the function either follows a two-struct fast path (with an
/// integer-coordinate distance check) or falls back to single-struct paths,
/// each ending in an 8-argument record call. `a1` is a float carried into the
/// record; `a4` is a mode flag; `a0` is forwarded untouched. No return value.
///
/// The original also stores one scratch word over its own incoming `a3` slot
/// (dead storage: the single call site passes by value and never reads it
/// back, verified by disassembly), which safe Rust cannot address, so the
/// contract does not compare the stack.
lf_checker_rt::export!(cdecl, rw_00afeab0(a0: u32, a1: f32, a2: u32, a3: u32, a4: u32) -> () {
    let full = m32(a2.wrapping_add(8));
    let row = g32(0x1178384u32.wrapping_add((full & 0xffff).wrapping_mul(4)));
    let entry = row.wrapping_add(a3.wrapping_mul(8));
    let h1 = m32(entry);
    let p1base = g32(0x1178284u32.wrapping_add((h1 & 0xffff).wrapping_mul(4)));
    if p1base == 0 {
        return;
    }
    let p1 = p1base.wrapping_add((h1 >> 16).wrapping_mul(32));
    if (m8(p1.wrapping_add(0x1e)) & 0x0f) != 2 {
        fn4_tail(a0, a1, a2, a4, entry, p1);
        return;
    }
    // Fast path: resolve the second struct from the first one's link words.
    let wj = m16(p1.wrapping_add(8)) as u32;
    let sx = mi16(p1.wrapping_add(0x12)) as i32;
    let mut row2 =
        g32(0x1178384u32.wrapping_add(wj.wrapping_mul(4))).wrapping_add((sx as u32).wrapping_mul(8));
    if m32(row2) == full {
        row2 = row2.wrapping_add(8);
    }
    let h2 = m32(row2);
    let p2base = g32(0x1178284u32.wrapping_add((h2 & 0xffff).wrapping_mul(4)));
    if p2base == 0 {
        fn4_tail(a0, a1, a2, a4, entry, p1);
        return;
    }
    let p2 = p2base.wrapping_add((h2 >> 16).wrapping_mul(32));
    if (m8(row2.wrapping_add(5)) & 0xc0) == 0 {
        fn4_tail(a0, a1, a2, a4, entry, p1);
        return;
    }
    let dl = (a4 & 0xff) as u8;
    if dl == 0 && (m8(p1.wrapping_add(0x1f)) & 0x20) == 0 {
        let r = lf_checker_rt::callee_cdecl!(1, u32, p1, row2);
        if (r & 0xff) == 0 {
            fn4_tail(a0, a1, a2, a4, entry, p1);
            return;
        }
    } else {
        // Integer-coordinate distance check between the two structs.
        let k1 = gf32(0xfe87a4);
        let v4 = (mi16(p1.wrapping_add(0x14)) as f32) * k1;
        let v3 = (mi16(p1.wrapping_add(0x16)) as f32) * k1;
        let v1 = (mi16(p2.wrapping_add(0x14)) as f32) * k1;
        let v0 = (mi16(p2.wrapping_add(0x16)) as f32) * k1;
        let d4 = (v4 - v1).abs();
        let d3 = (v3 - v0).abs();
        if d4 > d3 {
            if dl != 0 {
                return;
            }
            let r = lf_checker_rt::callee_cdecl!(1, u32, p1, row2);
            if (r & 0xff) == 0 {
                fn4_tail(a0, a1, a2, a4, entry, p1);
                return;
            }
        }
    }
    lf_checker_rt::callee_cdecl!(
        2,
        u32,
        a0,
        a1.to_bits(),
        p1,
        p2,
        row2,
        0u32,
        0x3f800000u32,
        1u32
    );
});

// Single-struct fallback paths shared by all early exits above.
#[inline(never)]
fn fn4_tail(a0: u32, a1: f32, a2: u32, a4: u32, entry: u32, p1: u32) {
    if (m8(entry.wrapping_add(5)) & 0xc0) != 0 {
        let r = lf_checker_rt::callee_cdecl!(1, u32, a2, entry);
        if (r & 0xff) != 0 {
            let e = ((m8(entry.wrapping_add(6)) >> 4) & 7) as u32;
            let mut t = e as f32;
            t = t * gf32(0xe83170);
            t = t - gf32(0xfe87b4);
            lf_checker_rt::callee_cdecl!(2, u32, a0, a1.to_bits(), a2, p1, entry, 0u32, t.to_bits(), 1u32);
            return;
        }
    }
    if ((a4 & 0xff) as u8) != 0 {
        return;
    }
    // edx carries a0 (the forwarded pointer) on entry to this block and the
    // post-call reload restores the same pair, so both paths agree.
    if (m8(a2.wrapping_add(0x1c)) & 0xf0) == 0xa0 {
        lf_checker_rt::callee_cdecl!(
            2,
            u32,
            a0,
            a1.to_bits(),
            a2,
            p1,
            entry,
            1u32,
            0x3f800000u32,
            4u32
        );
    }
    if (m8(p1.wrapping_add(0x1c)) & 0xf0) != 0x50 {
        return;
    }
    let e = ((m8(entry.wrapping_add(6)) >> 4) & 7) as u32;
    let mut t = e as f32;
    t = t * gf32(0xe83170);
    t = t - gf32(0xfe87b4);
    lf_checker_rt::callee_cdecl!(2, u32, a0, a1.to_bits(), a2, p1, entry, 2u32, t.to_bits(), 4u32);
}

#[inline(always)]
fn g32(file_va: u32) -> u32 {
    // SAFETY: the worker maps the original image; these globals are listed in
    // the contract and read-only here.
    unsafe { *lf_checker_rt::global::<u32>(file_va) }
}

#[inline(always)]
fn gf32(file_va: u32) -> f32 {
    // SAFETY: same as above; these addresses hold float constants.
    unsafe { *lf_checker_rt::global::<f32>(file_va) }
}

#[inline(always)]
fn m32(addr: u32) -> u32 {
    // SAFETY: the contract maps every address the original reads.
    unsafe { *(addr as *const u32) }
}

#[inline(always)]
fn m16(addr: u32) -> u16 {
    // SAFETY: same as above.
    unsafe { *(addr as *const u16) }
}

#[inline(always)]
fn mi16(addr: u32) -> i16 {
    // SAFETY: same as above.
    unsafe { *(addr as *const i16) }
}

#[inline(always)]
fn m8(addr: u32) -> u8 {
    // SAFETY: same as above.
    unsafe { *(addr as *const u8) }
}
