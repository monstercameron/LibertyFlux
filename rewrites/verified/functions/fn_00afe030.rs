// original: 0x00afe030 input-ui entry scanner
/// Scan the entries of the registry at the global slot, testing each one
/// against a distance gate and notifying it through its function table.
///
/// The registry holds a base pointer, a flag array, a count and a stride.
/// Each entry whose flag byte has the top bit clear is measured: an
/// intercepted query fills a scratch triple, the squared distance to the
/// entry's anchor point selects one of two threshold ladders built from `a0`
/// and the game's float constants, and the matching ladder step invokes the
/// entry's notify slots. No return value.
lf_checker_rt::export!(cdecl, rw_00afe030(a0: f32) -> () {
    let g = g32(0x12e22a4);
    let mut count = m32(g.wrapping_add(8));
    if count == 0 {
        return;
    }
    loop {
        let flagbase = m32(g.wrapping_add(4));
        count = count.wrapping_sub(1);
        if (m8(flagbase.wrapping_add(count)) & 0x80) == 0 {
            let stride = m32(g.wrapping_add(12));
            let base = m32(g);
            let esi = base.wrapping_add(stride.wrapping_mul(count));
            if esi != 0 {
                fn2_entry(a0, esi);
            }
        }
        if count == 0 {
            return;
        }
    }
});

// Test one entry against the distance gate.
#[inline(never)]
fn fn2_entry(a0: f32, esi: u32) {
    let mut out = [0u32; 4];
    lf_checker_rt::callee_cdecl!(1, u32, out.as_mut_ptr() as u32);
    let pos = m32(esi.wrapping_add(0x20));
    // The pushed argument is still on the stack when these read, so the
    // triple sits one word lower than a post-cleanup reading suggests.
    let dx = f32::from_bits(out[0]) - mf32(pos.wrapping_add(0x30));
    let dy = f32::from_bits(out[1]) - mf32(pos.wrapping_add(0x34));
    let dz = f32::from_bits(out[2]) - mf32(pos.wrapping_add(0x38));
    let mut slot = dx * dx + dy * dy + dz * dz;
    // The gate query takes no register argument: the original's ecx at this
    // site is the previous callee's scratch residue (0 from the stub here,
    // indeterminate garbage from the real callee), so no rewrite can forward
    // it and the contract compares the call without registers.
    let r2 = lf_checker_rt::callee_cdecl!(2, u32,);
    if (r2 & 0xff) != 0 {
        let o = m32(esi.wrapping_add(0x6c));
        if o != 0 {
            let vt = m32(o);
            let target = m32(vt.wrapping_add(0x104));
            let f: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(target as usize) };
            slot = f(o);
        }
    }
    let r7 = lf_checker_rt::callee_thiscall!(7, u32, esi);
    let k1 = gf32(0xfe88e8);
    let x2 = if (r7 & 0xff) != 0 && m32(esi.wrapping_add(0x1304)) != 2 {
        k1
    } else {
        gf32(0xfe8a24)
    };
    let x3 = k1;
    let mut t0 = gf32(0x103f6bc);
    if !(t0 > x3) {
        t0 = x3;
    }
    let mut t1 = gf32(0x103ffc0) * a0;
    let x2v = x2 * t0;
    let mut s0 = gf32(0x103ffbc) * a0;
    t1 = t1 * x2v;
    s0 = s0 * x2v;
    s0 = s0 * s0;
    let vt = m32(esi);
    if slot > s0 {
        // Near step: notify slot 0x144, else slot 0x148.
        let f144: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(m32(vt.wrapping_add(0x144)) as usize) };
        if (f144(esi) & 0xff) == 0 {
            let f148: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(m32(vt.wrapping_add(0x148)) as usize) };
            f148(esi);
        }
    } else {
        // Far step: notify slot 0x144, then slot 0x14c when set.
        t1 = t1 * t1;
        if t1 > slot {
            let f144: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(m32(vt.wrapping_add(0x144)) as usize) };
            if (f144(esi) & 0xff) != 0 {
                let f14c: extern "thiscall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(m32(vt.wrapping_add(0x14c)) as usize) };
                f14c(esi);
            }
        }
    }
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
fn m8(addr: u32) -> u8 {
    // SAFETY: same as above.
    unsafe { *(addr as *const u8) }
}

#[inline(always)]
fn mf32(addr: u32) -> f32 {
    // SAFETY: same as above; these addresses hold floats.
    unsafe { *(addr as *const f32) }
}
