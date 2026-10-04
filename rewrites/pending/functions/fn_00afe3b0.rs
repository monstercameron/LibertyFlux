// original: 0x00afe3b0 input-ui float predicate
/// Decide whether the point described by `a0` passes a two-stage float gate.
///
/// `a0` points at three floats (x, y, z); `a1` at two (x, y). The function
/// measures the squared xy distance between them, asks an intercepted
/// membership query about `a0`, and then applies one of two threshold ladders
/// built from the game's float constants and the `f2`/`f3` parameters.
/// Returns 1 when the point passes, 0 otherwise.
///
/// The original also stores two scratch words over its own incoming argument
/// slots (dead storage: both call sites pass by value and never read them
/// back, verified by disassembly), which safe Rust cannot address, so the
/// contract does not compare the stack.
lf_checker_rt::export!(cdecl, rw_00afe3b0(a0: u32, a1: u32, f2: f32, f3: f32) -> u8 {
    // SAFETY: the contract always passes heap pointers to 3+ / 2+ floats.
    let x2 = unsafe { *(a0 as *const f32) };
    let x1 = unsafe { *((a0 + 4) as *const f32) };
    let z0 = unsafe { *((a0 + 8) as *const f32) };
    let b0 = unsafe { *(a1 as *const f32) };
    let b1 = unsafe { *((a1 + 4) as *const f32) };

    let idx = g32(0x118d818);
    let this = g32(0x118d818u32.wrapping_add(idx.wrapping_mul(4))).wrapping_add(0x10);

    let dx = b0 - x2;
    let dy = b1 - x1;
    let dist2 = dx * dx + dy * dy;

    let r1 = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        x2.to_bits(),
        x1.to_bits(),
        z0.to_bits(),
        0x40a00000u32,
        0u32
    );
    let present = r1 != 0;
    let under = (g32(0x12e21f0) < g32(0x103ffe8)) as u8;

    if present {
        if under == 0 {
            return 0;
        }
    } else {
        let r2 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(0x128e310));
        if (r2 & 0xFF) == 0 {
            // Near path: scale f3 against the 200.0 constant, then compare
            // the squared distance against the scaled window and a 56.0 floor.
            let k = gf32(0x103ff88);
            let mut t = f3;
            if !(t > k) {
                t = k;
            }
            t = t / k;
            t = t * gf32(0x103ff90);
            t = t * f2;
            t = t * t;
            if dist2 > t {
                return 0;
            }
            let s = gf32(0x103ff64);
            let s2 = s * s;
            if !(s2 > dist2) {
                return 1;
            } else {
                return 0;
            }
        }
        if under == 0 {
            return 0;
        }
    }

    // Far path: three nested windows around (m*f2)^2, ((185-10)*f2)^2 and
    // ((185*0.75)*f2)^2, where m is the smaller of f3 and 200.0.
    let k = gf32(0x103ff88);
    let m = if k > f3 { k } else { f3 };
    let base = gf32(0x103ff5c);
    let c3 = m * f2;
    let c3sq = c3 * c3;
    let c2 = (base - gf32(0xfe8b08)) * f2;
    let c2sq = c2 * c2;
    let c1 = (base * gf32(0xfe888c)) * f2;
    let c1sq = c1 * c1;
    if dist2 > c3sq {
        return 0;
    }
    if c2sq > dist2 {
        return 0;
    }
    if c1sq > dist2 {
        return 0;
    }
    1
});

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
